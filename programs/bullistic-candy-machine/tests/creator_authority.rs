#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};

use bullistic_candy_machine::MintPhase;
use utils::CandyConfigBuilder;

use crate::utils::{candy_machine_program_test, CandyManagerBuilder};

mod core;
mod utils;

#[tokio::test]
async fn mint_update_authority_should_be_creator_authority() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_collection(true)
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .enable_mint_phase(MintPhase::Public)
        .build();

    let price = candy_data.price;

    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();
    candy_manager.set_collection(context).await.unwrap();

    let nft = candy_manager
        .mint_and_assert_successful(context, Some(price), false, None)
        .await
        .unwrap();

    let nft_metadata = nft.get_metadata(context).await;
    let creator_authority = candy_manager.creator_authority;
    assert_eq!(
        nft_metadata.update_authority,
        creator_authority.pubkey(),
        "NFT update_authority should match the candy machine creator_authority."
    );

    let nft_creators = nft_metadata.data.creators.unwrap();
    assert!(
        nft_creators.len() == 1,
        "NFT metadata creators length should be 1."
    );
    assert_eq!(
        nft_creators.first().unwrap().address,
        creator_authority.pubkey(),
        "NFT metadata creator should be the creator_authority."
    );
}
