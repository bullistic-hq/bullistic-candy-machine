#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

use bullistic_candy_machine::{
    MintPhase,
    SplTokenAllowlistMode::{BurnEveryTime, NeverBurn},
};
use spl_associated_token_account::get_associated_token_address;
use utils::CandyConfigBuilder;

use crate::core::helpers::{airdrop, get_token_balance};
use crate::utils::{
    candy_machine_program_test, helpers::sol, CandyManagerBuilder, SplTokenAllowlistConfig,
};

mod core;
mod utils;

#[tokio::test]
async fn mint_using_spl_token_allowlist() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let spl_token_allowlist_settings = SplTokenAllowlistConfig::new(BurnEveryTime);

    let mut candy_manager = CandyManagerBuilder::new()
        .set_collection(true)
        .set_spl_token_allowlist_config(spl_token_allowlist_settings.clone())
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Premint)
        .build();

    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();
    candy_manager.set_collection(context).await.unwrap();

    candy_manager
        .mint_and_assert_bot_tax(context, None, None)
        .await
        .unwrap();
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Allowlist)
        .set_spl_token_allowlist_settings(SplTokenAllowlistConfig::to_candy_format(
            spl_token_allowlist_settings,
            &candy_manager.spl_token_allowlist_info.mint,
        ))
        .set_allowlist_price(1)
        .build();

    candy_manager
        .update(context, None, candy_data)
        .await
        .unwrap();
    candy_manager
        .mint_and_assert_successful(context, Some(1), false, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn spl_token_allowlist_burn_mode_burns_the_token() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let spl_token_allowlist_settings = SplTokenAllowlistConfig::new(BurnEveryTime);

    let mut candy_manager = CandyManagerBuilder::new()
        .set_collection(true)
        .set_spl_token_allowlist_config(spl_token_allowlist_settings.clone())
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Allowlist)
        .set_spl_token_allowlist_settings(SplTokenAllowlistConfig::to_candy_format(
            spl_token_allowlist_settings,
            &candy_manager.spl_token_allowlist_info.mint,
        ))
        .build();

    let price = candy_data.price;

    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();
    candy_manager.set_collection(context).await.unwrap();

    let buyer_spl_token_ata = get_associated_token_address(
        &candy_manager.minter.pubkey(),
        &candy_manager.spl_token_allowlist_info.mint,
    );
    let buy_spl_token_balance = get_token_balance(context, &buyer_spl_token_ata).await;
    assert_eq!(
        buy_spl_token_balance, 1,
        "Buyer SPL token balance should be 1."
    );

    candy_manager
        .mint_and_assert_successful(context, Some(price), false, None)
        .await
        .unwrap();

    let buy_spl_token_balance = get_token_balance(context, &buyer_spl_token_ata).await;
    assert_eq!(
        buy_spl_token_balance, 0,
        "Buyer SPL token balance should have been burned to zero."
    );

    candy_manager
        .mint_and_assert_bot_tax(context, None, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn spl_token_allowlist_no_burn_mode_allows_multi_mint() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let spl_token_allowlist_settings = SplTokenAllowlistConfig::new(NeverBurn);

    let mut candy_manager = CandyManagerBuilder::new()
        .set_collection(true)
        .set_spl_token_allowlist_config(spl_token_allowlist_settings.clone())
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Allowlist)
        .set_spl_token_allowlist_settings(SplTokenAllowlistConfig::to_candy_format(
            spl_token_allowlist_settings,
            &candy_manager.spl_token_allowlist_info.mint,
        ))
        .build();

    let price = candy_data.price;

    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();
    candy_manager.set_collection(context).await.unwrap();

    for _ in 0..2 {
        candy_manager
            .mint_and_assert_successful(context, Some(price), false, None)
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn spl_token_is_required_for_spl_token_allowlist() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let spl_token_allowlist_settings = SplTokenAllowlistConfig::new(BurnEveryTime);

    let mut candy_manager = CandyManagerBuilder::new()
        .set_collection(true)
        .set_spl_token_allowlist_config(spl_token_allowlist_settings.clone())
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Allowlist)
        .set_spl_token_allowlist_settings(SplTokenAllowlistConfig::to_candy_format(
            spl_token_allowlist_settings,
            &candy_manager.spl_token_allowlist_info.mint,
        ))
        .set_allowlist_price(1)
        .build();

    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();
    candy_manager.set_collection(context).await.unwrap();

    let minter_without_spl_token = Keypair::new();
    airdrop(context, &minter_without_spl_token.pubkey(), sol(3))
        .await
        .unwrap();
    candy_manager.set_new_minter_keypair(minter_without_spl_token);

    candy_manager
        .mint_and_assert_bot_tax(context, None, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn can_mint_in_public_phase_with_spl_token_allowlist_set() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let spl_token_allowlist_settings = SplTokenAllowlistConfig::new(BurnEveryTime);

    let mut candy_manager = CandyManagerBuilder::new()
        .set_collection(true)
        .set_spl_token_allowlist_config(spl_token_allowlist_settings.clone())
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Public)
        .set_spl_token_allowlist_settings(SplTokenAllowlistConfig::to_candy_format(
            spl_token_allowlist_settings,
            &candy_manager.spl_token_allowlist_info.mint,
        ))
        .set_allowlist_price(1)
        .build();

    let price = candy_data.price;

    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();
    candy_manager.set_collection(context).await.unwrap();

    let minter_without_spl_token = Keypair::new();
    airdrop(context, &minter_without_spl_token.pubkey(), price * 2)
        .await
        .unwrap();
    candy_manager.set_new_minter_keypair(minter_without_spl_token);

    candy_manager
        .mint_and_assert_successful(context, Some(price), false, None)
        .await
        .unwrap();
}
