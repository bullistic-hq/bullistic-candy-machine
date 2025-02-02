use anchor_lang::prelude::*;

use crate::constants::FREEZE_FEATURE_INDEX;
use crate::{
    is_feature_active, validate_candy_machine_allowlist_state, validate_mint_phase_times,
    CandyError, CandyMachine, CandyMachineData,
};

/// Update the candy machine state.
#[derive(Accounts)]
pub struct UpdateCandyMachine<'info> {
    #[account(
        mut,
        has_one = bullistic_authority
    )]
    candy_machine: Account<'info, CandyMachine>,
    bullistic_authority: Signer<'info>,
    /// CHECK: wallet can be any account and is not written to or read
    treasury_wallet: UncheckedAccount<'info>,
    // Remaining accounts.
    // treasury mint
}

pub fn handle_update_authority(
    ctx: Context<UpdateCandyMachine>,
    new_authority: Option<Pubkey>,
) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;

    if let Some(new_auth) = new_authority {
        if is_feature_active(&candy_machine.data.uuid, FREEZE_FEATURE_INDEX)
            && candy_machine.bullistic_authority != new_auth
        {
            return err!(CandyError::NoChangingAuthorityWithFreeze);
        }
        candy_machine.bullistic_authority = new_auth;
    }

    Ok(())
}

// updates without modifying UUID
pub fn handle_update_candy_machine(
    ctx: Context<UpdateCandyMachine>,
    data: CandyMachineData,
) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;

    // Note: there is currently no validation to ensure an update doesn't change
    // any sale time settings after sales have already begun.
    validate_mint_phase_times(&data)?;

    validate_candy_machine_allowlist_state(&data)?;

    if data.items_available != candy_machine.data.items_available && data.hidden_settings.is_none()
    {
        return err!(CandyError::CannotChangeNumberOfLines);
    }

    let treasury_mint = ctx
        .remaining_accounts
        .get(0)
        .map(|account_info| account_info.key());

    if candy_machine.data.items_available > 0
        && candy_machine.data.hidden_settings.is_none()
        && data.hidden_settings.is_some()
    {
        return err!(CandyError::CannotSwitchToHiddenSettings);
    }

    let old_uuid = candy_machine.data.uuid.clone();
    if is_feature_active(&old_uuid, FREEZE_FEATURE_INDEX)
        && candy_machine.treasury_mint != treasury_mint
    {
        return err!(CandyError::NoChangingTokenWithFreeze);
    }

    candy_machine.treasury_wallet = ctx.accounts.treasury_wallet.key();
    candy_machine.data = data;
    candy_machine.data.uuid = old_uuid;
    candy_machine.treasury_mint = treasury_mint;

    Ok(())
}
