use std::cell::RefMut;

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};
use arrayref::array_ref;
use mpl_token_metadata::instruction::freeze_delegated_account;
use mpl_token_metadata::state::DataV2;
use mpl_token_metadata::utils::create_or_allocate_account_raw;
use mpl_token_metadata::{
    instruction::{
        create_master_edition_v3, create_metadata_accounts_v3, update_metadata_accounts_v2,
    },
    state::{MAX_NAME_LENGTH, MAX_URI_LENGTH},
};
use solana_program::{
    clock::Clock,
    program::{invoke, invoke_signed},
    serialize_utils::{read_pubkey, read_u16},
    system_instruction, sysvar,
    sysvar::{instructions::get_instruction_relative, SysvarId},
};
use spl_token::instruction::{approve, initialize_mint, mint_to};

use crate::constants::{COMPUTE_BUDGET, FREEZE_FEATURE_INDEX};
use crate::MintPhase;
use crate::{
    constants::{
        A_TOKEN, BOT_FEE, COLLECTIONS_FEATURE_INDEX, CONFIG_ARRAY_START, CONFIG_LINE_SIZE, PREFIX,
    },
    utils::*,
    BuyerInfoAccount, BuyerMerkleAllowlistProofData, CandyError, CandyMachine, ConfigLine,
    FreezePda, SplTokenAllowlistMode, BUYER_INFO_ACCOUNT_PREFIX, BUYER_INFO_ACCOUNT_SPACE,
};

/// Mint a new NFT pseudo-randomly from the config array.
#[derive(Accounts)]
#[instruction(creator_bump: u8)]
pub struct MintNFT<'info> {
    #[account(
        mut,
        has_one = treasury_wallet,
        has_one = creator_authority,
    )]
    candy_machine: Box<Account<'info, CandyMachine>>,
    /// CHECK: account constraints checked in account trait
    #[account(
        seeds=[
            PREFIX.as_bytes(),
            candy_machine.key().as_ref()
        ],
        bump = creator_bump
    )]
    candy_machine_creator: UncheckedAccount<'info>,
    buyer: Signer<'info>,
    /// CHECK: treasury_wallet can be any account and is not written to or read
    #[account(mut)]
    treasury_wallet: UncheckedAccount<'info>,
    // With the following accounts we aren't using anchor macros because they are CPI'd
    // through to token-metadata which will do all the validations we need on them.
    /// CHECK: account checked in CPI
    #[account(mut)]
    metadata: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    mint: Signer<'info>,
    /// CHECK: account checked in CPI
    creator_authority: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    master_edition: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
    /// CHECK: checked in program.
    recent_slothashes: UncheckedAccount<'info>,
    /// CHECK: account constraints checked in account trait
    #[account(address = sysvar::instructions::id())]
    instruction_sysvar_account: UncheckedAccount<'info>,
    /// CHECK: Validated in the instruction handler.
    bot_signer_authority: UncheckedAccount<'info>,
    /// CHECK: This account is validated in the instruction handler.
    #[account(
        mut,
        seeds = [
            BUYER_INFO_ACCOUNT_PREFIX.as_bytes(),
            candy_machine.key().as_ref(),
            buyer.key().as_ref()
        ],
        bump
    )]
    buyer_info_account: UncheckedAccount<'info>,
    /// CHECK: This account is validated in the instruction handler.
    #[account(mut)]
    buyer_token_account: UncheckedAccount<'info>,
    ata_program: Program<'info, AssociatedToken>,
    // Some additional remaining_accounts may also be included. See the enum
    // below for details.
}

// Note: If these accounts are added, they need to be added in the order they
// are listed in the enum.
enum RemainingAccounts {
    // Only needed if candy machine has spl_token_allowlist_settings.
    SplTokenAllowlistTokenAccount,
    // Only needed if candy machine has spl_token_allowlist_settings and mode is BurnEveryTime.
    SplTokenAllowlistTokenMint,
    // Only needed if candy machine has a treasury mint (uses an SPL token).
    TreasuryTokenAccount,
    // Only needed if freeze feature is active.
    BuyerNftMintTokenAccount,
    // Only needed if freeze feature is active.
    FreezePda,
    // Only needed if spl token mint is enabled.
    FreezeAta,
}

pub fn handle_mint_nft<'info>(
    ctx: Context<'_, '_, '_, 'info, MintNFT<'info>>,
    creator_bump: u8,
    buyer_info_account_bump: u8,
    buyer_merkle_allowlist_proof_data: Option<BuyerMerkleAllowlistProofData>,
    // Sole purpose of passing this in is to make this ix easier to parse.
    expected_price: u64,
) -> Result<()> {
    let candy_pubkey = ctx.accounts.candy_machine.key();
    let candy_machine = &mut ctx.accounts.candy_machine;
    let candy_machine_creator = &ctx.accounts.candy_machine_creator;
    let treasury_wallet = ctx.accounts.treasury_wallet.to_account_info();
    let buyer = &ctx.accounts.buyer;
    let token_program = &ctx.accounts.token_program;
    let clock = Clock::get()?;
    let recent_slothashes = &ctx.accounts.recent_slothashes;
    let instruction_sysvar_account = &ctx.accounts.instruction_sysvar_account;
    let instruction_sysvar_account_info = instruction_sysvar_account.to_account_info();
    let instruction_sysvar = instruction_sysvar_account_info.data.borrow();
    let current_ix = get_instruction_relative(0, &instruction_sysvar_account_info).unwrap();
    // We must ensure the metadata cannot be passed in with data in it, this must remain the first check before any bot taxes
    if !ctx.accounts.metadata.data_is_empty() {
        return err!(CandyError::MetadataAccountMustBeEmpty);
    }

    let bot_signer_authority = &ctx.accounts.bot_signer_authority;
    let is_bot_signer_authority_valid =
        assert_valid_bot_signer_authority(&bot_signer_authority.key());

    if let Err(_e) = is_bot_signer_authority_valid {
        punish_bots(
            CandyError::InvalidBotSignerAuthority,
            buyer.to_account_info(),
            ctx.accounts.candy_machine.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            BOT_FEE,
        )?;
        return Ok(());
    }

    if candy_machine.data.bot_protection_enabled && !bot_signer_authority.is_signer {
        punish_bots(
            CandyError::InvalidBotSignerAuthority,
            buyer.to_account_info(),
            ctx.accounts.candy_machine.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            BOT_FEE,
        )?;
        return Ok(());
    }

    if get_expected_remaining_accounts_count(candy_machine) < ctx.remaining_accounts.len() {
        punish_bots(
            CandyError::IncorrectRemainingAccountsLen,
            buyer.to_account_info(),
            ctx.accounts.candy_machine.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            BOT_FEE,
        )?;
        return Ok(());
    }

    if candy_machine.items_redeemed >= candy_machine.data.items_available {
        return err!(CandyError::CandyMachineEmpty);
    }

    if !cmp_pubkeys(&recent_slothashes.key(), &SlotHashes::id()) {
        return err!(CandyError::IncorrectSlotHashesPubkey);
    }

    if !cmp_pubkeys(&current_ix.program_id, &crate::id()) {
        punish_bots(
            CandyError::SuspiciousTransaction,
            buyer.to_account_info(),
            ctx.accounts.candy_machine.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            BOT_FEE,
        )?;
        return Ok(());
    }
    let next_ix = get_instruction_relative(1, &instruction_sysvar_account_info);
    match next_ix {
        Ok(ix) => {
            let discriminator = &ix.data[0..8];
            let after_collection_ix = get_instruction_relative(2, &instruction_sysvar_account_info);

            if !cmp_pubkeys(&ix.program_id, &crate::id())
                || discriminator != [103, 17, 200, 25, 118, 95, 125, 61]
                || after_collection_ix.is_ok()
            {
                // We fail here. Its much cheaper to fail here than to allow a malicious user to add an ix at the end and then fail.
                msg!("Failing and halting here due to an extra unauthorized instruction from program ID {}.", ix.program_id.to_string());
                return err!(CandyError::SuspiciousTransaction);
            }
        }
        Err(_) => {
            if is_feature_active(&candy_machine.data.uuid, COLLECTIONS_FEATURE_INDEX) {
                punish_bots(
                    CandyError::MissingSetCollectionDuringMint,
                    buyer.to_account_info(),
                    ctx.accounts.candy_machine.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    BOT_FEE,
                )?;
                return Ok(());
            }
        }
    }
    let mut idx = 0;
    let num_instructions =
        read_u16(&mut idx, &instruction_sysvar).map_err(|_| ProgramError::InvalidAccountData)?;

    for index in 0..num_instructions {
        let mut current = 2 + (index * 2) as usize;
        let start = read_u16(&mut current, &instruction_sysvar).unwrap();

        current = start as usize;
        let num_accounts = read_u16(&mut current, &instruction_sysvar).unwrap();
        current += (num_accounts as usize) * (1 + 32);
        let program_id = read_pubkey(&mut current, &instruction_sysvar).unwrap();

        if !cmp_pubkeys(&program_id, &crate::id())
            && !cmp_pubkeys(&program_id, &spl_token::id())
            && !cmp_pubkeys(&program_id, &solana_program::system_program::ID)
            && !cmp_pubkeys(&program_id, &A_TOKEN)
            && !cmp_pubkeys(&program_id, &COMPUTE_BUDGET)
        {
            msg!("Transaction had ix with program id {}.", program_id);
            punish_bots(
                CandyError::SuspiciousTransaction,
                buyer.to_account_info(),
                ctx.accounts.candy_machine.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                BOT_FEE,
            )?;
            return Ok(());
        }
    }

    let mint_phase = CandyMachine::get_mint_phase(candy_machine, clock.unix_timestamp);
    let is_mint_phase_valid = validate_mint_phase(
        buyer,
        &mint_phase,
        candy_machine,
        &buyer_merkle_allowlist_proof_data,
    );

    if let Err(candy_error) = is_mint_phase_valid {
        punish_bots(
            candy_error,
            buyer.to_account_info(),
            ctx.accounts.candy_machine.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            BOT_FEE,
        )?;
        return Ok(());
    }

    let price = CandyMachine::get_mint_price(candy_machine, &mint_phase);

    if price != expected_price {
        msg!(
            "Invalid mint price for mint_phase {:?}: actual mint price = {}, expected price = {}.",
            mint_phase,
            price,
            expected_price
        );
        return Err(CandyError::InvalidMintPrice.into());
    }

    let buyer_info_account = &ctx.accounts.buyer_info_account;
    let limit_per_address = candy_machine.data.limit_per_address;

    let provided_merkle_allowlist_proof = buyer_merkle_allowlist_proof_data.is_some();

    // Only create the BuyerInfoAccount if the edition has a limit_per_address
    // OR if the buyer provided an allowlist proof.
    let should_create_buyer_info_account = limit_per_address > 0 || provided_merkle_allowlist_proof;
    if should_create_buyer_info_account && buyer_info_account.data_is_empty() {
        let signer_seeds = [
            BUYER_INFO_ACCOUNT_PREFIX.as_bytes(),
            &candy_machine.key().to_bytes(),
            &buyer.key().to_bytes(),
            &[buyer_info_account_bump],
        ];

        create_or_allocate_account_raw(
            *ctx.program_id,
            buyer_info_account,
            &ctx.accounts.system_program,
            buyer,
            BUYER_INFO_ACCOUNT_SPACE,
            &signer_seeds,
        )?;

        write_anchor_account_discriminator::<BuyerInfoAccount>(buyer_info_account)?;
    }

    // Allowlist checks only apply during allowlist mint phase.
    let is_allowlist_phase = mint_phase == MintPhase::Allowlist;

    let is_buyer_omni_minter = is_omni_minter(buyer, candy_machine);

    // Only check the Merkle allowlist proof if the allowlist proof data is provided.
    if let (true, false, Some(proof_data)) = (
        is_allowlist_phase,
        is_buyer_omni_minter,
        buyer_merkle_allowlist_proof_data,
    ) {
        let amount = proof_data.amount;
        let proof = proof_data.proof;
        let root_index_for_proof = proof_data.root_index_for_proof as usize;

        let roots_list = &candy_machine.data.merkle_allowlist_root_list;
        if roots_list.is_empty() {
            msg!("Invalid allowlist proof provided, the current roots list is empty.");
            return err!(CandyError::InvalidAllowlistProof);
        } else if root_index_for_proof >= roots_list.len() {
            msg!(
                "Invalid root_index_for_proof provided, received: {}, roots_list length = {}.",
                root_index_for_proof,
                roots_list.len()
            );
            return err!(CandyError::InvalidAllowlistProof);
        }

        let leaf = solana_program::keccak::hashv(&[
            &[0x00],
            &buyer.key().to_bytes(),
            &candy_machine.key().to_bytes(),
            &amount.to_le_bytes(),
        ]);

        let root: [u8; 32] = roots_list[root_index_for_proof];

        let is_proof_valid = verify_merkle_proof(&proof, root, leaf.0);
        if !is_proof_valid {
            msg!(
                "Invalid proof provided for root_index_for_proof: {}.",
                root_index_for_proof
            );
            return err!(CandyError::InvalidAllowlistProof);
        }

        let buyer_info_account: Account<BuyerInfoAccount> = Account::try_from(buyer_info_account)?;
        require!(
            buyer_info_account.number_bought_merkle_allowlist_phase < amount,
            CandyError::AllowlistMintsAlreadyUsed
        );

        msg!(
            "Valid merkle allowlist proof submitted by {:?} with root index {}.",
            buyer.key(),
            root_index_for_proof
        );
    }

    if let (true, false, Some(spl_token_allowlist_settings)) = (
        is_allowlist_phase,
        is_buyer_omni_minter,
        &candy_machine.data.spl_token_allowlist_settings,
    ) {
        let spl_token_allowlist_token_account = get_remaining_account(
            candy_machine,
            ctx.remaining_accounts,
            RemainingAccounts::SplTokenAllowlistTokenAccount,
        );

        let buyer_allowlist_token_account = assert_is_token_account(
            &spl_token_allowlist_token_account,
            &buyer.key(),
            &spl_token_allowlist_settings.mint,
        );

        if buyer_allowlist_token_account.is_err() {
            punish_bots(
                CandyError::NoSplAllowlistToken,
                buyer.to_account_info(),
                ctx.accounts.candy_machine.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                BOT_FEE,
            )?;
            return Ok(());
        }
        // Unwrap to keep code less indented. Err is checked above.
        let buyer_allowlist_token_account = buyer_allowlist_token_account.unwrap();

        if buyer_allowlist_token_account.amount == 0 {
            punish_bots(
                CandyError::NoSplAllowlistToken,
                buyer.to_account_info(),
                ctx.accounts.candy_machine.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                BOT_FEE,
            )?;
            return Ok(());
        }

        if buyer_allowlist_token_account.amount > 0
            && spl_token_allowlist_settings.mode == SplTokenAllowlistMode::BurnEveryTime
        {
            let allowlist_token_mint = get_remaining_account(
                candy_machine,
                ctx.remaining_accounts,
                RemainingAccounts::SplTokenAllowlistTokenMint,
            );

            let key_check = assert_keys_equal(
                &allowlist_token_mint.key(),
                &spl_token_allowlist_settings.mint,
            );

            if key_check.is_err() {
                punish_bots(
                    CandyError::MintMismatch,
                    buyer.to_account_info(),
                    ctx.accounts.candy_machine.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    BOT_FEE,
                )?;
                return Ok(());
            }

            spl_token_burn(TokenBurnParams {
                mint: allowlist_token_mint.clone(),
                source: spl_token_allowlist_token_account.clone(),
                amount: 1,
                authority: buyer.to_account_info(),
                authority_signer_seeds: None,
                token_program: token_program.to_account_info(),
            })?;
        }
    }

    let (wallet_to_use, freeze_pda): (AccountInfo, Option<Account<FreezePda>>) =
        if is_feature_active(&candy_machine.data.uuid, FREEZE_FEATURE_INDEX) {
            if let Some(mint) = candy_machine.treasury_mint {
                let freeze_pda_info = get_remaining_account(
                    candy_machine,
                    ctx.remaining_accounts,
                    RemainingAccounts::FreezePda,
                );
                let freeze_ata = get_remaining_account(
                    candy_machine,
                    ctx.remaining_accounts,
                    RemainingAccounts::FreezeAta,
                );
                assert_is_ata(&freeze_ata, freeze_pda_info.key, &mint)?;
                let freeze_pda: Account<FreezePda> = Account::try_from(&freeze_pda_info)?;
                if freeze_pda.thaw_eligible(clock.unix_timestamp, candy_machine) {
                    (treasury_wallet, None)
                } else {
                    let freeze_ata = freeze_ata.to_account_info();
                    (freeze_ata, Some(freeze_pda))
                }
            } else {
                let freeze_pda_info = get_remaining_account(
                    candy_machine,
                    ctx.remaining_accounts,
                    RemainingAccounts::FreezePda,
                );
                let freeze_pda: Account<FreezePda> = Account::try_from(&freeze_pda_info)?;
                if freeze_pda.thaw_eligible(clock.unix_timestamp, candy_machine) {
                    (treasury_wallet, None)
                } else {
                    (freeze_pda_info, Some(freeze_pda))
                }
            }
        } else {
            (treasury_wallet, None)
        };

    if let Some(mint) = candy_machine.treasury_mint {
        let token_account_info = get_remaining_account(
            candy_machine,
            ctx.remaining_accounts,
            RemainingAccounts::TreasuryTokenAccount,
        );

        let token_account = assert_is_ata(&token_account_info, &buyer.key(), &mint)?;

        if token_account.amount < price {
            msg!(
                "The mint price is {} SPL tokens but the buyer only had {}.",
                price,
                token_account.amount
            );
            return err!(CandyError::NotEnoughTokens);
        }

        spl_token_transfer(TokenTransferParams {
            source: token_account_info.clone(),
            destination: wallet_to_use.to_account_info(),
            authority: buyer.to_account_info(),
            authority_signer_seeds: &[],
            token_program: token_program.to_account_info(),
            amount: price,
        })?;
    } else {
        if ctx.accounts.buyer.lamports() < price {
            msg!(
                "The mint price is {} SOL but the buyer only had {}.",
                price,
                ctx.accounts.buyer.lamports()
            );
            return err!(CandyError::NotEnoughSOL);
        }
        invoke(
            &system_instruction::transfer(&ctx.accounts.buyer.key(), &wallet_to_use.key(), price),
            &[
                ctx.accounts.buyer.to_account_info(),
                wallet_to_use.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }

    // *** BEGIN CREATE ATA ***
    let mint = &ctx.accounts.mint;
    let buyer_token_account = &ctx.accounts.buyer_token_account;
    let ata_program = &ctx.accounts.ata_program;
    let system_program = &ctx.accounts.system_program;
    let rent = &ctx.accounts.rent;
    let rent_struct = &Rent::from_account_info(&rent.to_account_info())?;
    let min_rent_lamports = rent_struct.minimum_balance(Mint::LEN).max(1);
    invoke_signed(
        &system_instruction::create_account(
            &buyer.key(),
            &mint.key(),
            min_rent_lamports,
            Mint::LEN as u64,
            &token_program.key(),
        ),
        &[
            buyer.to_account_info(),
            mint.to_account_info(),
            system_program.to_account_info(),
        ],
        &[],
    )?;

    invoke_signed(
        &initialize_mint(
            &token_program.key(),
            &mint.key(),
            &buyer.key(),
            Some(&buyer.key()),
            0,
        )
        .unwrap(),
        &[
            mint.to_account_info(),
            rent.to_account_info(),
            token_program.to_account_info(),
        ],
        &[],
    )?;

    make_ata(
        buyer_token_account.to_account_info(),
        buyer.to_account_info(),
        mint.to_account_info(),
        buyer.to_account_info(),
        ata_program.to_account_info(),
        token_program.to_account_info(),
        system_program.to_account_info(),
        rent.to_account_info(),
        &[],
    )?;

    invoke_signed(
        &mint_to(
            &token_program.key(),
            &mint.key(),
            &buyer_token_account.key(),
            &buyer.key(),
            &[],
            1,
        )
        .unwrap(),
        &[
            mint.to_account_info(),
            buyer_token_account.to_account_info(),
            buyer.to_account_info(),
            token_program.to_account_info(),
        ],
        &[],
    )?;
    // *** END CREATE ATA ***

    // Sequential minting is only allowed in the premint phase for now.
    let config_line_initial_index =
        if candy_machine.data.sequential_mint_order_enabled && mint_phase == MintPhase::Premint {
            candy_machine.items_redeemed as usize
        } else {
            let data = recent_slothashes.data.borrow();
            let most_recent = array_ref![data, 12, 8];

            let index = u64::from_le_bytes(*most_recent);
            index
                .checked_rem(candy_machine.data.items_available)
                .ok_or(CandyError::NumericalOverflowError)? as usize
        };

    let config_line = get_config_line(
        candy_machine,
        config_line_initial_index,
        candy_machine.items_redeemed,
    )?;

    candy_machine.items_redeemed = candy_machine
        .items_redeemed
        .checked_add(1)
        .ok_or(CandyError::NumericalOverflowError)?;

    let cm_key = candy_machine.key();
    let authority_seeds = [PREFIX.as_bytes(), cm_key.as_ref(), &[creator_bump]];

    // The original creators list only includes the 1 cm creator, which gets
    // removed after minting in the update instruction below.
    let creators_for_mint_ix: Vec<mpl_token_metadata::state::Creator> =
        vec![mpl_token_metadata::state::Creator {
            address: candy_machine_creator.key(),
            verified: true,
            share: 100,
        }];

    let metadata_infos = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.buyer.to_account_info(),
        ctx.accounts.buyer.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        candy_machine_creator.to_account_info(),
    ];

    let master_edition_infos = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.buyer.to_account_info(),
        ctx.accounts.buyer.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
        candy_machine_creator.to_account_info(),
    ];

    invoke_signed(
        &create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.buyer.key(),
            ctx.accounts.buyer.key(),
            candy_machine_creator.key(),
            config_line.name.clone(),
            candy_machine.data.symbol.clone(),
            config_line.uri.clone(),
            Some(creators_for_mint_ix),
            candy_machine.data.seller_fee_basis_points,
            true,
            candy_machine.data.is_mutable,
            None,
            None,
            None,
        ),
        metadata_infos.as_slice(),
        &[&authority_seeds],
    )?;
    invoke_signed(
        &create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
            candy_machine_creator.key(),
            ctx.accounts.buyer.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.buyer.key(),
            Some(candy_machine.data.max_supply),
        ),
        master_edition_infos.as_slice(),
        &[&authority_seeds],
    )?;

    let creators: Vec<mpl_token_metadata::state::Creator> = candy_machine
        .data
        .creators
        .iter()
        .map(|creator| mpl_token_metadata::state::Creator {
            address: creator.address,
            verified: false,
            share: creator.share,
        })
        .collect();

    let update_data = DataV2 {
        name: config_line.name,
        symbol: candy_machine.data.symbol.clone(),
        uri: config_line.uri,
        seller_fee_basis_points: candy_machine.data.seller_fee_basis_points,
        creators: Some(creators),
        collection: None,
        uses: None,
    };

    let is_mutable = if !candy_machine.data.is_mutable {
        Some(false)
    } else {
        None
    };

    // Now update NFT creators and update_authority.
    invoke_signed(
        &update_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            candy_machine_creator.key(),
            Some(candy_machine.creator_authority),
            Some(update_data),
            Some(true),
            is_mutable,
        ),
        &[
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            candy_machine_creator.to_account_info(),
        ],
        &[&authority_seeds],
    )?;

    if let Some(mut freeze_pda) = freeze_pda {
        msg!("About to freeze NFT.");
        let mint_pubkey = ctx.accounts.mint.key();
        let nft_token_account_info = get_remaining_account(
            candy_machine,
            ctx.remaining_accounts,
            RemainingAccounts::BuyerNftMintTokenAccount,
        );
        assert_is_ata(&nft_token_account_info, &buyer.key(), &mint_pubkey)?;
        let seeds: &[&[u8]] = &[FreezePda::PREFIX.as_bytes(), candy_pubkey.as_ref()];
        let (expected_freeze_key, freeze_bump) = Pubkey::find_program_address(seeds, &crate::id());
        assert_keys_equal(&expected_freeze_key, &freeze_pda.key())?;
        // redundant check
        freeze_pda.assert_from_candy(&candy_pubkey)?;

        freeze_pda.frozen_count += 1;

        if freeze_pda.freeze_fee > 0 {
            invoke(
                &system_instruction::transfer(
                    &ctx.accounts.buyer.key(),
                    &freeze_pda.key(),
                    freeze_pda.freeze_fee,
                ),
                &[
                    ctx.accounts.buyer.to_account_info(),
                    freeze_pda.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }

        if freeze_pda.mint_start.is_none() {
            freeze_pda.mint_start = Some(clock.unix_timestamp);
        }

        let freeze_seeds = [
            FreezePda::PREFIX.as_bytes(),
            candy_pubkey.as_ref(),
            &[freeze_bump],
        ];
        let mut freeze_ix = freeze_delegated_account(
            mpl_token_metadata::ID,
            freeze_pda.key(),
            nft_token_account_info.key(),
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
        );
        // token metadata ix is sorta bad, so this line fixes it to enable freeze without marking signer as mutable
        freeze_ix.accounts[0] = AccountMeta::new_readonly(freeze_pda.key(), true);

        invoke(
            &approve(
                &spl_token::ID,
                &nft_token_account_info.key(),
                &freeze_pda.key(),
                &buyer.key(),
                &[],
                1,
            )?,
            &[
                nft_token_account_info.to_account_info(),
                freeze_pda.to_account_info(),
                buyer.to_account_info(),
            ],
        )?;
        invoke_signed(
            &freeze_ix,
            &[
                freeze_pda.to_account_info(),
                nft_token_account_info.to_account_info(),
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.mint.to_account_info(),
            ],
            &[&freeze_seeds],
        )?;
        freeze_pda.exit(&crate::id())?;
    }

    if is_allowlist_phase && provided_merkle_allowlist_proof {
        let mut buyer_info_account: Account<BuyerInfoAccount> =
            Account::try_from(buyer_info_account)?;
        let number_bought_for_merkle_allowlist = buyer_info_account
            .number_bought_merkle_allowlist_phase
            .checked_add(1)
            .unwrap();
        buyer_info_account.number_bought_merkle_allowlist_phase =
            number_bought_for_merkle_allowlist;

        // This re-serializes the account to persist the changes.
        buyer_info_account.exit(&crate::id())?;
    }

    if mint_phase == MintPhase::Public && !is_buyer_omni_minter && limit_per_address > 0 {
        let mut buyer_info_account: Account<BuyerInfoAccount> =
            Account::try_from(buyer_info_account)?;
        require!(
            buyer_info_account.number_bought_public_phase < limit_per_address,
            CandyError::BuyLimitPerAddressExceeded
        );

        let number_bought = buyer_info_account
            .number_bought_public_phase
            .checked_add(1)
            .unwrap();
        buyer_info_account.number_bought_public_phase = number_bought;

        // This re-serializes the account to persist the changes.
        buyer_info_account.exit(&crate::id())?;
    }

    Ok(())
}

pub fn get_good_index(
    arr: &mut RefMut<&mut [u8]>,
    items_available: usize,
    index: usize,
    pos: bool,
) -> Result<(usize, bool)> {
    let mut index_to_use = index;
    let mut taken = 1;
    let mut found = false;
    let bit_mask_vec_start = CONFIG_ARRAY_START
        + 4
        + (items_available) * CONFIG_LINE_SIZE
        + 4
        + items_available
            .checked_div(8)
            .ok_or(CandyError::NumericalOverflowError)?
        + 4;

    while taken > 0 && index_to_use < items_available {
        let my_position_in_vec = bit_mask_vec_start
            + index_to_use
                .checked_div(8)
                .ok_or(CandyError::NumericalOverflowError)?;
        if arr[my_position_in_vec] == 255 {
            let eight_remainder = 8 - index_to_use
                .checked_rem(8)
                .ok_or(CandyError::NumericalOverflowError)?;
            let reversed = 8 - eight_remainder + 1;
            if (eight_remainder != 0 && pos) || (reversed != 0 && !pos) {
                if pos {
                    index_to_use += eight_remainder;
                } else {
                    if index_to_use < 8 {
                        break;
                    }
                    index_to_use -= reversed;
                }
            } else if pos {
                index_to_use += 8;
            } else {
                index_to_use -= 8;
            }
        } else {
            let position_from_right = 7 - index_to_use
                .checked_rem(8)
                .ok_or(CandyError::NumericalOverflowError)?;
            let mask = u8::pow(2, position_from_right as u32);

            taken = mask & arr[my_position_in_vec];

            match taken {
                x if x > 0 => {
                    if pos {
                        index_to_use += 1;
                    } else {
                        if index_to_use == 0 {
                            break;
                        }
                        index_to_use -= 1;
                    }
                }
                0 => {
                    found = true;
                    arr[my_position_in_vec] |= mask;
                }
                _ => (),
            }
        }
    }
    Ok((index_to_use, found))
}

pub fn get_config_line(
    a: &Account<'_, CandyMachine>,
    index: usize,
    mint_number: u64,
) -> Result<ConfigLine> {
    if let Some(hs) = &a.data.hidden_settings {
        return Ok(ConfigLine {
            name: hs.name.clone() + "#" + &(mint_number + 1).to_string(),
            uri: hs.uri.clone(),
        });
    }
    let a_info = a.to_account_info();

    let mut arr = a_info.data.borrow_mut();

    let (mut index_to_use, good) =
        get_good_index(&mut arr, a.data.items_available as usize, index, true)?;
    if !good {
        let (index_to_use_new, good_new) =
            get_good_index(&mut arr, a.data.items_available as usize, index, false)?;
        index_to_use = index_to_use_new;
        if !good_new {
            return err!(CandyError::CannotFindUsableConfigLine);
        }
    }

    if arr[CONFIG_ARRAY_START + 4 + index_to_use * (CONFIG_LINE_SIZE)] == 1 {
        return err!(CandyError::CannotFindUsableConfigLine);
    }

    let data_array = &mut arr[CONFIG_ARRAY_START + 4 + index_to_use * (CONFIG_LINE_SIZE)
        ..CONFIG_ARRAY_START + 4 + (index_to_use + 1) * (CONFIG_LINE_SIZE)];

    let mut name_vec = Vec::with_capacity(MAX_NAME_LENGTH);
    let mut uri_vec = Vec::with_capacity(MAX_URI_LENGTH);

    #[allow(clippy::needless_range_loop)]
    for i in 4..4 + MAX_NAME_LENGTH {
        if data_array[i] == 0 {
            break;
        }
        name_vec.push(data_array[i])
    }

    #[allow(clippy::needless_range_loop)]
    for i in 8 + MAX_NAME_LENGTH..8 + MAX_NAME_LENGTH + MAX_URI_LENGTH {
        if data_array[i] == 0 {
            break;
        }
        uri_vec.push(data_array[i])
    }
    let config_line: ConfigLine = ConfigLine {
        name: match String::from_utf8(name_vec) {
            Ok(val) => val,
            Err(_) => return err!(CandyError::InvalidString),
        },
        uri: match String::from_utf8(uri_vec) {
            Ok(val) => val,
            Err(_) => return err!(CandyError::InvalidString),
        },
    };

    msg!(
        "Minting config line at index {} with uri = '{}' and name = '{}'.",
        index_to_use,
        config_line.uri,
        config_line.name
    );

    Ok(config_line)
}

fn get_spl_token_allowlist_remaining_accounts_counter(candy: &CandyMachine) -> usize {
    let mut counter: usize = 0;
    if let Some(spl_token_allowlist_settings) = &candy.data.spl_token_allowlist_settings {
        counter += 1;
        if spl_token_allowlist_settings.mode == SplTokenAllowlistMode::BurnEveryTime {
            counter += 1;
        }
    }

    counter
}

fn get_treasury_remaining_accounts_counter(candy: &CandyMachine) -> usize {
    match candy.treasury_mint {
        Some(_) => 1,
        None => 0,
    }
}

fn get_remaining_account<'a>(
    candy: &CandyMachine,
    remaining_accounts: &[AccountInfo<'a>],
    account: RemainingAccounts,
) -> AccountInfo<'a> {
    let account_index: usize = match account {
        RemainingAccounts::SplTokenAllowlistTokenAccount => 0,
        RemainingAccounts::SplTokenAllowlistTokenMint => 1,
        RemainingAccounts::TreasuryTokenAccount => {
            get_spl_token_allowlist_remaining_accounts_counter(candy)
        }
        RemainingAccounts::FreezePda => {
            get_spl_token_allowlist_remaining_accounts_counter(candy)
                + get_treasury_remaining_accounts_counter(candy)
        }
        RemainingAccounts::BuyerNftMintTokenAccount => {
            get_spl_token_allowlist_remaining_accounts_counter(candy)
                + get_treasury_remaining_accounts_counter(candy)
                + 1
        }
        RemainingAccounts::FreezeAta => {
            get_spl_token_allowlist_remaining_accounts_counter(candy)
                + get_treasury_remaining_accounts_counter(candy)
                + 2
        }
    };

    remaining_accounts[account_index].clone()
}

pub fn get_expected_remaining_accounts_count(candy: &CandyMachine) -> usize {
    let mut expected_count = 0;

    if let Some(spl_token_allowlist_settings) = &candy.data.spl_token_allowlist_settings {
        expected_count += 1;
        if spl_token_allowlist_settings.mode == SplTokenAllowlistMode::BurnEveryTime {
            expected_count += 1;
        }
    }

    if candy.treasury_mint.is_some() {
        expected_count += 1;
    }

    if is_feature_active(&candy.data.uuid, FREEZE_FEATURE_INDEX) {
        expected_count += 2;
        if candy.treasury_mint.is_some() {
            expected_count += 1;
        }
    }
    expected_count
}
