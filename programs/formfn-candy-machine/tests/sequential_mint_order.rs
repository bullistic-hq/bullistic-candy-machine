#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::{
    core::helpers::{airdrop, clone_keypair, strip_empty_bytes_from_string},
    utils::{
        candy_machine_program_test,
        helpers::{get_config_line_name, sol},
        CandyConfigBuilder, CandyManagerBuilder,
    },
};
use formfn_candy_machine::MintPhase;

mod core;
mod utils;

#[tokio::test]
async fn creator_authority_can_pre_mint_in_sequential_order() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;

    let mut candy_manager = CandyManagerBuilder::new().build(context).await;
    candy_manager.set_new_minter_keypair(clone_keypair(&candy_manager.creator_authority));

    let nft_total_supply_for_mint = 10;
    let price_in_sol = sol(1);
    let tx_fee_buffer_sol = sol(5);
    let total_airdrop_sol = nft_total_supply_for_mint * price_in_sol + tx_fee_buffer_sol;
    airdrop(context, &candy_manager.minter.pubkey(), total_airdrop_sol)
        .await
        .unwrap();

    let mut candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_sequential_mint_order_enabled(true)
        .enable_mint_phase(MintPhase::Premint)
        .build();

    candy_data.items_available = nft_total_supply_for_mint;
    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    let candy_start = candy_manager.get_candy(context).await;
    assert_eq!(
        candy_start.items_redeemed, 0,
        "Should start with 0 items_redeemed"
    );

    for i in 0..nft_total_supply_for_mint {
        let nft = candy_manager
            .mint_and_assert_successful(context, Some(price_in_sol), true, None)
            .await
            .unwrap();

        let metadata = nft.get_metadata(context).await;
        let name = strip_empty_bytes_from_string(metadata.data.name);
        let expected = get_config_line_name(i as u32);

        assert_eq!(
            name, expected,
            "Name should match expected name, received: {}, expected: {}",
            name, expected
        );
    }
}
