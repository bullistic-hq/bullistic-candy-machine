use anchor_lang::prelude::*;

use crate::{constants::NUMBER_OF_MERKLE_ROOTS_TO_STORE, CandyError, CandyMachine};

/// Append roots to the candy machine merkle allowlist root list.
#[derive(Accounts)]
pub struct AppendMerkleAllowlistRoots<'info> {
    formfn_authority: Signer<'info>,
    #[account(
        mut,
        has_one = formfn_authority
    )]
    candy_machine: Account<'info, CandyMachine>,
}

pub fn handle_append_merkle_allowlist_roots(
    ctx: Context<AppendMerkleAllowlistRoots>,
    mut roots_to_append: Vec<[u8; 32]>,
) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;

    if candy_machine.data.spl_token_allowlist_settings.is_some() {
        return Err(CandyError::InvalidAllowlistSettings.into());
    }

    let merkle_allowlist_root_list = &mut candy_machine.data.merkle_allowlist_root_list;

    let roots_to_append_length = roots_to_append.len();
    let list_length_after_append = merkle_allowlist_root_list.len() + roots_to_append_length;

    if list_length_after_append > NUMBER_OF_MERKLE_ROOTS_TO_STORE {
        msg!(
            "Request to append {} roots to list with maximum length of {} is invalid.",
            roots_to_append_length,
            NUMBER_OF_MERKLE_ROOTS_TO_STORE
        );
        return err!(CandyError::MaximumRootCountExceeded);
    }

    merkle_allowlist_root_list.append(&mut roots_to_append);

    msg!(
        "Successfully appended {} new roots to the merkle allowlist root list. Total root list length = {}.",
        roots_to_append_length,
        merkle_allowlist_root_list.len()
    );

    Ok(())
}

/// Clear the roots list. Update is append-only so the only way to change an
/// existing roots list is to clear it and recreate the entire list.
#[derive(Accounts)]
pub struct ClearMerkleAllowlistRoots<'info> {
    formfn_authority: Signer<'info>,
    #[account(
        mut,
        has_one = formfn_authority
    )]
    candy_machine: Account<'info, CandyMachine>,
}

pub fn handle_clear_merkle_allowlist_roots(ctx: Context<ClearMerkleAllowlistRoots>) -> Result<()> {
    let candy_machine = &mut ctx.accounts.candy_machine;

    let existing_root_list_length = candy_machine.data.merkle_allowlist_root_list.len();

    let empty_root_list: Vec<[u8; 32]> = Vec::new();
    candy_machine.data.merkle_allowlist_root_list = empty_root_list;

    msg!(
        "Successfully cleared merkle allowlist root list. Previous root list length = {}.",
        existing_root_list_length
    );

    Ok(())
}
