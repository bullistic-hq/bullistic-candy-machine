pub mod constants;
pub mod errors;
pub mod processor;
pub mod state;
pub mod utils;

pub use errors::CandyError;
pub use processor::*;
pub use state::*;
pub use utils::*;

use anchor_lang::prelude::*;
use solana_security_txt::security_txt;

extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;

security_txt! {
    name: "Formfunction Candy Machine Program",
    project_url: "https://formfunction.xyz",
    source_code: "https://github.com/formfunction-hq",
    contacts: "email:matt@formfunction.xyz",
    policy: "https://formfunction.notion.site/Security-Policy-4ca262c13fe3452087b356ef1fd165e9",
    preferred_languages: "en",
    auditors: "n/a",
    acknowledgements: "Thanks for visiting."
}

declare_id!("gachaC2NGh63y4ogLK8xHLeB5ZFZ8ypDLXEQyKNm8sy");

#[program]
pub mod formfn_candy_machine {
    use super::*;

    pub fn initialize_candy_machine(
        ctx: Context<InitializeCandyMachine>,
        data: CandyMachineData,
    ) -> Result<()> {
        handle_initialize_candy_machine(ctx, data)
    }

    pub fn update_candy_machine(
        ctx: Context<UpdateCandyMachine>,
        data: CandyMachineData,
    ) -> Result<()> {
        handle_update_candy_machine(ctx, data)
    }

    pub fn update_authority(
        ctx: Context<UpdateCandyMachine>,
        new_authority: Option<Pubkey>,
    ) -> Result<()> {
        handle_update_authority(ctx, new_authority)
    }

    pub fn append_merkle_allowlist_roots(
        ctx: Context<AppendMerkleAllowlistRoots>,
        roots_to_append: Vec<[u8; 32]>,
    ) -> Result<()> {
        handle_append_merkle_allowlist_roots(ctx, roots_to_append)
    }

    pub fn clear_merkle_allowlist_roots(ctx: Context<ClearMerkleAllowlistRoots>) -> Result<()> {
        handle_clear_merkle_allowlist_roots(ctx)
    }

    pub fn add_config_lines(
        ctx: Context<AddConfigLines>,
        index: u32,
        config_lines: Vec<ConfigLine>,
    ) -> Result<()> {
        handle_add_config_lines(ctx, index, config_lines)
    }

    pub fn set_collection(ctx: Context<SetCollection>) -> Result<()> {
        handle_set_collection(ctx)
    }

    pub fn remove_collection(ctx: Context<RemoveCollection>) -> Result<()> {
        handle_remove_collection(ctx)
    }

    pub fn mint_nft<'info>(
        ctx: Context<'_, '_, '_, 'info, MintNFT<'info>>,
        creator_bump: u8,
        buyer_info_account_bump: u8,
        buyer_merkle_allowlist_proof_data: Option<BuyerMerkleAllowlistProofData>,
        expected_price: u64,
    ) -> Result<()> {
        handle_mint_nft(
            ctx,
            creator_bump,
            buyer_info_account_bump,
            buyer_merkle_allowlist_proof_data,
            expected_price,
        )
    }

    pub fn set_collection_during_mint(ctx: Context<SetCollectionDuringMint>) -> Result<()> {
        handle_set_collection_during_mint(ctx)
    }

    pub fn withdraw_funds<'info>(
        ctx: Context<'_, '_, '_, 'info, WithdrawFunds<'info>>,
    ) -> Result<()> {
        handle_withdraw_funds(ctx)
    }

    pub fn set_freeze(ctx: Context<SetFreeze>, freeze_time: i64) -> Result<()> {
        handle_set_freeze(ctx, freeze_time)
    }

    pub fn remove_freeze(ctx: Context<RemoveFreeze>) -> Result<()> {
        handle_remove_freeze(ctx)
    }

    pub fn thaw_nft(ctx: Context<ThawNFT>) -> Result<()> {
        handle_thaw_nft(ctx)
    }

    pub fn unlock_funds<'info>(ctx: Context<'_, '_, '_, 'info, UnlockFunds<'info>>) -> Result<()> {
        handle_unlock_funds(ctx)
    }
}
