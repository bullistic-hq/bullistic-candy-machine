use crate::constants::FREEZE_FEE;
use crate::CandyError;
use anchor_lang::prelude::*;
use mpl_token_metadata::state::{MAX_NAME_LENGTH, MAX_URI_LENGTH};

/// Candy machine state and config data.
#[account]
#[derive(Default, Debug)]
pub struct CandyMachine {
    pub formfn_authority: Pubkey,
    pub creator_authority: Pubkey,
    pub treasury_wallet: Pubkey,
    pub treasury_mint: Option<Pubkey>,
    pub items_redeemed: u64,
    pub data: CandyMachineData,
    // After this is additional account space which contains the config lines
    // and related data, which is deserialized manually as a byte array.
}

/// Candy machine settings data.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct CandyMachineData {
    pub uuid: String,
    pub price: u64,
    pub premint_price: Option<u64>,
    pub allowlist_price: Option<u64>,
    /// The symbol for the asset
    pub symbol: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    pub max_supply: u64,
    pub items_available: u64,
    pub is_mutable: bool,
    pub allowlist_sale_start_time: Option<i64>,
    pub public_sale_start_time: i64,
    pub public_sale_end_time: i64,
    pub creators: Vec<Creator>,
    pub omni_mint_wallets: Vec<Pubkey>,
    pub hidden_settings: Option<HiddenSettings>,
    pub bot_protection_enabled: bool,
    // Denotes the limit per address, 0 if unlimited.
    pub limit_per_address: u16,
    // If true, minting in the pre-mint phase is in sequential order.
    pub sequential_mint_order_enabled: bool,
    // Vector of merkle tree root hashes for address based allowlist.
    pub merkle_allowlist_root_list: Vec<[u8; 32]>,
    // SPL token allowlist settings.
    pub spl_token_allowlist_settings: Option<SplTokenAllowlistSettings>,
}

impl CandyMachine {
    pub fn assert_not_minted(&self, candy_error: Error) -> Result<()> {
        if self.items_redeemed > 0 {
            Err(candy_error)
        } else {
            Ok(())
        }
    }

    pub fn get_mint_phase(&self, now: i64) -> MintPhase {
        let allowlist_sale_start_time = self.data.allowlist_sale_start_time;
        let public_sale_start_time = self.data.public_sale_start_time;
        let public_sale_end_time = self.data.public_sale_end_time;

        if now >= public_sale_end_time {
            return MintPhase::Expired;
        }

        if now >= public_sale_start_time {
            return MintPhase::Public;
        }

        match allowlist_sale_start_time {
            Some(allowlist_sale_start_time) => {
                if now >= allowlist_sale_start_time {
                    MintPhase::Allowlist
                } else {
                    MintPhase::Premint
                }
            }
            None => MintPhase::Premint,
        }
    }

    pub fn get_mint_price(&self, mint_phase: &MintPhase) -> u64 {
        let price = self.data.price;
        let premint_price = self.data.premint_price;
        let allowlist_price = self.data.allowlist_price;

        match mint_phase {
            MintPhase::Premint => premint_price.unwrap_or(price),
            MintPhase::Allowlist => allowlist_price.unwrap_or(price),
            MintPhase::Public => price,
            MintPhase::Expired => price,
        }
    }
}

/// Individual config line for storing NFT data pre-mint.
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct ConfigLine {
    pub name: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
}

// Unfortunate duplication of token metadata so that IDL picks it up.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    // In percentages, NOT basis points ;) Watch out!
    pub share: u8,
}

/// Hidden Settings for large mints used with offline data.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct HiddenSettings {
    pub name: String,
    pub uri: String,
    pub hash: [u8; 32],
}

pub const HIDDEN_SETTINGS_SPACE: usize = 1 + // Option
4 + MAX_NAME_LENGTH + // name length,
4 + MAX_URI_LENGTH + // uri length,
32; // hash

pub const BUYER_INFO_ACCOUNT_PREFIX: &str = "buyer_info_account";

#[account]
#[derive(Default)]
pub struct BuyerInfoAccount {
    /// Number bought during the Merkle allowlist phase.
    pub number_bought_merkle_allowlist_phase: u16,
    /// Number bought during the public phase.
    pub number_bought_public_phase: u16,
}

pub const BUYER_INFO_ACCOUNT_SPACE: usize = 8 + // Discriminator
2 + // number_bought_merkle_allowlist_phase
2 + // number_bought_public_phase
64; // padding

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MintPhase {
    Premint,
    Allowlist,
    Public,
    Expired,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct BuyerMerkleAllowlistProofData {
    pub amount: u16,
    pub proof: Vec<[u8; 32]>,
    pub root_index_for_proof: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SplTokenAllowlistSettings {
    pub mode: SplTokenAllowlistMode,
    pub mint: Pubkey,
}

pub const SPL_TOKEN_ALLOWLIST_SETTINGS_SPACE: usize = 1 + // Option
1 + // mode
32; // mint

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Eq, PartialEq, Debug)]
pub enum SplTokenAllowlistMode {
    BurnEveryTime,
    NeverBurn,
}

/// Collection PDA account
#[account]
#[derive(Default, Debug)]
pub struct CollectionPda {
    pub mint: Pubkey,
    pub candy_machine: Pubkey,
}

impl CollectionPda {
    pub const PREFIX: &'static str = "collection";
}

/// Collection PDA account
#[account]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct FreezePda {
    // duplicate key in order to find the candy machine without txn crawling
    pub candy_machine: Pubkey,   // 32
    pub allow_thaw: bool,        // 1
    pub frozen_count: u64,       // 8
    pub mint_start: Option<i64>, // 1 + 8
    pub freeze_time: i64,        // 8
    pub freeze_fee: u64,         // 8
}

impl FreezePda {
    pub const SIZE: usize = 8 + 32 + 32 + 1 + 8 + 1 + 8 + 8 + 8;

    pub const PREFIX: &'static str = "freeze";

    pub fn init(&mut self, candy_machine: Pubkey, mint_start: Option<i64>, freeze_time: i64) {
        self.candy_machine = candy_machine;
        self.allow_thaw = false;
        self.frozen_count = 0;
        self.mint_start = mint_start;
        self.freeze_time = freeze_time;
        self.freeze_fee = FREEZE_FEE;
    }

    pub fn thaw_eligible(&self, current_timestamp: i64, candy_machine: &CandyMachine) -> bool {
        if self.allow_thaw || candy_machine.items_redeemed >= candy_machine.data.items_available {
            return true;
        } else if let Some(start_timestamp) = self.mint_start {
            if current_timestamp >= start_timestamp + self.freeze_time {
                return true;
            }
        }
        false
    }

    pub fn assert_from_candy(&self, candy_machine: &Pubkey) -> Result<()> {
        if &self.candy_machine != candy_machine {
            return err!(CandyError::FreezePdaMismatch);
        }
        Ok(())
    }
}
