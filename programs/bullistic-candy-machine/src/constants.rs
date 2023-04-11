use mpl_token_metadata::state::{
    MAX_CREATOR_LEN, MAX_CREATOR_LIMIT, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH,
};
use solana_program::pubkey::Pubkey;

use crate::{HIDDEN_SETTINGS_SPACE, SPL_TOKEN_ALLOWLIST_SETTINGS_SPACE};

pub const EXPIRE_OFFSET: i64 = 10 * 60;
pub const PREFIX: &str = "candy_machine";

pub const BULLISTIC_CANDY_MACHINE_ERROR_OFFSET: u32 = 2000;

pub const BOT_FEE: u64 = 10000000; // 0.1 SOL
pub const FREEZE_FEE: u64 = 0; //100000; // 0.0001 SOL

pub const MAX_FREEZE_TIME: i64 = 60 * 60 * 24 * 31; // 1 month

pub const COLLECTIONS_FEATURE_INDEX: usize = 0;
pub const FREEZE_FEATURE_INDEX: usize = 1;
pub const FREEZE_LOCK_FEATURE_INDEX: usize = 2;

pub const COLLECTION_PDA_SIZE: usize = 8 + 32 + 32;

pub const CONFIG_LINE_SIZE: usize = 4 + MAX_NAME_LENGTH + 4 + MAX_URI_LENGTH;

pub const A_TOKEN: Pubkey = solana_program::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
pub const COMPUTE_BUDGET: Pubkey =
    solana_program::pubkey!("ComputeBudget111111111111111111111111111111");

pub const ANTI_BOT_DEV_AUTHORITY: Pubkey =
    solana_program::pubkey!("antiDV8bRvF4XTeRqmyHV1jpHD4Lvz7gKBKBBRQb8ir");

pub const ANTI_BOT_MAINNET_AUTHORITY: Pubkey =
    solana_program::pubkey!("antiScHGm8NAqfpdFNYbv3c9ntY6xksvvTN3B9cDf5Y");

pub const MERKLE_ROOT_SIZE: usize = 32;
pub const NUMBER_OF_MERKLE_ROOTS_TO_STORE: usize = 100;
pub const MERKLE_ALLOWLIST_ROOT_LIST_SPACE: usize =
    MERKLE_ROOT_SIZE * NUMBER_OF_MERKLE_ROOTS_TO_STORE;

// This value is limited by the max transaction size.
pub const MAX_ROOT_NUMBER_PER_APPEND_MERKLE_ALLOWLIST_TX: usize = 10;

pub const MAX_OMNI_MINT_WALLETS: usize = 5;

pub const CONFIG_ARRAY_START: usize = 8 + // key
// CandyMachine:
32 + // bullistic_authority
32 + // creator_authority
32 + // wallet
33 + // token mint
8 + // items redeemed
// CandyMachineData:
4 + 6 + // uuid
8 + // price
9 + // optional premint_price
9 + // optional allowlist_price
4 + MAX_SYMBOL_LENGTH + // u32 len + symbol
8 + // items available
2 + // seller fee basis points
8 + // max supply
1 + // is mutable
9 + // allowlist_sale_start_time
8 + // public_sale_start_time
8 + // public_sale_end_time
4 + MAX_CREATOR_LIMIT * MAX_CREATOR_LEN + // creators vec
4 + 32 * MAX_OMNI_MINT_WALLETS + // omni_mint_wallets vec
HIDDEN_SETTINGS_SPACE + // hidden_settings
1 + // bot_protection_enabled
2 + // limit_per_address
1 + // sequential_mint_order_enabled
4 + MERKLE_ALLOWLIST_ROOT_LIST_SPACE + // merkle_allowlist_root_list vec
SPL_TOKEN_ALLOWLIST_SETTINGS_SPACE; // spl_token_allowlist_settings
