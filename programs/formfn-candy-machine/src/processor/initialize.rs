use anchor_lang::{prelude::*, Discriminator};
use mpl_token_metadata::state::{MAX_CREATOR_LIMIT, MAX_SYMBOL_LENGTH};
use spl_token::state::Mint;

use crate::{
    assert_initialized, assert_owned_by, cmp_pubkeys,
    constants::{CONFIG_ARRAY_START, CONFIG_LINE_SIZE, MAX_OMNI_MINT_WALLETS},
    validate_candy_machine_allowlist_state, validate_mint_phase_times, CandyError, CandyMachine,
    CandyMachineData,
};

/// Create a new candy machine.
#[derive(Accounts)]
#[instruction(data: CandyMachineData)]
pub struct InitializeCandyMachine<'info> {
    /// CHECK: account constraints checked in account trait
    #[account(
        zero,
        rent_exempt = skip,
        constraint = candy_machine.to_account_info().owner == program_id &&
        candy_machine.to_account_info().data_len() >= get_space_for_candy(data)?)
    ]
    candy_machine: UncheckedAccount<'info>,
    /// CHECK: treasury_wallet can be any account and is not written to or read
    treasury_wallet: UncheckedAccount<'info>,
    /// CHECK: formfn_authority can be any account and is not written to or read
    formfn_authority: UncheckedAccount<'info>,
    /// CHECK: creator_authority can be any account and is not written to or read
    creator_authority: UncheckedAccount<'info>,
    payer: Signer<'info>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize_candy_machine(
    ctx: Context<InitializeCandyMachine>,
    data: CandyMachineData,
) -> Result<()> {
    let candy_machine_account = &mut ctx.accounts.candy_machine;

    if data.uuid.len() != 6 {
        return err!(CandyError::UuidMustBeExactly6Length);
    }

    validate_mint_phase_times(&data)?;

    validate_candy_machine_allowlist_state(&data)?;

    let mut candy_machine = CandyMachine {
        data,
        formfn_authority: ctx.accounts.formfn_authority.key(),
        creator_authority: ctx.accounts.creator_authority.key(),
        treasury_wallet: ctx.accounts.treasury_wallet.key(),
        treasury_mint: None,
        items_redeemed: 0,
    };

    candy_machine.data.uuid = "000000".to_string();

    if !ctx.remaining_accounts.is_empty() {
        let treasury_mint_info = &ctx.remaining_accounts[0];
        let _treasury_mint: Mint = assert_initialized(treasury_mint_info)?;
        let token_account: spl_token::state::Account =
            assert_initialized(&ctx.accounts.treasury_wallet)?;

        assert_owned_by(treasury_mint_info, &spl_token::id())?;
        assert_owned_by(&ctx.accounts.treasury_wallet, &spl_token::id())?;

        if !cmp_pubkeys(&token_account.mint, &treasury_mint_info.key()) {
            return err!(CandyError::MintMismatch);
        }

        candy_machine.treasury_mint = Some(*treasury_mint_info.key);
    }

    let mut array_of_zeroes = vec![];
    while array_of_zeroes.len() < MAX_SYMBOL_LENGTH - candy_machine.data.symbol.len() {
        array_of_zeroes.push(0u8);
    }
    let new_symbol =
        candy_machine.data.symbol.clone() + std::str::from_utf8(&array_of_zeroes).unwrap();
    candy_machine.data.symbol = new_symbol;

    if candy_machine.data.creators.len() > MAX_CREATOR_LIMIT {
        return err!(CandyError::TooManyCreators);
    }

    if candy_machine.data.omni_mint_wallets.len() > MAX_OMNI_MINT_WALLETS {
        return err!(CandyError::TooManyOmniMintWallets);
    }

    let mut new_data = CandyMachine::discriminator().try_to_vec().unwrap();
    new_data.append(&mut candy_machine.try_to_vec().unwrap());
    let mut data = candy_machine_account.data.borrow_mut();
    // god forgive me couldnt think of better way to deal with this
    for i in 0..new_data.len() {
        data[i] = new_data[i];
    }

    // only if we are not using hidden settings we will have space for
    // the config lines
    if candy_machine.data.hidden_settings.is_none() {
        let vec_start = CONFIG_ARRAY_START
            + 4
            + (candy_machine.data.items_available as usize) * CONFIG_LINE_SIZE;
        let as_bytes = (candy_machine
            .data
            .items_available
            .checked_div(8)
            .ok_or(CandyError::NumericalOverflowError)? as u32)
            .to_le_bytes();
        for i in 0..4 {
            data[vec_start + i] = as_bytes[i]
        }
    }

    Ok(())
}

pub fn get_space_for_candy(data: CandyMachineData) -> Result<usize> {
    let num = if data.hidden_settings.is_some() {
        CONFIG_ARRAY_START
    } else {
        CONFIG_ARRAY_START
            + 4
            + (data.items_available as usize) * CONFIG_LINE_SIZE
            + 8
            + 2 * ((data
                .items_available
                .checked_div(8)
                .ok_or(CandyError::NumericalOverflowError)?
                + 1) as usize)
    };

    Ok(num)
}
