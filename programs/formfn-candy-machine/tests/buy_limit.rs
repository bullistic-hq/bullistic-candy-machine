#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use formfn_candy_machine::{
    BuyerMerkleAllowlistProofData, CandyError, MintPhase, SplTokenAllowlistMode::BurnEveryTime,
};
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::{
    core::helpers::airdrop,
    utils::{
        candy_machine_program_test, get_allowlist_config_data, helpers::sol, CandyConfigBuilder,
        CandyManagerBuilder, SplTokenAllowlistConfig, DEFAULT_PRICE,
    },
};

mod core;
mod utils;

#[tokio::test]
async fn buy_limit() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::default(context).await;

    let price_in_sol = sol(1);
    let limit_per_address = 3;
    let total_nft_supply = 10;
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_limit_per_address(limit_per_address)
        .set_items_available(total_nft_supply)
        .build();

    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    let candy_start = candy_manager.get_candy(context).await;
    assert_eq!(
        candy_start.data.limit_per_address, limit_per_address,
        "limit_per_address should match provided value: {}",
        limit_per_address
    );

    let number_of_buyers_to_test = 3;
    for buyer_index in 0..number_of_buyers_to_test {
        for _ in 0..limit_per_address {
            candy_manager
                .mint_and_assert_successful(context, Some(price_in_sol), true, None)
                .await
                .unwrap();
        }

        candy_manager
            .mint_and_assert_failure(context, None, CandyError::BuyLimitPerAddressExceeded)
            .await;

        if buyer_index < number_of_buyers_to_test - 1 {
            candy_manager.set_new_minter_keypair(Keypair::new());
            airdrop(context, &candy_manager.minter.pubkey(), sol(10))
                .await
                .unwrap();
        }
    }
}

#[tokio::test]
async fn buy_limit_with_spl_token_allowlist() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let allowlist_price = 1;
    let price = 2;
    let limit_per_address = 3;
    let spl_token_allowlist_settings = SplTokenAllowlistConfig::new(BurnEveryTime);

    let mut candy_manager = CandyManagerBuilder::new()
        .set_collection(true)
        .set_spl_token_allowlist_config(spl_token_allowlist_settings.clone())
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Allowlist)
        .set_spl_token_allowlist_settings(SplTokenAllowlistConfig::to_candy_format(
            spl_token_allowlist_settings.clone(),
            &candy_manager.spl_token_allowlist_info.mint,
        ))
        .set_limit_per_address(limit_per_address)
        .set_allowlist_price(allowlist_price)
        .set_price(price)
        .build();

    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();
    candy_manager.set_collection(context).await.unwrap();

    candy_manager
        .mint_and_assert_successful(context, Some(allowlist_price), false, None)
        .await
        .unwrap();

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Public)
        .set_spl_token_allowlist_settings(SplTokenAllowlistConfig::to_candy_format(
            spl_token_allowlist_settings,
            &candy_manager.spl_token_allowlist_info.mint,
        ))
        .set_limit_per_address(limit_per_address)
        .set_allowlist_price(allowlist_price)
        .set_price(price)
        .build();

    candy_manager
        .update(context, None, candy_data)
        .await
        .unwrap();

    for _ in 0..limit_per_address {
        candy_manager
            .mint_and_assert_successful(context, Some(price), false, None)
            .await
            .unwrap();
    }

    candy_manager
        .mint_and_assert_failure(context, None, CandyError::BuyLimitPerAddressExceeded)
        .await;
}

#[tokio::test]
async fn buy_limit_with_merkle_allowlist() {
    let allowlist_config = get_allowlist_config_data();

    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_candy_machine(allowlist_config.candy_machine_keypair)
        .build(context)
        .await;

    let limit_per_address = 3;
    let allowlist_price = DEFAULT_PRICE / 2;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Allowlist)
        .set_allowlist_price(allowlist_price)
        .set_limit_per_address(limit_per_address)
        .build();
    candy_manager.create(context, candy_data).await.unwrap();
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

    let sol_to_airdrop = DEFAULT_PRICE * 5;
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

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Public)
        .set_allowlist_price(allowlist_price)
        .set_limit_per_address(limit_per_address)
        .build();
    let price = candy_data.price;

    candy_manager
        .update(context, None, candy_data)
        .await
        .unwrap();

    for _ in 0..limit_per_address {
        candy_manager
            .mint_and_assert_successful(context, Some(price), false, None)
            .await
            .unwrap();
    }

    candy_manager
        .mint_and_assert_failure(context, None, CandyError::BuyLimitPerAddressExceeded)
        .await;
}
