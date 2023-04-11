#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use anchor_client::solana_sdk::transaction::Transaction;
use bullistic_candy_machine::{CandyError, SplTokenAllowlistMode};
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::account::{AccountSharedData, WritableAccount};
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::core::helpers::{prepare_nft, update_blockhash};
use crate::utils::helpers::{assert_tx_failed_with_error_code, find_candy_creator};
use crate::utils::{
    candy_machine_program_test, mint_nft, mint_nft_ix, CandyConfigBuilder, CandyManagerBuilder,
    SplTokenAllowlistConfig, SplTokenAllowlistInfo,
};

pub mod core;
pub mod utils;

#[tokio::test]
async fn fail_metadata_not_blank() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_items_available(2)
        .build();
    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    let nft_info = prepare_nft(&candy_manager.minter).await;

    context.set_account(
        &nft_info.metadata_pubkey,
        &AccountSharedData::create(
            1000000000,
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
            mpl_token_metadata::id(),
            false,
            1,
        ),
    );
    let (candy_machine_creator, creator_bump) =
        find_candy_creator(&candy_manager.candy_machine.pubkey());

    let mint_price = candy_manager.get_mint_price(context).await;

    let tx_result = mint_nft(
        context,
        &candy_manager.candy_machine.pubkey(),
        &candy_machine_creator,
        creator_bump,
        &candy_manager.treasury_wallet,
        &candy_manager.creator_authority.pubkey(),
        &candy_manager.minter,
        &nft_info,
        candy_manager.token_info.clone(),
        candy_manager.spl_token_allowlist_info.clone(),
        candy_manager.collection_info.clone(),
        candy_manager.freeze_info.clone(),
        false,
        None,
        mint_price,
    )
    .await;
    assert_tx_failed_with_error_code(tx_result, CandyError::MetadataAccountMustBeEmpty);
}

#[tokio::test]
async fn metadata_check_before_bot_tax() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_items_available(2)
        .build();
    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    let nft_info = prepare_nft(&candy_manager.minter).await;
    candy_manager.spl_token_allowlist_info = SplTokenAllowlistInfo {
        set: true,
        mint: Pubkey::new_unique(),
        auth_account: Pubkey::new_unique(),
        minter_account: Pubkey::new_unique(),
        spl_token_allowlist_config: SplTokenAllowlistConfig {
            burn: SplTokenAllowlistMode::BurnEveryTime,
        },
    };

    context.set_account(
        &nft_info.metadata_pubkey,
        &AccountSharedData::create(
            1000000000,
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
            mpl_token_metadata::id(),
            false,
            1,
        ),
    );
    let (candy_machine_creator, creator_bump) =
        find_candy_creator(&candy_manager.candy_machine.pubkey());

    let mint_price = candy_manager.get_mint_price(context).await;

    let mut ix = mint_nft_ix(
        &candy_manager.candy_machine.pubkey(),
        &candy_machine_creator,
        creator_bump,
        &candy_manager.treasury_wallet,
        &candy_manager.creator_authority.pubkey(),
        &candy_manager.minter,
        &nft_info,
        candy_manager.token_info.clone(),
        candy_manager.spl_token_allowlist_info.clone(),
        candy_manager.collection_info.clone(),
        candy_manager.freeze_info.clone(),
        false,
        None,
        mint_price,
    );

    ix[0].accounts.pop();
    update_blockhash(context).await.unwrap();
    let tx = Transaction::new_signed_with_payer(
        ix.as_slice(),
        Some(&candy_manager.minter.pubkey()),
        &[&candy_manager.minter, &nft_info.mint],
        context.last_blockhash,
    );

    let tx_result = context
        .banks_client
        .process_transaction(tx)
        .await
        .map_err(|e| e.into());
    assert_tx_failed_with_error_code(tx_result, CandyError::MetadataAccountMustBeEmpty);
}
