#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use bullistic_candy_machine::CandyError;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::core::helpers::get_balance;
use crate::utils::{
    candy_machine_program_test,
    helpers::{lamports, sol},
    CandyConfigBuilder, CandyManagerBuilder,
};

mod core;
mod utils;

#[tokio::test]
async fn mint_many() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let nft_total_supply_for_mint = 25;
    let price_in_sol = sol(1);
    let tx_fee_buffer_sol = sol(50);
    let total_airdrop_sol = nft_total_supply_for_mint * price_in_sol + tx_fee_buffer_sol;

    let mut candy_manager = CandyManagerBuilder::new()
        .set_sol_airdrop_size_for_minter(lamports(total_airdrop_sol))
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_items_available(nft_total_supply_for_mint)
        .build();
    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    let candy_start = candy_manager.get_candy(context).await;
    assert_eq!(
        candy_start.items_redeemed, 0,
        "Should start with 0 items_redeemed"
    );

    let pre_balance = get_balance(context, &candy_manager.bullistic_authority.pubkey()).await;

    for _ in 0..nft_total_supply_for_mint {
        let nft = candy_manager
            .mint_and_assert_successful(context, Some(price_in_sol), true, None)
            .await
            .unwrap();
        println!(
            "NFT mint = {:?}, edition = {:?}",
            nft.mint.to_base58_string(),
            nft.edition_pubkey
        );
    }

    candy_manager
        .mint_and_assert_failure(context, None, CandyError::CandyMachineEmpty)
        .await;

    let candy_end = candy_manager.get_candy(context).await;
    assert_eq!(
        candy_start.items_redeemed + candy_end.items_redeemed,
        nft_total_supply_for_mint,
        "Total redeemed should equal all available nfts"
    );

    let final_post_balance = get_balance(context, &candy_manager.bullistic_authority.pubkey()).await;
    let balance_diff = final_post_balance - pre_balance;
    let total_sol_spend = price_in_sol * nft_total_supply_for_mint;
    assert!(balance_diff >= total_sol_spend);

    println!(
        "balance_diff = {}, total_sol_spend = {}, total_redeemed = {}",
        balance_diff, total_sol_spend, candy_end.items_redeemed
    );
}
