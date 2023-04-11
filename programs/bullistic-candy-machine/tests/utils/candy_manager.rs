use std::fmt::Debug;

use anchor_lang::AccountDeserialize;
use bullistic_candy_machine::{
    cmp_pubkeys, BuyerInfoAccount, BuyerMerkleAllowlistProofData, CandyError,
    SplTokenAllowlistSettings,
};
use mpl_token_metadata::pda::find_collection_authority_account;
use solana_program::clock::Clock;
use solana_program::program_option::COption;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTestContext;
use solana_sdk::{
    signature::{Keypair, Signer},
    transport,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::AccountState;

use bullistic_candy_machine::constants::{FREEZE_FEATURE_INDEX, FREEZE_LOCK_FEATURE_INDEX};
use bullistic_candy_machine::{
    constants::BOT_FEE,
    is_feature_active, CandyMachine, CandyMachineData, CollectionPda, FreezePda,
    SplTokenAllowlistMode,
    SplTokenAllowlistMode::{BurnEveryTime, NeverBurn},
};

use crate::utils::{remove_freeze, set_freeze, thaw_nft, unlock_funds};
use crate::{
    core::helpers::create_associated_token_account, utils::helpers::find_buyer_info_account_pda,
};
use crate::{
    core::helpers::update_blockhash,
    utils::helpers::{assert_tx_failed_with_error_code, CandyTestLogger},
};
use crate::{
    core::{
        helpers::{
            airdrop, assert_account_empty, clone_keypair, create_mint, get_account,
            get_account_if_exists, get_balance, get_token_account, get_token_balance,
            mint_to_wallets, prepare_nft,
        },
        MasterEditionManager, MetadataManager,
    },
    utils::{
        add_all_config_lines, clear_merkle_allowlist_roots,
        helpers::{find_candy_creator, find_collection_pda, sol},
        initialize_candy_machine, mint_nft, remove_collection, set_collection,
        update_candy_machine, SolanaProgramTestResult,
    },
};

use super::helpers::{
    get_current_unix_timestamp, parse_candy_machine_config_lines, ParsedConfigLinesResult,
};
use super::{append_merkle_allowlist_roots, DEFAULT_SOL_AIRDROP_SIZE};

#[derive(Debug)]
pub struct CandyManagerBuilder {
    candy_machine: Option<Keypair>,
    bullistic_authority: Option<Keypair>,
    creator_authority: Option<Keypair>,
    minter: Option<Keypair>,
    bot_protection_enabled: bool,
    collection: bool,
    token: bool,
    freeze: Option<FreezeConfig>,
    spl_token_allowlist_config: Option<SplTokenAllowlistConfig>,
    sol_airdrop_size_for_minter: u64,
}

impl CandyManagerBuilder {
    pub fn new() -> CandyManagerBuilder {
        CandyManagerBuilder {
            candy_machine: None,
            bullistic_authority: None,
            creator_authority: None,
            minter: None,
            bot_protection_enabled: false,
            collection: false,
            token: false,
            freeze: None,
            spl_token_allowlist_config: None,
            sol_airdrop_size_for_minter: DEFAULT_SOL_AIRDROP_SIZE,
        }
    }

    pub async fn default(context: &mut ProgramTestContext) -> CandyManager {
        CandyManagerBuilder::new().build(context).await
    }

    pub fn set_candy_machine(mut self, candy_machine: Keypair) -> CandyManagerBuilder {
        self.candy_machine = Some(candy_machine);
        self
    }

    pub fn set_bullistic_authority(mut self, bullistic_authority: Keypair) -> CandyManagerBuilder {
        self.bullistic_authority = Some(bullistic_authority);
        self
    }

    pub fn set_creator_authority(mut self, creator_authority: Keypair) -> CandyManagerBuilder {
        self.creator_authority = Some(creator_authority);
        self
    }

    pub fn set_minter(mut self, minter: Keypair) -> CandyManagerBuilder {
        self.minter = Some(minter);
        self
    }

    pub fn set_bot_protection_enabled(
        mut self,
        bot_protection_enabled: bool,
    ) -> CandyManagerBuilder {
        self.bot_protection_enabled = bot_protection_enabled;
        self
    }

    pub fn set_collection(mut self, collection: bool) -> CandyManagerBuilder {
        self.collection = collection;
        self
    }

    pub fn set_token(mut self, token: bool) -> CandyManagerBuilder {
        self.token = token;
        self
    }

    pub fn set_freeze(mut self, freeze: FreezeConfig) -> CandyManagerBuilder {
        self.freeze = Some(freeze);
        self
    }

    pub fn set_spl_token_allowlist_config(
        mut self,
        spl_token_allowlist_config: SplTokenAllowlistConfig,
    ) -> CandyManagerBuilder {
        self.spl_token_allowlist_config = Some(spl_token_allowlist_config);
        self
    }

    pub fn set_sol_airdrop_size_for_minter(
        mut self,
        sol_airdrop_size_for_minter: u64,
    ) -> CandyManagerBuilder {
        self.sol_airdrop_size_for_minter = sol_airdrop_size_for_minter;
        self
    }

    pub async fn build(self, context: &mut ProgramTestContext) -> CandyManager {
        CandyManager::init(
            context,
            self.collection,
            self.token,
            self.freeze,
            self.spl_token_allowlist_config,
            self.candy_machine,
            self.bullistic_authority,
            self.creator_authority,
            self.minter,
            self.bot_protection_enabled,
            self.sol_airdrop_size_for_minter,
        )
        .await
    }
}

#[derive(Debug)]
pub struct CandyManager {
    pub candy_machine: Keypair,
    pub bullistic_authority: Keypair,
    pub creator_authority: Keypair,
    pub treasury_wallet: Pubkey,
    pub minter: Keypair,
    pub collection_info: CollectionInfo,
    pub token_info: TokenInfo,
    pub spl_token_allowlist_info: SplTokenAllowlistInfo,
    pub freeze_info: FreezeInfo,
    pub bot_protection_enabled: bool,
}

impl Clone for CandyManager {
    fn clone(&self) -> Self {
        CandyManager {
            candy_machine: clone_keypair(&self.candy_machine),
            bullistic_authority: clone_keypair(&self.bullistic_authority),
            creator_authority: clone_keypair(&self.creator_authority),
            treasury_wallet: self.treasury_wallet,
            minter: clone_keypair(&self.minter),
            collection_info: self.collection_info.clone(),
            token_info: self.token_info.clone(),
            spl_token_allowlist_info: self.spl_token_allowlist_info.clone(),
            freeze_info: self.freeze_info.clone(),
            bot_protection_enabled: self.bot_protection_enabled,
        }
    }
}

#[derive(Debug)]
pub struct CollectionInfo {
    pub set: bool,
    pub pda: Pubkey,
    pub mint: Keypair,
    pub metadata: Pubkey,
    pub master_edition: Pubkey,
    pub token_account: Pubkey,
    pub authority_record: Pubkey,
}

impl Clone for CollectionInfo {
    fn clone(&self) -> Self {
        CollectionInfo {
            set: self.set,
            pda: self.pda,
            mint: clone_keypair(&self.mint),
            metadata: self.metadata,
            master_edition: self.master_edition,
            token_account: self.token_account,
            authority_record: self.authority_record,
        }
    }
}

impl CollectionInfo {
    #[allow(dead_code)]
    pub fn new(
        set: bool,
        pda: Pubkey,
        mint: Keypair,
        metadata: Pubkey,
        master_edition: Pubkey,
        token_account: Pubkey,
        authority_record: Pubkey,
    ) -> Self {
        CollectionInfo {
            set,
            pda,
            mint,
            metadata,
            master_edition,
            token_account,
            authority_record,
        }
    }

    pub async fn init(
        context: &mut ProgramTestContext,
        set: bool,
        candy_machine: &Pubkey,
        bullistic_authority: Keypair,
    ) -> Self {
        println!("Init Collection Info");
        let metadata_info = MetadataManager::new(&bullistic_authority);
        metadata_info
            .create_v2(
                context,
                "Collection Name".to_string(),
                "COLLECTION".to_string(),
                "URI".to_string(),
                None,
                0,
                true,
                Some(&bullistic_authority.pubkey()),
                None,
                None,
            )
            .await
            .unwrap();
        let master_edition_info = MasterEditionManager::new(&metadata_info);
        master_edition_info
            .create_v3(context, Some(0))
            .await
            .unwrap();

        let collection_pda = find_collection_pda(candy_machine).0;
        let collection_authority_record =
            find_collection_authority_account(&metadata_info.mint.pubkey(), &collection_pda).0;

        CollectionInfo {
            set,
            pda: collection_pda,
            mint: clone_keypair(&metadata_info.mint),
            metadata: metadata_info.pubkey,
            master_edition: master_edition_info.edition_pubkey,
            token_account: metadata_info.get_ata(),
            authority_record: collection_authority_record,
        }
    }
}

#[derive(Debug)]
pub struct TokenInfo {
    pub set: bool,
    pub mint: Pubkey,
    pub authority: Keypair,
    pub auth_account: Pubkey,
    pub minter_account: Pubkey,
}

impl TokenInfo {
    #[allow(dead_code)]
    pub fn new(
        set: bool,
        mint: Pubkey,
        authority: Keypair,
        auth_account: Pubkey,
        minter_account: Pubkey,
    ) -> Self {
        TokenInfo {
            set,
            mint,
            authority,
            auth_account,
            minter_account,
        }
    }

    pub async fn init(
        context: &mut ProgramTestContext,
        set: bool,
        authority: &Keypair,
        authority_alloc: (Pubkey, u64),
        minter: (Pubkey, u64),
    ) -> Self {
        println!("Init token");
        let mint = create_mint(context, &authority.pubkey(), None, 0, None)
            .await
            .unwrap();
        let atas = mint_to_wallets(
            context,
            &mint.pubkey(),
            authority,
            vec![authority_alloc, minter],
        )
        .await
        .unwrap();

        TokenInfo {
            set,
            mint: mint.pubkey(),
            authority: clone_keypair(authority),
            auth_account: atas[0],
            minter_account: atas[1],
        }
    }
}

impl Clone for TokenInfo {
    fn clone(&self) -> Self {
        TokenInfo {
            set: self.set,
            mint: self.mint,
            authority: clone_keypair(&self.authority),
            auth_account: self.auth_account,
            minter_account: self.minter_account,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FreezeInfo {
    pub freeze_time: i64,
    pub set: bool,
    pub ata: Pubkey,
    pub pda: Pubkey,
}

impl FreezeInfo {
    pub fn new(set: bool, candy_machine: &Pubkey, freeze_time: i64, mint: Pubkey) -> Self {
        let seeds: &[&[u8]] = &[FreezePda::PREFIX.as_bytes(), candy_machine.as_ref()];
        let pda = Pubkey::find_program_address(seeds, &bullistic_candy_machine::ID).0;
        let freeze_ata = get_associated_token_address(&pda, &mint);
        FreezeInfo {
            set,
            pda,
            freeze_time,
            ata: freeze_ata,
        }
    }

    pub async fn init(
        context: &mut ProgramTestContext,
        set: bool,
        candy_machine: &Pubkey,
        freeze_time: i64,
        mint: Pubkey,
    ) -> Self {
        let freeze_info = FreezeInfo::new(set, candy_machine, freeze_time, mint);
        create_associated_token_account(context, &freeze_info.pda, &mint)
            .await
            .unwrap();
        freeze_info
    }

    pub fn find_freeze_ata(&self, treasury_mint: &Pubkey) -> Pubkey {
        get_associated_token_address(&self.pda, treasury_mint)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FreezeConfig {
    pub set: bool,
    pub freeze_time: i64,
}

impl FreezeConfig {
    pub fn new(set: bool, freeze_time: i64) -> Self {
        Self { set, freeze_time }
    }
}

#[derive(Debug)]
pub struct SplTokenAllowlistInfo {
    pub set: bool,
    pub mint: Pubkey,
    pub auth_account: Pubkey,
    pub minter_account: Pubkey,
    pub spl_token_allowlist_config: SplTokenAllowlistConfig,
}

#[derive(Debug, Clone)]
pub struct SplTokenAllowlistConfig {
    pub burn: SplTokenAllowlistMode,
}

impl SplTokenAllowlistConfig {
    pub fn new(burn: SplTokenAllowlistMode) -> Self {
        SplTokenAllowlistConfig { burn }
    }

    pub fn to_candy_format(self, mint: &Pubkey) -> SplTokenAllowlistSettings {
        SplTokenAllowlistSettings {
            mint: *mint,
            mode: self.burn,
        }
    }
}

impl Default for SplTokenAllowlistConfig {
    fn default() -> Self {
        SplTokenAllowlistConfig { burn: NeverBurn }
    }
}

impl SplTokenAllowlistInfo {
    #[allow(dead_code)]
    pub fn new(
        set: bool,
        mint: Pubkey,
        auth_account: Pubkey,
        minter_account: Pubkey,
        spl_token_allowlist_config: SplTokenAllowlistConfig,
    ) -> Self {
        SplTokenAllowlistInfo {
            set,
            mint,
            auth_account,
            minter_account,
            spl_token_allowlist_config,
        }
    }

    pub async fn init(
        context: &mut ProgramTestContext,
        set: bool,
        authority: &Keypair,
        spl_token_allowlist_config: SplTokenAllowlistConfig,
        authority_alloc: (Pubkey, u64),
        minter: (Pubkey, u64),
    ) -> Self {
        println!("Init SPL token allowlist");
        let mint = create_mint(
            context,
            &authority.pubkey(),
            Some(&authority.pubkey()),
            0,
            None,
        )
        .await
        .unwrap();
        let atas = mint_to_wallets(
            context,
            &mint.pubkey(),
            authority,
            vec![authority_alloc, minter],
        )
        .await
        .unwrap();

        SplTokenAllowlistInfo {
            set,
            mint: mint.pubkey(),
            spl_token_allowlist_config,
            auth_account: atas[0],
            minter_account: atas[1],
        }
    }
}

impl Clone for SplTokenAllowlistInfo {
    fn clone(&self) -> Self {
        SplTokenAllowlistInfo {
            set: self.set,
            mint: self.mint,
            minter_account: self.minter_account,
            auth_account: self.auth_account,
            spl_token_allowlist_config: self.spl_token_allowlist_config.clone(),
        }
    }
}

impl CandyManager {
    pub fn new(
        candy_machine: Keypair,
        bullistic_authority: Keypair,
        creator_authority: Keypair,
        treasury_wallet: Pubkey,
        minter: Keypair,
        collection_info: CollectionInfo,
        token_info: TokenInfo,
        spl_token_allowlist_info: SplTokenAllowlistInfo,
        freeze_info: FreezeInfo,
        bot_protection_enabled: bool,
    ) -> Self {
        CandyManager {
            candy_machine,
            bullistic_authority,
            creator_authority,
            treasury_wallet,
            minter,
            collection_info,
            token_info,
            spl_token_allowlist_info,
            freeze_info,
            bot_protection_enabled,
        }
    }

    // NOTE/TODO: This should also include other minter related updates which
    // are covered in the init function below. However, those are related to other
    // CandyMachine features which we are currently not using, so they are omitted
    // for now.
    pub fn set_new_minter_keypair(&mut self, new_minter_keypair: Keypair) -> () {
        self.minter = new_minter_keypair;
    }

    pub async fn init(
        context: &mut ProgramTestContext,
        collection: bool,
        token: bool,
        freeze: Option<FreezeConfig>,
        spl_token_allowlist_config: Option<SplTokenAllowlistConfig>,
        candy_machine: Option<Keypair>,
        bullistic_authority: Option<Keypair>,
        creator_authority: Option<Keypair>,
        minter: Option<Keypair>,
        bot_protection_enabled: bool,
        sol_airdrop_size_for_minter: u64,
    ) -> Self {
        let logger = CandyTestLogger::new_start("Init Candy Machine Manager");
        let bullistic_authority = bullistic_authority.unwrap_or(Keypair::new());
        let default_creator_authority = if collection {
            clone_keypair(&bullistic_authority)
        } else {
            Keypair::new()
        };
        let creator_authority = creator_authority.unwrap_or(default_creator_authority);
        let candy_machine = candy_machine.unwrap_or(Keypair::new());
        let minter = minter.unwrap_or(Keypair::new());

        airdrop(
            context,
            &bullistic_authority.pubkey(),
            sol(DEFAULT_SOL_AIRDROP_SIZE),
        )
        .await
        .unwrap();

        airdrop(
            context,
            &creator_authority.pubkey(),
            sol(DEFAULT_SOL_AIRDROP_SIZE),
        )
        .await
        .unwrap();

        airdrop(context, &minter.pubkey(), sol(sol_airdrop_size_for_minter))
            .await
            .unwrap();

        let collection_info = CollectionInfo::init(
            context,
            collection,
            &candy_machine.pubkey(),
            clone_keypair(&bullistic_authority),
        )
        .await;

        let token_info = TokenInfo::init(
            context,
            token,
            &bullistic_authority,
            (bullistic_authority.pubkey(), 10),
            (minter.pubkey(), 1),
        )
        .await;

        let freeze_info = match freeze {
            Some(config) => {
                FreezeInfo::init(
                    context,
                    config.set,
                    &candy_machine.pubkey(),
                    config.freeze_time,
                    token_info.mint,
                )
                .await
            }
            None => {
                FreezeInfo::init(context, false, &candy_machine.pubkey(), 0, token_info.mint).await
            }
        };

        let spl_token_allowlist_info = match spl_token_allowlist_config {
            Some(config) => {
                SplTokenAllowlistInfo::init(
                    context,
                    true,
                    &bullistic_authority,
                    config,
                    (bullistic_authority.pubkey(), 10),
                    (minter.pubkey(), 1),
                )
                .await
            }
            None => {
                SplTokenAllowlistInfo::init(
                    context,
                    false,
                    &bullistic_authority,
                    SplTokenAllowlistConfig::default(),
                    (bullistic_authority.pubkey(), 10),
                    (minter.pubkey(), 1),
                )
                .await
            }
        };

        let treasury_wallet = match &token_info.set {
            true => token_info.auth_account,
            false => bullistic_authority.pubkey(),
        };
        logger.end();
        CandyManager::new(
            candy_machine,
            bullistic_authority,
            creator_authority,
            treasury_wallet,
            minter,
            collection_info,
            token_info,
            spl_token_allowlist_info,
            freeze_info,
            bot_protection_enabled,
        )
    }

    pub async fn get_candy(&self, context: &mut ProgramTestContext) -> CandyMachine {
        let account = get_account(context, &self.candy_machine.pubkey()).await;
        CandyMachine::try_deserialize(&mut account.data.as_ref()).unwrap()
    }

    pub async fn get_collection_pda(&self, context: &mut ProgramTestContext) -> CollectionPda {
        let account = get_account(context, &self.collection_info.pda).await;
        CollectionPda::try_deserialize(&mut account.data.as_ref()).unwrap()
    }

    pub async fn get_freeze_pda(&self, context: &mut ProgramTestContext) -> FreezePda {
        let account = get_account(context, &self.freeze_info.pda).await;
        FreezePda::try_deserialize(&mut account.data.as_ref()).unwrap()
    }

    pub async fn parse_config_lines(
        &self,
        context: &mut ProgramTestContext,
    ) -> ParsedConfigLinesResult {
        let candy_machine_state = self.get_candy(context).await;
        let candy_machine_account = get_account(context, &self.candy_machine.pubkey()).await;
        parse_candy_machine_config_lines(candy_machine_state, candy_machine_account)
    }

    pub async fn get_buyer_info_account(
        &self,
        context: &mut ProgramTestContext,
    ) -> BuyerInfoAccount {
        let (buyer_info_account_pda, _) =
            find_buyer_info_account_pda(&self.candy_machine.pubkey(), &self.minter.pubkey());
        let account = get_account(context, &buyer_info_account_pda).await;
        BuyerInfoAccount::try_deserialize(&mut account.data.as_ref()).unwrap()
    }

    pub async fn get_mint_price(&self, context: &mut ProgramTestContext) -> u64 {
        let now = get_current_unix_timestamp();
        let candy_machine = self.get_candy(context).await;
        let mint_phase = CandyMachine::get_mint_phase(&candy_machine, now);
        CandyMachine::get_mint_price(&candy_machine, &mint_phase)
    }

    pub async fn assert_freeze_set(
        &self,
        context: &mut ProgramTestContext,
        expected_freeze_pda: &FreezePda,
    ) -> FreezePda {
        let freeze_pda_account = self.get_freeze_pda(context).await;
        let candy_machine_account = self.get_candy(context).await;
        assert_eq!(*expected_freeze_pda, freeze_pda_account);
        assert!(is_feature_active(
            &candy_machine_account.data.uuid,
            FREEZE_FEATURE_INDEX
        ));
        assert!(is_feature_active(
            &candy_machine_account.data.uuid,
            FREEZE_LOCK_FEATURE_INDEX
        ));
        freeze_pda_account
    }

    pub async fn assert_frozen(
        &self,
        context: &mut ProgramTestContext,
        new_nft: &MasterEditionManager,
    ) {
        let token_account = get_token_account(context, &new_nft.token_account)
            .await
            .unwrap();
        assert_eq!(
            token_account.state,
            AccountState::Frozen,
            "Token account state is not correct"
        );
        assert_eq!(
            token_account.delegate,
            COption::Some(self.freeze_info.pda),
            "Token account delegate is not correct"
        );
        assert_eq!(
            token_account.delegated_amount, 1,
            "Delegated amount is not correct"
        );
    }

    pub async fn assert_thawed(
        &self,
        context: &mut ProgramTestContext,
        new_nft: &MasterEditionManager,
        undelegated: bool,
    ) {
        let token_account = get_token_account(context, &new_nft.token_account)
            .await
            .unwrap();
        assert_eq!(
            token_account.state,
            AccountState::Initialized,
            "Token account state is not correct"
        );
        if undelegated {
            assert_eq!(
                token_account.delegate,
                COption::None,
                "Token account delegate is not None"
            );
            assert_eq!(
                token_account.delegated_amount, 0,
                "Delegated amount is not 0"
            );
        } else {
            assert_eq!(
                token_account.delegate,
                COption::Some(self.freeze_info.pda),
                "Token account delegate is not correct"
            );
            assert_eq!(
                token_account.delegated_amount, 1,
                "Delegated amount is not correct"
            );
        }
    }

    pub async fn create(
        &mut self,
        context: &mut ProgramTestContext,
        candy_data: CandyMachineData,
    ) -> SolanaProgramTestResult {
        let logger = CandyTestLogger::new_start("Initialize Candy Machine");
        initialize_candy_machine(
            context,
            &self.candy_machine,
            &self.bullistic_authority,
            &self.creator_authority,
            &self.treasury_wallet,
            candy_data,
            self.token_info.clone(),
        )
        .await?;
        logger.end();
        Ok(())
    }

    pub async fn set_collection(
        &mut self,
        context: &mut ProgramTestContext,
    ) -> SolanaProgramTestResult {
        let logger = CandyTestLogger::new_start("Set Collection");
        set_collection(
            context,
            &self.candy_machine.pubkey(),
            &self.bullistic_authority,
            &self.creator_authority,
            &self.collection_info,
        )
        .await?;
        self.collection_info.set = true;
        logger.end();
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn remove_collection(
        &mut self,
        context: &mut ProgramTestContext,
    ) -> SolanaProgramTestResult {
        let logger = CandyTestLogger::new_start("Remove Collection");
        remove_collection(
            context,
            &self.candy_machine.pubkey(),
            &self.bullistic_authority,
            &self.collection_info,
        )
        .await?;
        self.collection_info.set = false;
        logger.end();
        Ok(())
    }

    pub async fn fill_config_lines(
        &mut self,
        context: &mut ProgramTestContext,
    ) -> SolanaProgramTestResult {
        let logger = CandyTestLogger::new_start("Fill Config Lines");
        add_all_config_lines(
            context,
            &self.candy_machine.pubkey(),
            &self.bullistic_authority,
        )
        .await?;
        logger.end();
        Ok(())
    }

    pub async fn update(
        &mut self,
        context: &mut ProgramTestContext,
        new_treasury_wallet: Option<Pubkey>,
        new_data: CandyMachineData,
    ) -> SolanaProgramTestResult {
        let logger = CandyTestLogger::new_start("Update Candy Machine");
        if let Some(treasury_wallet) = new_treasury_wallet {
            self.treasury_wallet = treasury_wallet;
        }
        let token_info = if self.token_info.set {
            Some(self.token_info.mint)
        } else {
            None
        };
        update_candy_machine(
            context,
            &self.candy_machine.pubkey(),
            &self.bullistic_authority,
            new_data,
            &self.treasury_wallet,
            token_info,
        )
        .await?;
        logger.end();
        Ok(())
    }

    pub async fn append_merkle_allowlist_roots(
        &mut self,
        context: &mut ProgramTestContext,
        roots_to_append: Vec<[u8; 32]>,
    ) -> SolanaProgramTestResult {
        let logger = CandyTestLogger::new_start("Update Candy Machine Merkle Allowlist Roots");
        append_merkle_allowlist_roots(
            context,
            &self.candy_machine.pubkey(),
            &self.bullistic_authority,
            roots_to_append,
        )
        .await?;
        logger.end();
        Ok(())
    }

    pub async fn clear_merkle_allowlist_roots(
        &mut self,
        context: &mut ProgramTestContext,
    ) -> SolanaProgramTestResult {
        let logger = CandyTestLogger::new_start("Clear Candy Machine Merkle Allowlist Roots");
        clear_merkle_allowlist_roots(
            context,
            &self.candy_machine.pubkey(),
            &self.bullistic_authority,
        )
        .await?;
        logger.end();
        Ok(())
    }

    pub async fn set_freeze(
        &mut self,
        context: &mut ProgramTestContext,
    ) -> SolanaProgramTestResult {
        let logger = CandyTestLogger::new_start("Set freeze");
        set_freeze(
            context,
            &self.candy_machine.pubkey(),
            &self.bullistic_authority,
            &self.freeze_info,
            &self.token_info,
        )
        .await?;
        self.freeze_info.set = true;
        logger.end();
        Ok(())
    }

    pub async fn remove_freeze(
        &mut self,
        context: &mut ProgramTestContext,
    ) -> SolanaProgramTestResult {
        let logger = CandyTestLogger::new_start("Remove freeze");
        remove_freeze(
            context,
            &self.candy_machine.pubkey(),
            &self.bullistic_authority,
            &self.freeze_info,
        )
        .await?;
        self.freeze_info.set = false;
        logger.end();
        Ok(())
    }

    pub async fn thaw_nft(
        &mut self,
        context: &mut ProgramTestContext,
        nft_info: &MasterEditionManager,
        bullistic_authority: &Keypair,
    ) -> SolanaProgramTestResult {
        let logger = CandyTestLogger::new_start("Thaw NFT");
        thaw_nft(
            context,
            &self.candy_machine.pubkey(),
            bullistic_authority,
            &self.freeze_info,
            nft_info,
        )
        .await?;
        logger.end();
        Ok(())
    }

    pub async fn unlock_funds(
        &mut self,
        context: &mut ProgramTestContext,
    ) -> SolanaProgramTestResult {
        let logger = CandyTestLogger::new_start("Unlock Funds");
        unlock_funds(
            context,
            &self.candy_machine.pubkey(),
            &self.bullistic_authority,
            &self.freeze_info,
            &self.token_info,
        )
        .await?;
        logger.end();
        Ok(())
    }

    pub async fn mint_nft(
        &mut self,
        context: &mut ProgramTestContext,
        // This overrides the CandyManager bot_protection_enabled setting.
        add_bot_signer_override: Option<bool>,
        buyer_merkle_allowlist_proof_data: Option<BuyerMerkleAllowlistProofData>,
    ) -> SolanaProgramTestResult<MasterEditionManager> {
        let logger = CandyTestLogger::new_start("Mint NFT");
        let nft_info = prepare_nft(&self.minter).await;
        let (candy_machine_creator, creator_bump) =
            find_candy_creator(&self.candy_machine.pubkey());
        let add_bot_signer = if let Some(signer_override) = add_bot_signer_override {
            signer_override
        } else {
            self.bot_protection_enabled
        };

        let mint_price = self.get_mint_price(context).await;

        mint_nft(
            context,
            &self.candy_machine.pubkey(),
            &candy_machine_creator,
            creator_bump,
            &self.treasury_wallet,
            &self.creator_authority.pubkey(),
            &self.minter,
            &nft_info,
            self.token_info.clone(),
            self.spl_token_allowlist_info.clone(),
            self.collection_info.clone(),
            self.freeze_info.clone(),
            add_bot_signer,
            buyer_merkle_allowlist_proof_data,
            mint_price,
        )
        .await?;
        logger.end();
        Ok(nft_info)
    }

    pub async fn mint_and_assert_successful(
        &mut self,
        context: &mut ProgramTestContext,
        balance_change: Option<u64>,
        auto_spl_token_allowlist: bool,
        buyer_merkle_allowlist_proof_data: Option<BuyerMerkleAllowlistProofData>,
    ) -> transport::Result<MasterEditionManager> {
        update_blockhash(context).await?;

        let candy_start = self.get_candy(context).await;
        let start_balance = get_balance(context, &self.minter.pubkey()).await;
        let wallet_to_use = if self.freeze_info.set && {
            let freeze = self.get_freeze_pda(context).await;
            let current_timestamp = context
                .banks_client
                .get_sysvar::<Clock>()
                .await?
                .unix_timestamp;
            !freeze.thaw_eligible(current_timestamp, &candy_start)
        } {
            if self.token_info.set {
                get_associated_token_address(&self.freeze_info.pda, &self.token_info.mint)
            } else {
                self.freeze_info.pda
            }
        } else {
            self.treasury_wallet
        };
        let start_wallet_balance = if self.token_info.set {
            get_token_balance(context, &wallet_to_use).await
        } else {
            get_balance(context, &wallet_to_use).await
        };
        let start_token_balance = get_token_balance(context, &self.token_info.minter_account).await;
        let start_spl_token_allowlist_balance =
            get_token_balance(context, &self.spl_token_allowlist_info.minter_account).await;

        let (buyer_edition_info_account_pda, _) =
            find_buyer_info_account_pda(&self.candy_machine.pubkey(), &self.minter.pubkey());
        let buyer_edition_info_account_before_minting =
            get_account_if_exists(context, &buyer_edition_info_account_pda).await;
        let buyer_info_account_should_be_created =
            candy_start.data.limit_per_address > 0 || buyer_merkle_allowlist_proof_data.is_some();

        let mut new_nft = self
            .mint_nft(context, None, buyer_merkle_allowlist_proof_data)
            .await
            .unwrap();

        update_blockhash(context).await?;

        let candy_end = self.get_candy(context).await;
        let end_balance = get_balance(context, &self.minter.pubkey()).await;
        let end_wallet_balance = if self.token_info.set {
            get_token_balance(context, &wallet_to_use).await
        } else {
            get_balance(context, &wallet_to_use).await
        };
        let end_token_balance = get_token_balance(context, &self.token_info.minter_account).await;
        let end_spl_token_allowlist_balance =
            get_token_balance(context, &self.spl_token_allowlist_info.minter_account).await;
        let metadata =
            MetadataManager::get_data_from_account(context, &new_nft.metadata_pubkey).await;
        let associated_token_account =
            get_associated_token_address(&self.minter.pubkey(), &metadata.mint);
        let associated_token_account = get_token_account(context, &associated_token_account)
            .await
            .unwrap();

        assert_eq!(
            associated_token_account.amount, 1,
            "Minter is not the owner"
        );

        assert_eq!(
            candy_start.items_redeemed + 1,
            candy_end.items_redeemed,
            "Items redeemed wasn't 1"
        );
        if self.collection_info.set {
            assert_eq!(
                &metadata.collection.as_ref().unwrap().key,
                &self.collection_info.mint.pubkey(),
                "Collection key wasn't set correctly!"
            );
            assert!(
                &metadata.collection.as_ref().unwrap().verified,
                "Collection wasn't verified!"
            );
        } else {
            assert!(
                &metadata.collection.is_none(),
                "Collection was set when it shouldn't be!"
            );
        }

        let sol_fees = {
            // These are the expected fees from the transaction. If the program
            // changes to have more/less fee-requiring steps, this number will need
            // to be manually adjusted (use the test assertion output as a guide).
            let mut fees = 11981200;
            if self.bot_protection_enabled {
                fees = fees + 5000;
            }

            let buyer_edition_account_did_not_exist_before_mint =
                buyer_edition_info_account_before_minting.unwrap().is_none();

            if buyer_edition_account_did_not_exist_before_mint
                && buyer_info_account_should_be_created
            {
                // These are the extra account creation fees for the BuyerInfoAccount.
                fees = fees + 1419840;
            }
            if self.freeze_info.set {
                let freeze_pda = self.get_freeze_pda(context).await;
                fees += freeze_pda.freeze_fee;
            };
            fees
        };

        // In the following balance change checks, if the authority is minting
        // the assertions are slightly different because the authority account
        // is set as the CM wallet.
        if let Some(change) = balance_change {
            let is_authority_minting =
                cmp_pubkeys(&self.bullistic_authority.pubkey(), &self.minter.pubkey());

            if is_authority_minting {
                assert_eq!(
                    start_wallet_balance - end_wallet_balance,
                    sol_fees,
                    "CM wallet balance changed in a weird way!"
                );
            } else {
                assert_eq!(
                    end_wallet_balance - start_wallet_balance,
                    change,
                    "CM wallet balance changed in a weird way!"
                );
            }

            if self.token_info.set {
                assert_eq!(
                    start_token_balance - end_token_balance,
                    change,
                    "SPL token balance for minter changed in a weird way!"
                );
                assert_eq!(
                    start_balance - end_balance,
                    sol_fees,
                    "SOL balance for minter changed in a different way than it should have!"
                );
            } else {
                assert_eq!(
                    start_token_balance - end_token_balance,
                    0,
                    "SPL token balance for minter changed when it shouldn't have!"
                );

                if is_authority_minting {
                    assert_eq!(
                        start_balance - end_balance,
                        sol_fees,
                        "SOL balance for minter changed in a different way than it should have!"
                    );
                } else {
                    assert_eq!(
                        start_balance - end_balance,
                        sol_fees + change,
                        "SOL balance for minter changed in a different way than it should have!"
                    );
                }
            }
        }
        if auto_spl_token_allowlist {
            if self.spl_token_allowlist_info.set
                && self
                    .spl_token_allowlist_info
                    .spl_token_allowlist_config
                    .burn
                    == BurnEveryTime
                && start_spl_token_allowlist_balance > 0
            {
                assert_eq!(
                    start_spl_token_allowlist_balance - end_spl_token_allowlist_balance,
                    1,
                    "SPL token allowlist balance didn't decrease by 1!"
                );
            } else {
                assert_eq!(
                    start_spl_token_allowlist_balance - end_spl_token_allowlist_balance,
                    0,
                    "SPL token allowlist balance changed when it shouldn't have!"
                );
            }
        }
        new_nft.authority = clone_keypair(&self.bullistic_authority);
        Ok(new_nft)
    }

    pub async fn mint_and_assert_failure(
        &mut self,
        context: &mut ProgramTestContext,
        buyer_merkle_allowlist_proof_data: Option<BuyerMerkleAllowlistProofData>,
        expected_candy_error: CandyError,
    ) -> () {
        let tx_result = self
            .mint_nft(context, None, buyer_merkle_allowlist_proof_data)
            .await;
        assert_tx_failed_with_error_code(tx_result, expected_candy_error);
    }

    pub async fn mint_and_assert_bot_tax(
        &mut self,
        context: &mut ProgramTestContext,
        add_bot_signer_override: Option<bool>,
        buyer_merkle_allowlist_proof_data: Option<BuyerMerkleAllowlistProofData>,
    ) -> SolanaProgramTestResult {
        let start_balance = get_balance(context, &self.minter.pubkey()).await;
        let start_token_balance = get_token_balance(context, &self.token_info.minter_account).await;
        let start_spl_token_allowlist_balance =
            get_token_balance(context, &self.spl_token_allowlist_info.minter_account).await;
        let candy_start = self.get_candy(context).await;
        let new_nft = self
            .mint_nft(
                context,
                add_bot_signer_override,
                buyer_merkle_allowlist_proof_data,
            )
            .await?;
        let candy_end = self.get_candy(context).await;
        let end_balance = get_balance(context, &self.minter.pubkey()).await;
        let end_token_balance = get_token_balance(context, &self.token_info.minter_account).await;
        let end_spl_token_allowlist_balance =
            get_token_balance(context, &self.spl_token_allowlist_info.minter_account).await;
        let additional_tx_fees = 10000;
        assert_eq!(
            start_balance - end_balance,
            BOT_FEE + additional_tx_fees,
            "Balance changed in an unexpected way for this bot tax!"
        );
        assert_eq!(
            start_token_balance, end_token_balance,
            "SPL token balance changed!!"
        );
        assert_eq!(
            start_spl_token_allowlist_balance, end_spl_token_allowlist_balance,
            "SPL token allowlist token balance changed!"
        );
        assert_eq!(
            candy_start.items_redeemed, candy_end.items_redeemed,
            "Items redeemed was not 0!"
        );
        assert_account_empty(context, &new_nft.metadata_pubkey).await;
        Ok(())
    }
}
