use chrono::Duration;

use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::pubkey::Pubkey;

use bullistic_candy_machine::{
    CandyMachineData, Creator, HiddenSettings, MintPhase, SplTokenAllowlistSettings,
};
use solana_sdk::signer::Signer;

use super::{helpers::get_current_unix_timestamp, CandyManager};

pub const DEFAULT_UUID: &str = "ABCDEF";
pub const DEFAULT_PRICE: u64 = LAMPORTS_PER_SOL;
pub const DEFAULT_ITEMS_AVAILABLE: u64 = 11;
pub const DEFAULT_SYMBOL: &str = "SYMBOL";

pub struct CandyConfigBuilder {
    creator: Pubkey,
    omni_mint_wallets: Vec<Pubkey>,
    allowlist_sale_start_time: Option<i64>,
    public_sale_start_time: i64,
    public_sale_end_time: i64,
    is_mutable: bool,
    hidden_settings: Option<HiddenSettings>,
    spl_token_allowlist_settings: Option<SplTokenAllowlistSettings>,
    bot_protection_enabled: bool,
    limit_per_address: u16,
    sequential_mint_order_enabled: bool,
    items_available: u64,
    price: u64,
    premint_price: Option<u64>,
    allowlist_price: Option<u64>,
}

impl CandyConfigBuilder {
    pub fn new(candy_manager: &CandyManager) -> CandyConfigBuilder {
        let spl_token_allowlist_info = candy_manager.spl_token_allowlist_info.clone();
        let spl_token_allowlist_settings = match candy_manager.spl_token_allowlist_info.set {
            true => Some(SplTokenAllowlistSettings {
                mint: spl_token_allowlist_info.mint,
                mode: spl_token_allowlist_info.spl_token_allowlist_config.burn,
            }),
            false => None,
        };

        let now = get_current_unix_timestamp();
        let default_public_sale_end_time = now + Duration::days(1).num_seconds();

        CandyConfigBuilder {
            creator: candy_manager.creator_authority.pubkey(),
            omni_mint_wallets: vec![candy_manager.creator_authority.pubkey()],
            allowlist_sale_start_time: None,
            public_sale_start_time: 0,
            public_sale_end_time: default_public_sale_end_time,
            is_mutable: true,
            hidden_settings: None,
            spl_token_allowlist_settings,
            bot_protection_enabled: false,
            limit_per_address: 0,
            items_available: DEFAULT_ITEMS_AVAILABLE,
            sequential_mint_order_enabled: false,
            price: DEFAULT_PRICE,
            premint_price: None,
            allowlist_price: None,
        }
    }

    pub fn default(candy_manager: &CandyManager) -> CandyMachineData {
        CandyConfigBuilder::new(candy_manager).build()
    }

    pub fn set_creator(mut self, creator: Pubkey) -> CandyConfigBuilder {
        self.creator = creator;
        self
    }

    pub fn add_omni_mint_wallet(mut self, wallet: Pubkey) -> CandyConfigBuilder {
        self.omni_mint_wallets.push(wallet);
        self
    }

    pub fn set_allowlist_sale_start_time(
        mut self,
        allowlist_sale_start_time: Option<i64>,
    ) -> CandyConfigBuilder {
        self.allowlist_sale_start_time = allowlist_sale_start_time;
        self
    }

    pub fn set_public_sale_start_time(mut self, public_sale_start_time: i64) -> CandyConfigBuilder {
        self.public_sale_start_time = public_sale_start_time;
        self
    }

    pub fn set_public_sale_end_time(mut self, public_sale_end_time: i64) -> CandyConfigBuilder {
        self.public_sale_end_time = public_sale_end_time;
        self
    }

    pub fn set_is_mutable(mut self, is_mutable: bool) -> CandyConfigBuilder {
        self.is_mutable = is_mutable;
        self
    }

    pub fn set_hidden_settings(mut self, hidden_settings: HiddenSettings) -> CandyConfigBuilder {
        self.hidden_settings = Some(hidden_settings);
        self
    }

    pub fn set_spl_token_allowlist_settings(
        mut self,
        spl_token_allowlist_settings: SplTokenAllowlistSettings,
    ) -> CandyConfigBuilder {
        self.spl_token_allowlist_settings = Some(spl_token_allowlist_settings);
        self
    }

    pub fn set_bot_protection_enabled(
        mut self,
        bot_protection_enabled: bool,
    ) -> CandyConfigBuilder {
        self.bot_protection_enabled = bot_protection_enabled;
        self
    }

    pub fn set_limit_per_address(mut self, limit_per_address: u16) -> CandyConfigBuilder {
        self.limit_per_address = limit_per_address;
        self
    }

    pub fn set_sequential_mint_order_enabled(
        mut self,
        sequential_mint_order_enabled: bool,
    ) -> CandyConfigBuilder {
        self.sequential_mint_order_enabled = sequential_mint_order_enabled;
        self
    }

    pub fn set_items_available(mut self, items_available: u64) -> CandyConfigBuilder {
        self.items_available = items_available;
        self
    }

    pub fn set_price(mut self, price: u64) -> CandyConfigBuilder {
        self.price = price;
        self
    }

    pub fn set_pre_mint_price(mut self, pre_mint_price: u64) -> CandyConfigBuilder {
        self.premint_price = Some(pre_mint_price);
        self
    }

    pub fn set_allowlist_price(mut self, allowlist_price: u64) -> CandyConfigBuilder {
        self.allowlist_price = Some(allowlist_price);
        self
    }

    pub fn enable_mint_phase(self, mint_phase: MintPhase) -> CandyConfigBuilder {
        let now = get_current_unix_timestamp();
        let (allowlist_sale_start_time, public_sale_start_time, public_sale_end_time) =
            match mint_phase {
                MintPhase::Premint => (
                    now + Duration::minutes(30).num_seconds(),
                    now + Duration::minutes(60).num_seconds(),
                    now + Duration::minutes(120).num_seconds(),
                ),
                MintPhase::Allowlist => (
                    now - Duration::minutes(30).num_seconds(),
                    now + Duration::minutes(30).num_seconds(),
                    now + Duration::minutes(60).num_seconds(),
                ),
                MintPhase::Public => (
                    now - Duration::minutes(30).num_seconds(),
                    now - Duration::minutes(1).num_seconds(),
                    now + Duration::minutes(60).num_seconds(),
                ),
                MintPhase::Expired => (
                    now - Duration::minutes(30).num_seconds(),
                    now - Duration::minutes(15).num_seconds(),
                    now - Duration::minutes(1).num_seconds(),
                ),
            };

        self.set_allowlist_sale_start_time(Some(allowlist_sale_start_time))
            .set_public_sale_start_time(public_sale_start_time)
            .set_public_sale_end_time(public_sale_end_time)
    }

    pub fn build(self) -> CandyMachineData {
        CandyMachineData {
            uuid: DEFAULT_UUID.to_string(),
            items_available: self.items_available,
            price: self.price,
            premint_price: self.premint_price,
            allowlist_price: self.allowlist_price,
            symbol: DEFAULT_SYMBOL.to_string(),
            seller_fee_basis_points: 500,
            max_supply: 0,
            creators: vec![Creator {
                address: self.creator,
                verified: true,
                share: 100,
            }],
            omni_mint_wallets: self.omni_mint_wallets,
            is_mutable: self.is_mutable,
            allowlist_sale_start_time: self.allowlist_sale_start_time,
            public_sale_start_time: self.public_sale_start_time,
            public_sale_end_time: self.public_sale_end_time,
            hidden_settings: self.hidden_settings,
            spl_token_allowlist_settings: self.spl_token_allowlist_settings,
            bot_protection_enabled: self.bot_protection_enabled,
            limit_per_address: self.limit_per_address,
            merkle_allowlist_root_list: vec![],
            sequential_mint_order_enabled: self.sequential_mint_order_enabled,
        }
    }
}
