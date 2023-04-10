#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use solana_program::clock::Clock;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

use formfn_candy_machine::constants::{
    FREEZE_FEATURE_INDEX, FREEZE_FEE, FREEZE_LOCK_FEATURE_INDEX, MAX_FREEZE_TIME,
};
use formfn_candy_machine::{
    is_feature_active, FreezePda, MintPhase, SplTokenAllowlistMode::BurnEveryTime,
};

use crate::core::helpers::{
    get_balance, get_token_balance, new_funded_keypair, update_blockhash_to_slot,
};
use crate::utils::helpers::test_start;
use crate::utils::FreezeConfig;
use crate::{
    core::helpers::{assert_account_empty, clone_keypair},
    utils::{
        candy_machine_program_test, helpers::sol, CandyConfigBuilder, CandyManagerBuilder,
        SplTokenAllowlistConfig,
    },
};

pub mod core;
pub mod utils;

#[tokio::test]
async fn freeze_flow_with_spl_token() {
    test_start("Test Freeze");
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let spl_token_allowlist_settings = SplTokenAllowlistConfig::new(BurnEveryTime);

    let freeze_time = 60 * 60;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_collection(true)
        .set_token(true)
        .set_freeze(FreezeConfig::new(true, freeze_time))
        .set_spl_token_allowlist_config(spl_token_allowlist_settings.clone())
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Premint)
        .build();

    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();
    assert_account_empty(context, &candy_manager.collection_info.pda).await;
    candy_manager.set_collection(context).await.unwrap();

    assert_account_empty(context, &candy_manager.freeze_info.pda).await;
    candy_manager.set_freeze(context).await.unwrap();

    let mut expected_freeze_pda = FreezePda {
        candy_machine: candy_manager.candy_machine.pubkey(),
        freeze_fee: FREEZE_FEE,
        freeze_time,
        frozen_count: 0,
        allow_thaw: false,
        mint_start: None,
    };

    candy_manager
        .assert_freeze_set(context, &expected_freeze_pda)
        .await;

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

    let new_nft = candy_manager
        .mint_and_assert_successful(context, Some(1), true, None)
        .await
        .unwrap();
    let mint_start = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .unix_timestamp;
    expected_freeze_pda.mint_start = Some(mint_start);
    expected_freeze_pda.frozen_count += 1;

    candy_manager.assert_frozen(context, &new_nft).await;
    candy_manager
        .assert_freeze_set(context, &expected_freeze_pda)
        .await;

    candy_manager
        .thaw_nft(
            context,
            &new_nft,
            &clone_keypair(&candy_manager.formfn_authority),
        )
        .await
        .unwrap_err();
    candy_manager.assert_frozen(context, &new_nft).await;

    candy_manager.remove_freeze(context).await.unwrap();
    let freeze_pda = candy_manager.get_freeze_pda(context).await;
    assert!(freeze_pda.allow_thaw, "Allow thaw is not true!");

    candy_manager
        .thaw_nft(
            context,
            &new_nft,
            &clone_keypair(&candy_manager.formfn_authority),
        )
        .await
        .unwrap();

    candy_manager.assert_thawed(context, &new_nft, false).await;

    candy_manager
        .thaw_nft(context, &new_nft, &new_nft.owner)
        .await
        .unwrap();
    candy_manager.assert_thawed(context, &new_nft, true).await;

    let pre_balance = get_token_balance(context, &candy_manager.token_info.auth_account).await;
    candy_manager.unlock_funds(context).await.unwrap();
    let post_balance = get_token_balance(context, &candy_manager.token_info.auth_account).await;
    assert!(post_balance - pre_balance >= 1);
}

#[tokio::test]
async fn freeze_update() {
    test_start("Test Freeze Update");
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let freeze_time = 60 * 60;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_freeze(FreezeConfig::new(true, freeze_time))
        .build(context)
        .await;

    let random_key = new_funded_keypair(context, sol(1)).await;
    let candy_data = CandyConfigBuilder::default(&candy_manager);
    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    assert_account_empty(context, &candy_manager.freeze_info.pda).await;
    candy_manager.set_freeze(context).await.unwrap();

    let mut expected_freeze_pda = FreezePda {
        candy_machine: candy_manager.candy_machine.pubkey(),
        freeze_fee: FREEZE_FEE,
        freeze_time,
        frozen_count: 0,
        allow_thaw: false,
        mint_start: None,
    };
    candy_manager
        .assert_freeze_set(context, &expected_freeze_pda)
        .await;

    candy_manager.remove_freeze(context).await.unwrap();

    let candy_machine_account = candy_manager.get_candy(context).await;
    assert_account_empty(context, &candy_manager.freeze_info.pda).await;
    assert!(!is_feature_active(
        &candy_machine_account.data.uuid,
        FREEZE_FEATURE_INDEX
    ));
    assert!(!is_feature_active(
        &candy_machine_account.data.uuid,
        FREEZE_LOCK_FEATURE_INDEX
    ));

    candy_manager.set_freeze(context).await.unwrap();
    candy_manager
        .assert_freeze_set(context, &expected_freeze_pda)
        .await;

    let new_nft = candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), true, None)
        .await
        .unwrap();
    candy_manager.assert_frozen(context, &new_nft).await;

    let mint_start = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .unix_timestamp;
    expected_freeze_pda.mint_start = Some(mint_start);
    expected_freeze_pda.frozen_count += 1;

    candy_manager
        .assert_freeze_set(context, &expected_freeze_pda)
        .await;

    candy_manager
        .thaw_nft(
            context,
            &new_nft,
            &clone_keypair(&candy_manager.formfn_authority),
        )
        .await
        .unwrap_err();

    candy_manager.remove_freeze(context).await.unwrap();

    expected_freeze_pda.allow_thaw = true;
    let freeze_pda = candy_manager.get_freeze_pda(context).await;
    assert_eq!(freeze_pda, expected_freeze_pda);
    let uuid = candy_manager.get_candy(context).await.data.uuid;
    assert!(!is_feature_active(&uuid, FREEZE_FEATURE_INDEX));
    assert!(is_feature_active(&uuid, FREEZE_LOCK_FEATURE_INDEX));

    candy_manager
        .thaw_nft(context, &new_nft, &random_key)
        .await
        .unwrap();
    candy_manager.assert_thawed(context, &new_nft, false).await;
    let freeze_pda = candy_manager.get_freeze_pda(context).await;
    expected_freeze_pda.frozen_count -= 1;
    assert_eq!(freeze_pda, expected_freeze_pda);

    let new_nft_2 = candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), true, None)
        .await
        .unwrap();
    candy_manager.assert_thawed(context, &new_nft_2, true).await;

    let freeze_pda_before = candy_manager.get_freeze_pda(context).await;
    candy_manager
        .thaw_nft(
            context,
            &new_nft_2,
            &clone_keypair(&candy_manager.formfn_authority),
        )
        .await
        .unwrap();
    let freeze_pda_after = candy_manager.get_freeze_pda(context).await;
    assert_eq!(freeze_pda_before, freeze_pda_after);
}

#[tokio::test]
async fn thaw_after_freeze_time() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let freeze_time = 30; //30 seconds
    let mut candy_manager = CandyManagerBuilder::new()
        .set_freeze(FreezeConfig::new(true, freeze_time))
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::default(&candy_manager);
    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    assert_account_empty(context, &candy_manager.freeze_info.pda).await;
    candy_manager.set_freeze(context).await.unwrap();

    let expected_freeze_pda = FreezePda {
        candy_machine: candy_manager.candy_machine.pubkey(),
        freeze_fee: FREEZE_FEE,
        freeze_time,
        frozen_count: 0,
        allow_thaw: false,
        mint_start: None,
    };
    candy_manager
        .assert_freeze_set(context, &expected_freeze_pda)
        .await;
    let new_nft = candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), true, None)
        .await
        .unwrap();

    //test thaw fail
    candy_manager
        .thaw_nft(context, &new_nft, &new_nft.authority)
        .await
        .unwrap_err();

    update_blockhash_to_slot(context, 50_000).await.unwrap();

    candy_manager
        .thaw_nft(context, &new_nft, &new_nft.authority)
        .await
        .unwrap();

    let thaw_time = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .unix_timestamp;

    candy_manager.assert_thawed(context, &new_nft, false).await;

    let mint_time = candy_manager
        .get_freeze_pda(context)
        .await
        .mint_start
        .unwrap();
    assert!(
        thaw_time - mint_time >= freeze_time,
        "This shouldn't happen. Something must have went wrong."
    );

    // now that freeze time has passed, new mints shouldn't be frozen
    let new_nft = candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), true, None)
        .await
        .unwrap();
    candy_manager.assert_thawed(context, &new_nft, true).await;
}

#[tokio::test]
async fn unlock_funds() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let freeze_time = 30; //30 seconds
    let mut candy_manager = CandyManagerBuilder::new()
        .set_freeze(FreezeConfig::new(true, freeze_time))
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::default(&candy_manager);
    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    assert_account_empty(context, &candy_manager.freeze_info.pda).await;
    candy_manager.set_freeze(context).await.unwrap();

    let expected_freeze_pda = FreezePda {
        candy_machine: candy_manager.candy_machine.pubkey(),
        freeze_fee: FREEZE_FEE,
        freeze_time,
        frozen_count: 0,
        allow_thaw: false,
        mint_start: None,
    };
    candy_manager
        .assert_freeze_set(context, &expected_freeze_pda)
        .await;

    let new_nft = candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), true, None)
        .await
        .unwrap();

    candy_manager.remove_freeze(context).await.unwrap();
    // shouldn't work because one nft is still frozen
    candy_manager.unlock_funds(context).await.unwrap_err();
    candy_manager
        .thaw_nft(context, &new_nft, &new_nft.owner)
        .await
        .unwrap();
    candy_manager.assert_thawed(context, &new_nft, true).await;
    let pre_balance = get_balance(context, &candy_manager.formfn_authority.pubkey()).await;
    candy_manager.unlock_funds(context).await.unwrap();
    let post_balance = get_balance(context, &candy_manager.formfn_authority.pubkey()).await;
    assert!(post_balance - pre_balance >= sol(1));
}

#[tokio::test]
async fn mint_out_unfreeze() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let freeze_time = MAX_FREEZE_TIME;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_freeze(FreezeConfig::new(true, freeze_time))
        .build(context)
        .await;

    let random_key = new_funded_keypair(context, sol(1)).await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_items_available(2)
        .build();
    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    assert_account_empty(context, &candy_manager.freeze_info.pda).await;
    candy_manager.set_freeze(context).await.unwrap();

    let expected_freeze_pda = FreezePda {
        candy_machine: candy_manager.candy_machine.pubkey(),
        freeze_fee: FREEZE_FEE,
        freeze_time,
        frozen_count: 0,
        allow_thaw: false,
        mint_start: None,
    };

    candy_manager
        .assert_freeze_set(context, &expected_freeze_pda)
        .await;

    let nft1 = candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), true, None)
        .await
        .unwrap();

    candy_manager.assert_frozen(context, &nft1).await;

    // should fail
    candy_manager
        .thaw_nft(context, &nft1, &random_key)
        .await
        .unwrap_err();

    let nft2 = candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), true, None)
        .await
        .unwrap();

    candy_manager.assert_frozen(context, &nft2).await;

    // should succeed
    candy_manager
        .thaw_nft(context, &nft1, &random_key)
        .await
        .unwrap();

    // This should fail because nft2 is still frozen
    candy_manager.unlock_funds(context).await.unwrap_err();

    candy_manager
        .thaw_nft(context, &nft2, &random_key)
        .await
        .unwrap();

    let pre_balance = get_balance(context, &candy_manager.formfn_authority.pubkey()).await;
    candy_manager.unlock_funds(context).await.unwrap();
    let post_balance = get_balance(context, &candy_manager.formfn_authority.pubkey()).await;
    assert!(post_balance - pre_balance >= sol(2));
}
