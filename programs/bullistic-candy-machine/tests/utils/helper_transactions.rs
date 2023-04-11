use anchor_client::solana_sdk::{signature::Signer, system_program, sysvar};
use anchor_lang::*;
use anchor_spl::associated_token::AssociatedToken;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_instruction,
};
use solana_program_test::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, signature::Keypair, transaction::Transaction,
};

use bullistic_candy_machine::{
    constants::{CONFIG_ARRAY_START, CONFIG_LINE_SIZE},
    utils::cmp_pubkeys,
    BuyerMerkleAllowlistProofData, CandyMachine, CandyMachineData, ConfigLine,
    SplTokenAllowlistMode::BurnEveryTime,
};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    core::{helpers::update_blockhash, MasterEditionManager},
    utils::{
        candy_manager::{CollectionInfo, SplTokenAllowlistInfo, TokenInfo},
        helpers::{find_buyer_info_account_pda, get_bot_signer_keypair, make_config_lines},
        FreezeInfo, SolanaProgramTestResult,
    },
};

pub fn candy_machine_program_test() -> ProgramTest {
    let mut program = ProgramTest::new("bullistic_candy_machine", bullistic_candy_machine::id(), None);
    program.add_program("mpl_token_metadata", mpl_token_metadata::id(), None);
    program
}

pub async fn initialize_candy_machine(
    context: &mut ProgramTestContext,
    candy_account: &Keypair,
    payer: &Keypair,
    creator_authority: &Keypair,
    treasury_wallet: &Pubkey,
    candy_data: CandyMachineData,
    token_info: TokenInfo,
) -> SolanaProgramTestResult {
    let items_available = candy_data.items_available;
    let candy_account_size = if candy_data.hidden_settings.is_some() {
        CONFIG_ARRAY_START
    } else {
        CONFIG_ARRAY_START
            + 4
            + items_available as usize * CONFIG_LINE_SIZE
            + 8
            + 2 * (items_available as usize / 8 + 1)
    };

    let rent = context.banks_client.get_rent().await?;
    let lamports = rent.minimum_balance(candy_account_size);
    let create_ix = system_instruction::create_account(
        &payer.pubkey(),
        &candy_account.pubkey(),
        lamports,
        candy_account_size as u64,
        &bullistic_candy_machine::id(),
    );

    let mut accounts = bullistic_candy_machine::accounts::InitializeCandyMachine {
        candy_machine: candy_account.pubkey(),
        treasury_wallet: *treasury_wallet,
        bullistic_authority: payer.pubkey(),
        creator_authority: creator_authority.pubkey(),
        payer: payer.pubkey(),
        system_program: system_program::id(),
        rent: sysvar::rent::id(),
    }
    .to_account_metas(None);

    if token_info.set {
        accounts.push(AccountMeta::new_readonly(token_info.mint, false));
    }

    let data =
        bullistic_candy_machine::instruction::InitializeCandyMachine { data: candy_data }.data();

    let init_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };

    update_blockhash(context).await?;
    let tx = Transaction::new_signed_with_payer(
        &[create_ix, init_ix],
        Some(&payer.pubkey()),
        &[candy_account, payer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}

pub async fn update_candy_machine(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    bullistic_authority: &Keypair,
    data: CandyMachineData,
    treasury_wallet: &Pubkey,
    treasury_mint: Option<Pubkey>,
) -> SolanaProgramTestResult {
    let mut accounts = bullistic_candy_machine::accounts::UpdateCandyMachine {
        candy_machine: *candy_machine,
        bullistic_authority: bullistic_authority.pubkey(),
        treasury_wallet: *treasury_wallet,
    }
    .to_account_metas(None);
    if let Some(treasury_mint) = treasury_mint {
        accounts.push(AccountMeta::new_readonly(treasury_mint, false));
    }

    let data = bullistic_candy_machine::instruction::UpdateCandyMachine { data }.data();
    let update_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };

    update_blockhash(context).await?;
    let tx = Transaction::new_signed_with_payer(
        &[update_ix],
        Some(&bullistic_authority.pubkey()),
        &[bullistic_authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}

pub async fn append_merkle_allowlist_roots(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    bullistic_authority: &Keypair,
    roots_to_append: Vec<[u8; 32]>,
) -> SolanaProgramTestResult {
    let accounts = bullistic_candy_machine::accounts::AppendMerkleAllowlistRoots {
        bullistic_authority: bullistic_authority.pubkey(),
        candy_machine: *candy_machine,
    }
    .to_account_metas(None);

    let data =
        bullistic_candy_machine::instruction::AppendMerkleAllowlistRoots { roots_to_append }.data();

    let update_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };

    update_blockhash(context).await?;
    let tx = Transaction::new_signed_with_payer(
        &[update_ix],
        Some(&bullistic_authority.pubkey()),
        &[bullistic_authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}

pub async fn clear_merkle_allowlist_roots(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    bullistic_authority: &Keypair,
) -> SolanaProgramTestResult {
    let accounts = bullistic_candy_machine::accounts::ClearMerkleAllowlistRoots {
        bullistic_authority: bullistic_authority.pubkey(),
        candy_machine: *candy_machine,
    }
    .to_account_metas(None);

    let data = bullistic_candy_machine::instruction::ClearMerkleAllowlistRoots {}.data();

    let clear_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };

    update_blockhash(context).await?;
    let tx = Transaction::new_signed_with_payer(
        &[clear_ix],
        Some(&bullistic_authority.pubkey()),
        &[bullistic_authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}

pub async fn add_config_lines(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    bullistic_authority: &Keypair,
    index: u32,
    config_lines: Vec<ConfigLine>,
) -> SolanaProgramTestResult {
    let accounts = bullistic_candy_machine::accounts::AddConfigLines {
        candy_machine: *candy_machine,
        bullistic_authority: bullistic_authority.pubkey(),
    }
    .to_account_metas(None);

    let data = bullistic_candy_machine::instruction::AddConfigLines {
        index,
        config_lines,
    }
    .data();

    let add_config_line_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };

    update_blockhash(context).await?;

    let tx = Transaction::new_signed_with_payer(
        &[add_config_line_ix],
        Some(&bullistic_authority.pubkey()),
        &[bullistic_authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}

pub async fn add_all_config_lines(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    bullistic_authority: &Keypair,
) -> SolanaProgramTestResult {
    let candy_machine_account = context
        .banks_client
        .get_account(*candy_machine)
        .await
        .expect("account not found")
        .expect("account empty");

    let candy_machine_data: CandyMachine =
        CandyMachine::try_deserialize(&mut candy_machine_account.data.as_ref()).unwrap();
    let total_items = candy_machine_data.data.items_available;
    for i in 0..total_items / 10 {
        let index = (i * 10) as u32;
        let config_lines = make_config_lines(index, 10);
        add_config_lines(
            context,
            candy_machine,
            bullistic_authority,
            index,
            config_lines,
        )
        .await?;
    }
    let remainder = total_items % 10;
    if remainder > 0 {
        let extra = total_items % 10;
        let index = (total_items - extra) as u32;
        let config_lines = make_config_lines(index, remainder as u8);
        add_config_lines(
            context,
            candy_machine,
            bullistic_authority,
            index,
            config_lines,
        )
        .await?;
    }

    Ok(())
}

pub async fn set_collection(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    bullistic_authority: &Keypair,
    creator_authority: &Keypair,
    collection_info: &CollectionInfo,
) -> SolanaProgramTestResult {
    let accounts = bullistic_candy_machine::accounts::SetCollection {
        candy_machine: *candy_machine,
        bullistic_authority: bullistic_authority.pubkey(),
        creator_authority: creator_authority.pubkey(),
        collection_pda: collection_info.pda,
        payer: bullistic_authority.pubkey(),
        system_program: system_program::id(),
        rent: sysvar::rent::id(),
        metadata: collection_info.metadata,
        mint: collection_info.mint.pubkey(),
        edition: collection_info.master_edition,
        collection_authority_record: collection_info.authority_record,
        token_metadata_program: mpl_token_metadata::id(),
    }
    .to_account_metas(None);

    let data = bullistic_candy_machine::instruction::SetCollection {}.data();
    let set_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };

    update_blockhash(context).await?;
    let tx = Transaction::new_signed_with_payer(
        &[set_ix],
        Some(&bullistic_authority.pubkey()),
        &[bullistic_authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}

pub async fn remove_collection(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    bullistic_authority: &Keypair,
    collection_info: &CollectionInfo,
) -> SolanaProgramTestResult {
    let accounts = bullistic_candy_machine::accounts::RemoveCollection {
        candy_machine: *candy_machine,
        bullistic_authority: bullistic_authority.pubkey(),
        collection_pda: collection_info.pda,
        metadata: collection_info.metadata,
        mint: collection_info.mint.pubkey(),
        collection_authority_record: collection_info.authority_record,
        token_metadata_program: mpl_token_metadata::id(),
    }
    .to_account_metas(None);

    let data = bullistic_candy_machine::instruction::RemoveCollection {}.data();
    let set_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };

    update_blockhash(context).await?;
    let tx = Transaction::new_signed_with_payer(
        &[set_ix],
        Some(&bullistic_authority.pubkey()),
        &[bullistic_authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}

pub async fn set_freeze(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    bullistic_authority: &Keypair,
    freeze_info: &FreezeInfo,
    token_info: &TokenInfo,
) -> SolanaProgramTestResult {
    let mut accounts = bullistic_candy_machine::accounts::SetFreeze {
        candy_machine: *candy_machine,
        freeze_pda: freeze_info.pda,
        bullistic_authority: bullistic_authority.pubkey(),
        system_program: system_program::id(),
    }
    .to_account_metas(None);

    if token_info.set {
        accounts.push(AccountMeta::new(freeze_info.ata, false));
    }

    let data = bullistic_candy_machine::instruction::SetFreeze {
        freeze_time: freeze_info.freeze_time,
    }
    .data();
    let set_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };

    update_blockhash(context).await?;
    let tx = Transaction::new_signed_with_payer(
        &[set_ix],
        Some(&bullistic_authority.pubkey()),
        &[bullistic_authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}

pub async fn remove_freeze(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    bullistic_authority: &Keypair,
    freeze_info: &FreezeInfo,
) -> SolanaProgramTestResult {
    let accounts = bullistic_candy_machine::accounts::RemoveFreeze {
        candy_machine: *candy_machine,
        bullistic_authority: bullistic_authority.pubkey(),
        freeze_pda: freeze_info.pda,
    }
    .to_account_metas(None);

    let data = bullistic_candy_machine::instruction::RemoveFreeze {}.data();
    let set_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };
    update_blockhash(context).await?;
    let tx = Transaction::new_signed_with_payer(
        &[set_ix],
        Some(&bullistic_authority.pubkey()),
        &[bullistic_authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}

pub async fn thaw_nft(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    signer: &Keypair,
    freeze_info: &FreezeInfo,
    nft_info: &MasterEditionManager,
) -> SolanaProgramTestResult {
    let accounts = bullistic_candy_machine::accounts::ThawNFT {
        freeze_pda: freeze_info.pda,
        candy_machine: *candy_machine,
        token_account: nft_info.token_account,
        owner: nft_info.owner.pubkey(),
        mint: nft_info.mint.pubkey(),
        edition: nft_info.edition_pubkey,
        payer: signer.pubkey(),
        token_program: spl_token::ID,
        token_metadata_program: mpl_token_metadata::ID,
        system_program: system_program::id(),
    }
    .to_account_metas(None);

    let data = bullistic_candy_machine::instruction::ThawNft {}.data();
    let set_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };
    update_blockhash(context).await?;
    let tx = Transaction::new_signed_with_payer(
        &[set_ix],
        Some(&signer.pubkey()),
        &[signer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}

pub async fn unlock_funds(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    bullistic_authority: &Keypair,
    freeze_info: &FreezeInfo,
    token_info: &TokenInfo,
) -> SolanaProgramTestResult {
    let mut accounts = bullistic_candy_machine::accounts::UnlockFunds {
        freeze_pda: freeze_info.pda,
        candy_machine: *candy_machine,
        bullistic_authority: bullistic_authority.pubkey(),
        system_program: system_program::id(),
    }
    .to_account_metas(None);
    if token_info.set {
        accounts.push(AccountMeta::new_readonly(spl_token::id(), false));
        accounts.push(AccountMeta::new(
            freeze_info.find_freeze_ata(&token_info.mint),
            false,
        ));
        accounts.push(AccountMeta::new(token_info.auth_account, false));
    }

    let data = bullistic_candy_machine::instruction::UnlockFunds {}.data();
    let set_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };
    update_blockhash(context).await?;
    let tx = Transaction::new_signed_with_payer(
        &[set_ix],
        Some(&bullistic_authority.pubkey()),
        &[bullistic_authority],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}

#[allow(clippy::too_many_arguments)]
pub fn mint_nft_ix(
    candy_machine: &Pubkey,
    candy_creator_pda: &Pubkey,
    creator_bump: u8,
    treasury_wallet: &Pubkey,
    creator_authority: &Pubkey,
    buyer: &Keypair,
    new_nft: &MasterEditionManager,
    token_info: TokenInfo,
    spl_token_allowlist_info: SplTokenAllowlistInfo,
    collection_info: CollectionInfo,
    freeze_info: FreezeInfo,
    should_set_bot_signer_authority_as_signer: bool,
    buyer_merkle_allowlist_proof_data: Option<BuyerMerkleAllowlistProofData>,
    mint_price: u64,
) -> Vec<Instruction> {
    let metadata = new_nft.metadata_pubkey;
    let master_edition = new_nft.edition_pubkey;
    let mint = new_nft.mint.pubkey();

    let (buyer_info_account, _) = find_buyer_info_account_pda(&candy_machine, &buyer.pubkey());

    let buyer_token_account = get_associated_token_address(&buyer.pubkey(), &mint);

    let mut accounts = bullistic_candy_machine::accounts::MintNFT {
        candy_machine: *candy_machine,
        candy_machine_creator: *candy_creator_pda,
        buyer: buyer.pubkey(),
        treasury_wallet: *treasury_wallet,
        metadata,
        mint,
        creator_authority: *creator_authority,
        master_edition,
        token_metadata_program: mpl_token_metadata::id(),
        token_program: spl_token::id(),
        system_program: system_program::id(),
        rent: sysvar::rent::id(),
        recent_slothashes: sysvar::slot_hashes::id(),
        instruction_sysvar_account: sysvar::instructions::id(),
        bot_signer_authority: get_bot_signer_keypair().pubkey(),
        buyer_info_account,
        buyer_token_account,
        ata_program: AssociatedToken::id(),
    }
    .to_account_metas(None);

    if should_set_bot_signer_authority_as_signer {
        for account in accounts.iter_mut() {
            if cmp_pubkeys(&account.pubkey, &get_bot_signer_keypair().pubkey()) {
                account.is_signer = true;
            }
        }
    }

    if spl_token_allowlist_info.set {
        accounts.push(AccountMeta::new(
            spl_token_allowlist_info.minter_account,
            false,
        ));
        if spl_token_allowlist_info.spl_token_allowlist_config.burn == BurnEveryTime {
            accounts.push(AccountMeta::new(spl_token_allowlist_info.mint, false));
        }
    }

    if token_info.set {
        accounts.push(AccountMeta::new(token_info.minter_account, false));
    }

    if freeze_info.set {
        accounts.push(AccountMeta::new(freeze_info.pda, false));
        accounts.push(AccountMeta::new(new_nft.token_account, false));
        if token_info.set {
            accounts.push(AccountMeta::new(
                freeze_info.find_freeze_ata(&token_info.mint),
                false,
            ));
        }
    }

    let (_, buyer_info_account_bump) = find_buyer_info_account_pda(&candy_machine, &buyer.pubkey());
    let data = bullistic_candy_machine::instruction::MintNft {
        creator_bump,
        buyer_info_account_bump,
        buyer_merkle_allowlist_proof_data,
        expected_price: mint_price,
    }
    .data();

    let mut instructions = Vec::new();

    let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(500_000);

    instructions.push(compute_budget_ix);

    let mint_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };

    instructions.push(mint_ix);

    if collection_info.set {
        let accounts = bullistic_candy_machine::accounts::SetCollectionDuringMint {
            candy_machine: *candy_machine,
            metadata,
            buyer: buyer.pubkey(),
            collection_pda: collection_info.pda,
            token_metadata_program: mpl_token_metadata::id(),
            instruction_sysvar_account: sysvar::instructions::id(),
            collection_mint: collection_info.mint.pubkey(),
            collection_metadata: collection_info.metadata,
            collection_master_edition: collection_info.master_edition,
            creator_authority: *creator_authority,
            collection_authority_record: collection_info.authority_record,
        }
        .to_account_metas(None);
        let data = bullistic_candy_machine::instruction::SetCollectionDuringMint {}.data();
        let set_ix = Instruction {
            program_id: bullistic_candy_machine::id(),
            data,
            accounts,
        };
        instructions.push(set_ix)
    }
    instructions
}

#[allow(clippy::too_many_arguments)]
pub async fn mint_nft(
    context: &mut ProgramTestContext,
    candy_machine: &Pubkey,
    candy_creator_pda: &Pubkey,
    creator_bump: u8,
    wallet: &Pubkey,
    creator_authority: &Pubkey,
    buyer: &Keypair,
    new_nft: &MasterEditionManager,
    token_info: TokenInfo,
    spl_token_allowlist_info: SplTokenAllowlistInfo,
    collection_info: CollectionInfo,
    freeze_info: FreezeInfo,
    should_add_bot_signer: bool,
    buyer_merkle_allowlist_proof_data: Option<BuyerMerkleAllowlistProofData>,
    mint_price: u64,
) -> SolanaProgramTestResult {
    let ins = mint_nft_ix(
        candy_machine,
        candy_creator_pda,
        creator_bump,
        wallet,
        creator_authority,
        buyer,
        new_nft,
        token_info,
        spl_token_allowlist_info,
        collection_info,
        freeze_info,
        should_add_bot_signer,
        buyer_merkle_allowlist_proof_data,
        mint_price,
    );
    let bot_signer = get_bot_signer_keypair();
    let signers = if should_add_bot_signer {
        vec![buyer, &new_nft.mint, &bot_signer]
    } else {
        vec![buyer, &new_nft.mint]
    };
    update_blockhash(context).await?;
    let tx = Transaction::new_signed_with_payer(
        &ins,
        Some(&buyer.pubkey()),
        &signers,
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into())
}
