#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use solana_program_test::*;
use solana_sdk::signature::Keypair;

use crate::utils::{
    candy_machine_program_test, helpers::sol, CandyConfigBuilder, CandyManagerBuilder,
};

mod core;
mod utils;

#[tokio::test]
async fn bot_protection_enabled_requires_bot_signer() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_bot_protection_enabled(true)
        .build(context)
        .await;

    let price_in_sol = sol(1);

    let bot_protection_enabled = true;
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_bot_protection_enabled(bot_protection_enabled)
        .build();

    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    let candy_start = candy_manager.get_candy(context).await;
    assert_eq!(
        candy_start.data.bot_protection_enabled, bot_protection_enabled,
        "bot_protection_enabled should match provided value: {}",
        bot_protection_enabled
    );

    candy_manager
        .mint_and_assert_successful(context, Some(price_in_sol), true, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn bot_protection_enabled_enforces_bot_taxes() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_bot_protection_enabled(true)
        .build(context)
        .await;

    let bot_protection_enabled = true;
    let candy_data = CandyConfigBuilder::new(&candy_manager)
        .set_bot_protection_enabled(bot_protection_enabled)
        .build();

    candy_manager
        .create(context, candy_data.clone())
        .await
        .unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();

    let candy_start = candy_manager.get_candy(context).await;
    assert_eq!(
        candy_start.data.bot_protection_enabled, bot_protection_enabled,
        "bot_protection_enabled should match provided value: {}",
        bot_protection_enabled
    );

    let bot_tax_result = candy_manager
        .mint_and_assert_bot_tax(context, Some(false), None)
        .await;
    assert!(bot_tax_result.is_ok(), "bot_tax_result should be ok");
}
