#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use bullistic_candy_machine::{
    constants::NUMBER_OF_MERKLE_ROOTS_TO_STORE, BuyerMerkleAllowlistProofData, CandyError,
    MintPhase,
};
use merkle_test_utils::get_allowlist_config_data;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::{
    core::helpers::{airdrop, clone_keypair},
    utils::{
        candy_machine_program_test,
        helpers::{assert_tx_failed_with_error_code, sol},
        merkle_test_utils, CandyConfigBuilder, CandyManagerBuilder,
    },
};

mod core;
mod utils;

#[tokio::test]
async fn create_merkle_allowlist() {
    let allowlist_config = get_allowlist_config_data();

    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_items_available(3)
        .build();
    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();
    let candy_start = candy_manager.get_candy(context).await;

    let chunked_roots_to_add = allowlist_config.chunked_roots_to_add;

    let roots_to_add_later = chunked_roots_to_add[0].clone();

    // The on-chain roots list length limit may not match the test allowlist data
    // which was generated and is used here. But in this test we want to test overflowing
    // the on-chain limit (below), so continue adding roots until we reach the on-chain limit.
    // We're not checking proofs here, so it doesn't matter we reuse the same roots.
    let mut total_roots_added = 0;
    'outer: while total_roots_added < NUMBER_OF_MERKLE_ROOTS_TO_STORE {
        for roots in chunked_roots_to_add.clone().iter() {
            let remainder_to_add = NUMBER_OF_MERKLE_ROOTS_TO_STORE - total_roots_added;
            let mut roots_to_add = roots.clone();
            if roots.len() > remainder_to_add {
                roots_to_add = if remainder_to_add == 1 {
                    [roots[0]].to_vec()
                } else {
                    roots[0..remainder_to_add].to_vec()
                };
            }

            candy_manager
                .append_merkle_allowlist_roots(context, roots_to_add.clone())
                .await
                .unwrap();

            total_roots_added += roots_to_add.len();
            if total_roots_added == NUMBER_OF_MERKLE_ROOTS_TO_STORE {
                break 'outer;
            }
        }
    }

    let tx_result = candy_manager
        .append_merkle_allowlist_roots(context, roots_to_add_later)
        .await;
    assert_tx_failed_with_error_code(tx_result, CandyError::MaximumRootCountExceeded);

    let candy_end = candy_manager.get_candy(context).await;

    let initial_root_length = candy_start.data.merkle_allowlist_root_list.len();
    assert_eq!(
        initial_root_length, 0,
        "Root list should be empty initially."
    );
    assert_eq!(
        total_roots_added,
        candy_end.data.merkle_allowlist_root_list.len(),
        "Number of on-chain merkle roots should be equal to the number in the test config data."
    );

    for (index, onchain_root) in allowlist_config.merkle_allowlist_data.iter().enumerate() {
        let expected_root = candy_end.data.merkle_allowlist_root_list[index];
        for i in 0..32 {
            assert_eq!(
                onchain_root.root[i], expected_root[i],
                "Roots should match."
            )
        }
    }
}

#[tokio::test]
async fn mint_with_merkle_allowlist() {
    let allowlist_config = get_allowlist_config_data();

    let creator_authority = Keypair::new();

    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_candy_machine(allowlist_config.candy_machine_keypair)
        .set_creator_authority(clone_keypair(&creator_authority))
        .set_bullistic_authority(creator_authority)
        .set_minter(allowlist_config.first_minter_keypair)
        .set_collection(true)
        .build(context)
        .await;

    let price_in_sol = sol(1);

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_items_available(allowlist_config.total_mint_amount)
        .enable_mint_phase(MintPhase::Allowlist)
        .build();
    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();
    candy_manager.set_collection(context).await.unwrap();

    for roots in allowlist_config.chunked_roots_to_add.iter() {
        candy_manager
            .append_merkle_allowlist_roots(context, roots.clone())
            .await
            .unwrap();
    }

    let mut current_buyer_index: u64 = 0;
    let mut total_minted = 0;

    // NOTE: We just test the first 10 buyers here because it's faster and testing
    // the entire list tends to fail at some point when the test is running in CI.
    for (buyer_index, buyer) in allowlist_config.allowlist_buyers[0..10].iter().enumerate() {
        let buyer_keypair = Keypair::from_bytes(&buyer.keypair_object.secret_key).unwrap();
        candy_manager.set_new_minter_keypair(buyer_keypair);

        let sol_to_airdrop = sol((buyer.amount * 2) as u64);
        airdrop(context, &candy_manager.minter.pubkey(), sol_to_airdrop)
            .await
            .unwrap();

        let invalid_merkle_allowlist_proof_data =
            merkle_test_utils::get_invalid_merkle_allowlist_proof_data(
                &allowlist_config.allowlist_buyers,
                buyer_index,
            );

        candy_manager
            .mint_and_assert_failure(
                context,
                Some(invalid_merkle_allowlist_proof_data),
                CandyError::InvalidAllowlistProof,
            )
            .await;

        let valid_merkle_allowlist_proof_data = BuyerMerkleAllowlistProofData {
            amount: buyer.amount,
            proof: buyer.proof.clone(),
            root_index_for_proof: buyer.merkle_tree_index,
        };

        // Buy up to the allowlist limit for this buyer.
        for i in 0..buyer.amount {
            candy_manager
                .mint_and_assert_successful(
                    context,
                    Some(price_in_sol),
                    false,
                    Some(valid_merkle_allowlist_proof_data.clone()),
                )
                .await
                .unwrap();

            total_minted = total_minted + 1;

            let buyer_info_account = candy_manager.get_buyer_info_account(context).await;
            assert_eq!(
                buyer_info_account.number_bought_public_phase, 0,
                "BuyerInfoAccount number_bought_public_phase should not be incremented."
            );
            assert_eq!(
                buyer_info_account.number_bought_merkle_allowlist_phase,
                i + 1,
                "BuyerInfoAccount number_bought_allowlist_phase should be incremented."
            );
        }

        // Buying again should fail because the buyer exceeded their allowlist amount.
        if current_buyer_index < allowlist_config.total_number_of_buyers - 1 {
            candy_manager
                .mint_and_assert_failure(
                    context,
                    Some(valid_merkle_allowlist_proof_data),
                    CandyError::AllowlistMintsAlreadyUsed,
                )
                .await;
        } else {
            // The last buyer will buy the remaining mints in the CM, after which
            // it will be empty and the next mint transaction should fail.
            candy_manager
                .mint_and_assert_failure(context, None, CandyError::CandyMachineEmpty)
                .await;
        }

        current_buyer_index += 1;
    }

    let candy_end = candy_manager.get_candy(context).await;
    assert_eq!(
        candy_end.items_redeemed, total_minted,
        "Total redeemed should match allowlist length."
    );
    println!(
        "Successfully minted {} allowlist NFTs.",
        candy_end.items_redeemed
    );
}

#[tokio::test]
async fn clear_merkle_allowlist() {
    let allowlist_config = get_allowlist_config_data();

    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_candy_machine(allowlist_config.candy_machine_keypair)
        .set_minter(allowlist_config.first_minter_keypair)
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_items_available(10)
        .build();
    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    for roots in allowlist_config.chunked_roots_to_add.iter() {
        candy_manager
            .append_merkle_allowlist_roots(context, roots.clone())
            .await
            .unwrap();
    }

    let candy_start = candy_manager.get_candy(context).await;
    let start_root_list_length = candy_start.data.merkle_allowlist_root_list.len();

    candy_manager
        .clear_merkle_allowlist_roots(context)
        .await
        .unwrap();

    let candy_end = candy_manager.get_candy(context).await;
    let end_root_list_length = candy_end.data.merkle_allowlist_root_list.len();

    assert!(
        start_root_list_length > 0,
        "Root list should have length > 0 before being cleared."
    );
    assert!(
        end_root_list_length == 0,
        "Root list should have length = 0 after being cleared."
    );
}
