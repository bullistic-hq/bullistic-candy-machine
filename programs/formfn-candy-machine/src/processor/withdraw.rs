use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;

use crate::constants::{FREEZE_FEATURE_INDEX, FREEZE_LOCK_FEATURE_INDEX};
use crate::{cmp_pubkeys, is_feature_active, CandyError, CandyMachine, CollectionPda};

/// Withdraw SOL from candy machine account.
#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut, close = formfn_authority, has_one = formfn_authority)]
    candy_machine: Account<'info, CandyMachine>,
    #[account(mut, address = candy_machine.formfn_authority)]
    formfn_authority: Signer<'info>,
    // > Only if collection
    // CollectionPda account
}

pub fn handle_withdraw_funds<'info>(
    ctx: Context<'_, '_, '_, 'info, WithdrawFunds<'info>>,
) -> Result<()> {
    let authority = &ctx.accounts.formfn_authority;
    let candy_machine = &ctx.accounts.candy_machine;
    if is_feature_active(&candy_machine.data.uuid, FREEZE_FEATURE_INDEX) {
        return err!(CandyError::NoWithdrawWithFreeze);
    }
    if is_feature_active(&candy_machine.data.uuid, FREEZE_LOCK_FEATURE_INDEX) {
        return err!(CandyError::NoWithdrawWithFrozenFunds);
    }

    if !ctx.remaining_accounts.is_empty() {
        let candy_key = candy_machine.key();
        let seeds = [CollectionPda::PREFIX.as_bytes(), candy_key.as_ref()];
        let collection_pda = &ctx.remaining_accounts[0];
        if !cmp_pubkeys(
            &collection_pda.key(),
            &Pubkey::find_program_address(&seeds, &crate::id()).0,
        ) {
            return err!(CandyError::MismatchedCollectionPda);
        }
        let collection_pda: Account<CollectionPda> =
            Account::try_from(&collection_pda.to_account_info())?;
        collection_pda.close(authority.to_account_info())?;
    }

    Ok(())
}
