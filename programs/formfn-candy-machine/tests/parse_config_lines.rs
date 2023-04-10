#![cfg(feature = "test-bpf")]
#![allow(dead_code)]

use solana_program_test::*;
use solana_sdk::signature::Keypair;

use helpers::{get_config_line_name, get_config_line_uri, ParsedConfigLinesResult};
use utils::CandyConfigBuilder;

use crate::utils::helpers;
use crate::utils::{candy_machine_program_test, CandyManagerBuilder};

mod core;
mod utils;

#[tokio::test]
async fn parse_config_lines() {
    let mut context = candy_machine_program_test().start_with_context().await;
    let context = &mut context;
    let mut candy_manager = CandyManagerBuilder::new()
        .set_collection(true)
        .build(context)
        .await;

    let candy_data = CandyConfigBuilder::default(&candy_manager);

    candy_manager.create(context, candy_data).await.unwrap();
    candy_manager.fill_config_lines(context).await.unwrap();
    candy_manager.set_collection(context).await.unwrap();

    let ParsedConfigLinesResult {
        config_line_count_number,
        config_lines,
    } = candy_manager.parse_config_lines(context).await;

    let candy_machine_state = candy_manager.get_candy(context).await;
    assert_eq!(
        config_line_count_number as u64, candy_machine_state.data.items_available,
        "Config line count should match candy machines items available."
    );

    for i in 0..config_line_count_number {
        let config_line = &config_lines[i as usize];
        assert_eq!(config_line.name, get_config_line_name(i));
        assert_eq!(config_line.uri, get_config_line_uri(i));
    }
}
