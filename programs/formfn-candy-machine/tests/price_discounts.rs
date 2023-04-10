#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use solana_program_test::*;
use solana_sdk::signature::Keypair;

use formfn_candy_machine::{BuyerMerkleAllowlistProofData, MintPhase};
use solana_sdk::signer::Signer;
use utils::helpers::sol;
use utils::{CandyConfigBuilder, DEFAULT_PRICE};

use crate::utils::{candy_machine_program_test, get_allowlist_config_data, CandyManagerBuilder};

use crate::core::helpers::{airdrop, clone_keypair};

mod core;
mod utils;

#[tokio::test]
async fn pre_mint_price_discount() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    candy_manager.set_new_minter_keypair(clone_keypair(&candy_manager.creator_authority));

    let premint_price = DEFAULT_PRICE / 4;
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Premint)
        .set_pre_mint_price(premint_price)
        .build();

    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager
        .mint_and_assert_successful(context, Some(premint_price), false, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn pre_mint_for_free() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    candy_manager.set_new_minter_keypair(clone_keypair(&candy_manager.creator_authority));

    let premint_price = 0;
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Premint)
        .set_pre_mint_price(premint_price)
        .build();

    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager
        .mint_and_assert_successful(context, Some(premint_price), false, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn mint_with_merkle_allowlist_price_discount() {
    let allowlist_config = get_allowlist_config_data();

    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_candy_machine(allowlist_config.candy_machine_keypair)
        .build(context)
        .await;

    let allowlist_price = DEFAULT_PRICE / 2;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Allowlist)
        .set_allowlist_price(allowlist_price)
        .build();
    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    for roots in allowlist_config.chunked_roots_to_add.iter() {
        candy_manager
            .append_merkle_allowlist_roots(context, roots.clone())
            .await
            .unwrap();
    }

    let buyer = &allowlist_config.allowlist_buyers[0];
    let buyer_keypair = Keypair::from_bytes(&buyer.keypair_object.secret_key).unwrap();
    candy_manager.set_new_minter_keypair(buyer_keypair);

    let sol_to_airdrop = allowlist_price * 2;
    airdrop(context, &candy_manager.minter.pubkey(), sol_to_airdrop)
        .await
        .unwrap();
    let valid_merkle_allowlist_proof_data = BuyerMerkleAllowlistProofData {
        amount: buyer.amount,
        proof: buyer.proof.clone(),
        root_index_for_proof: buyer.merkle_tree_index,
    };

    candy_manager
        .mint_and_assert_successful(
            context,
            Some(allowlist_price),
            false,
            Some(valid_merkle_allowlist_proof_data),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn mint_with_merkle_allowlist_for_free() {
    let allowlist_config = get_allowlist_config_data();

    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_candy_machine(allowlist_config.candy_machine_keypair)
        .build(context)
        .await;

    let allowlist_price = 0;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Allowlist)
        .set_allowlist_price(allowlist_price)
        .build();
    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    for roots in allowlist_config.chunked_roots_to_add.iter() {
        candy_manager
            .append_merkle_allowlist_roots(context, roots.clone())
            .await
            .unwrap();
    }

    let buyer = &allowlist_config.allowlist_buyers[0];
    let buyer_keypair = Keypair::from_bytes(&buyer.keypair_object.secret_key).unwrap();
    candy_manager.set_new_minter_keypair(buyer_keypair);

    airdrop(context, &candy_manager.minter.pubkey(), sol(1))
        .await
        .unwrap();
    let valid_merkle_allowlist_proof_data = BuyerMerkleAllowlistProofData {
        amount: buyer.amount,
        proof: buyer.proof.clone(),
        root_index_for_proof: buyer.merkle_tree_index,
    };

    candy_manager
        .mint_and_assert_successful(
            context,
            Some(allowlist_price),
            false,
            Some(valid_merkle_allowlist_proof_data),
        )
        .await
        .unwrap();
}
