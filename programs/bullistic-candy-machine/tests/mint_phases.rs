#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use chrono::Duration;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signature::Signer};

use bullistic_candy_machine::{
    BuyerMerkleAllowlistProofData, CandyError, CandyMachineData, MintPhase,
    SplTokenAllowlistMode::BurnEveryTime,
};
use merkle_test_utils::get_empty_merkle_tree_node;
use utils::{
    helpers::{assert_tx_failed_with_error_code, get_current_unix_timestamp, sol},
    CandyConfigBuilder,
};

use crate::utils::{
    candy_machine_program_test, merkle_test_utils, CandyManagerBuilder, SplTokenAllowlistConfig,
    DEFAULT_SOL_AIRDROP_SIZE,
};

use crate::core::helpers::{airdrop, clone_keypair};

mod core;
mod utils;

#[tokio::test]
async fn create_and_update_candy_machine_mint_phase() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_collection(true)
        .set_token(true)
        .set_spl_token_allowlist_config(SplTokenAllowlistConfig::new(BurnEveryTime))
        .build(context)
        .await;

    let now = get_current_unix_timestamp();

    let mut invalid_candy_data_to_test: Vec<CandyMachineData> = vec![];

    // Public sale after end time.
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_public_sale_start_time(now + Duration::minutes(5).num_seconds())
        .set_public_sale_end_time(now + Duration::minutes(4).num_seconds())
        .build();
    invalid_candy_data_to_test.push(candy_data);

    // Allowlist sale after public sale start.
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_allowlist_sale_start_time(Some(now + Duration::minutes(3).num_seconds()))
        .set_public_sale_start_time(now + Duration::minutes(2).num_seconds())
        .set_public_sale_end_time(now + Duration::minutes(5).num_seconds())
        .build();
    invalid_candy_data_to_test.push(candy_data);

    // Allowlist sale after end time.
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_allowlist_sale_start_time(Some(now + Duration::minutes(6).num_seconds()))
        .set_public_sale_start_time(now + Duration::minutes(2).num_seconds())
        .set_public_sale_end_time(now + Duration::minutes(5).num_seconds())
        .build();
    invalid_candy_data_to_test.push(candy_data);

    for candy_data in invalid_candy_data_to_test.clone().iter() {
        let tx_result = candy_manager.create(context, candy_data.clone()).await;
        assert_tx_failed_with_error_code(tx_result, CandyError::CandyMachineInvalidMintPhases);
    }

    // Valid candy data.
    let valid_candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_allowlist_sale_start_time(Some(now + Duration::minutes(1).num_seconds()))
        .set_public_sale_start_time(now + Duration::minutes(2).num_seconds())
        .set_public_sale_end_time(now + Duration::minutes(3).num_seconds())
        .build();
    candy_manager
        .create(context, valid_candy_data.clone())
        .await
        .unwrap();

    // Test the same cases from above for updating a candy machine.
    for candy_data in invalid_candy_data_to_test.iter() {
        let tx_result = candy_manager
            .update(context, None, candy_data.clone())
            .await;
        assert_tx_failed_with_error_code(tx_result, CandyError::CandyMachineInvalidMintPhases);
    }

    // Check update is fine with valid data.
    candy_manager
        .update(context, None, valid_candy_data)
        .await
        .unwrap();
}

#[tokio::test]
async fn mint_during_premint_mint_phase() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Premint)
        .build();

    candy_manager.create(context, candy_data).await.unwrap();

    candy_manager
        .mint_and_assert_bot_tax(context, None, None)
        .await
        .unwrap();

    let invalid_allowlist_proof = BuyerMerkleAllowlistProofData {
        amount: 1,
        proof: vec![get_empty_merkle_tree_node()],
        root_index_for_proof: 0,
    };

    candy_manager
        .mint_and_assert_bot_tax(context, None, Some(invalid_allowlist_proof.clone()))
        .await
        .unwrap();
}

#[tokio::test]
async fn mint_during_allowlist_mint_phase() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Allowlist)
        .build();

    candy_manager.create(context, candy_data).await.unwrap();

    candy_manager
        .append_merkle_allowlist_roots(context, vec![get_empty_merkle_tree_node()])
        .await
        .unwrap();

    candy_manager
        .mint_and_assert_bot_tax(context, None, None)
        .await
        .unwrap();

    let invalid_allowlist_proof = BuyerMerkleAllowlistProofData {
        amount: 1,
        proof: vec![get_empty_merkle_tree_node()],
        root_index_for_proof: 0,
    };

    candy_manager
        .mint_and_assert_failure(
            context,
            Some(invalid_allowlist_proof.clone()),
            CandyError::InvalidAllowlistProof,
        )
        .await;
}

#[tokio::test]
async fn mint_during_public_mint_phase() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Public)
        .build();

    candy_manager.create(context, candy_data).await.unwrap();

    candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), false, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn mint_during_expired_mint_phase() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Expired)
        .build();

    candy_manager.create(context, candy_data).await.unwrap();

    candy_manager
        .mint_and_assert_bot_tax(context, None, None)
        .await
        .unwrap();
}

// Note: omni_mint_wallets actually cannot mint during the expired phase.
#[tokio::test]
async fn omni_minters_creator_authority_can_always_mint() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    candy_manager.set_new_minter_keypair(clone_keypair(&candy_manager.creator_authority));

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Premint)
        .build();

    candy_manager.create(context, candy_data).await.unwrap();

    candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), false, None)
        .await
        .unwrap();

    // Allowlist mint phase:
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Allowlist)
        .build();

    candy_manager
        .update(context, None, candy_data)
        .await
        .unwrap();

    candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), false, None)
        .await
        .unwrap();

    // Public mint phase:
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Public)
        .build();

    candy_manager
        .update(context, None, candy_data)
        .await
        .unwrap();

    candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), false, None)
        .await
        .unwrap();

    // Expired mint phase:
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Expired)
        .build();

    candy_manager
        .update(context, None, candy_data)
        .await
        .unwrap();

    candy_manager
        .mint_and_assert_bot_tax(context, None, None)
        .await
        .unwrap();
}

// Note: omni_mint_wallets actually cannot mint during the expired phase.
#[tokio::test]
async fn omni_minters_not_creator_authority_can_always_mint() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    let extra_creator_keypair = Keypair::new();
    airdrop(
        context,
        &extra_creator_keypair.pubkey(),
        sol(DEFAULT_SOL_AIRDROP_SIZE),
    )
    .await
    .unwrap();

    candy_manager.set_new_minter_keypair(clone_keypair(&extra_creator_keypair));

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Premint)
        .add_omni_mint_wallet(extra_creator_keypair.pubkey())
        .build();

    candy_manager.create(context, candy_data).await.unwrap();

    let candy_machine_state = candy_manager.get_candy(context).await;
    let omni_mint_wallets = candy_machine_state.data.omni_mint_wallets;
    assert!(omni_mint_wallets.contains(&extra_creator_keypair.pubkey()));
    assert!(omni_mint_wallets.contains(&candy_manager.creator_authority.pubkey()));

    candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), false, None)
        .await
        .unwrap();

    // Allowlist mint phase:
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Allowlist)
        .add_omni_mint_wallet(extra_creator_keypair.pubkey())
        .build();

    candy_manager
        .update(context, None, candy_data)
        .await
        .unwrap();

    candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), false, None)
        .await
        .unwrap();

    // Public mint phase:
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Public)
        .add_omni_mint_wallet(extra_creator_keypair.pubkey())
        .build();

    candy_manager
        .update(context, None, candy_data)
        .await
        .unwrap();

    candy_manager
        .mint_and_assert_successful(context, Some(sol(1)), false, None)
        .await
        .unwrap();

    // Expired mint phase:
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Expired)
        .add_omni_mint_wallet(extra_creator_keypair.pubkey())
        .build();

    candy_manager
        .update(context, None, candy_data)
        .await
        .unwrap();

    candy_manager
        .mint_and_assert_bot_tax(context, None, None)
        .await
        .unwrap();
}
