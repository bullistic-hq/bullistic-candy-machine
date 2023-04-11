use anchor_lang::prelude::ERROR_CODE_OFFSET;
use arrayref::array_ref;
use chrono::Utc;
use console::style;
use enum_index::EnumIndex;
use bullistic_candy_machine::constants::{CONFIG_ARRAY_START, CONFIG_LINE_SIZE};
use bullistic_candy_machine::{CandyError, CandyMachine};
use mpl_token_metadata::state::{MAX_NAME_LENGTH, MAX_URI_LENGTH};
use solana_program::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use solana_sdk::account::Account;
use solana_sdk::instruction::InstructionError;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::TransactionError;
use solana_sdk::transport::TransportError;
use spl_associated_token_account::get_associated_token_address;
use std::fmt::Debug;

use crate::utils::constants::BOT_SIGNER_AUTHORITY_SECRET;
use crate::utils::{FreezeInfo, SolanaProgramTestError, SolanaProgramTestResult, TokenInfo};
use bullistic_candy_machine::{
    constants::{BULLISTIC_CANDY_MACHINE_ERROR_OFFSET, PREFIX as CANDY_PREFIX},
    state::BUYER_INFO_ACCOUNT_PREFIX,
    ConfigLine,
};

pub fn get_config_line_name(index: u32) -> String {
    format!("Item #{}", index)
}

pub fn get_config_line_uri(index: u32) -> String {
    format!("Item #{} URI", index)
}

pub fn make_config_lines(start_index: u32, total: u8) -> Vec<ConfigLine> {
    let mut config_lines = Vec::with_capacity(total as usize);
    for i in 0..total {
        config_lines.push(ConfigLine {
            name: get_config_line_name(i as u32 + start_index),
            uri: get_config_line_uri(i as u32 + start_index),
        })
    }
    config_lines
}

pub fn find_candy_creator(candy_machine_key: &Pubkey) -> (Pubkey, u8) {
    let seeds = &[CANDY_PREFIX.as_bytes(), candy_machine_key.as_ref()];
    Pubkey::find_program_address(seeds, &bullistic_candy_machine::id())
}

pub fn find_collection_pda(candy_machine_key: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"collection".as_ref(), candy_machine_key.as_ref()],
        &bullistic_candy_machine::id(),
    )
}

pub fn find_buyer_info_account_pda(candy_machine: &Pubkey, buyer: &Pubkey) -> (Pubkey, u8) {
    let seeds = &[
        BUYER_INFO_ACCOUNT_PREFIX.as_bytes(),
        candy_machine.as_ref(),
        buyer.as_ref(),
    ];
    Pubkey::find_program_address(seeds, &bullistic_candy_machine::id())
}

pub fn find_freeze_ata(freeze_info: &FreezeInfo, token_info: &TokenInfo) -> Pubkey {
    get_associated_token_address(&freeze_info.pda, &token_info.mint)
}

pub fn sol(amount_in_sol: u64) -> u64 {
    amount_in_sol * LAMPORTS_PER_SOL
}

pub fn lamports(amount_in_lamports: u64) -> u64 {
    amount_in_lamports / LAMPORTS_PER_SOL
}

pub struct CandyTestLogger {
    test_name: String,
}

impl CandyTestLogger {
    pub fn new(test_name: &str) -> Self {
        Self {
            test_name: test_name.to_string(),
        }
    }

    pub fn new_start(test_name: &str) -> Self {
        let new = Self {
            test_name: test_name.to_string(),
        };
        new.start();
        new
    }

    pub fn start(&self) {
        println!(
            "{}",
            style(format!("\n{} start.", self.test_name)).bold().cyan()
        )
    }

    pub fn end(&self) {
        println!(
            "{}",
            style(format!("{} finished!\n", self.test_name))
                .bold()
                .green()
        )
    }
}

pub fn test_start(input: &str) {
    println!("\n{}", style(input).magenta().bold().underlined());
}

pub fn get_bot_signer_keypair() -> Keypair {
    Keypair::from_bytes(BOT_SIGNER_AUTHORITY_SECRET).unwrap()
}

pub fn get_current_unix_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub struct ParsedConfigLinesResult {
    pub config_lines: Vec<ConfigLine>,
    pub config_line_count_number: u32,
}

pub fn parse_candy_machine_config_lines(
    candy_machine_state: CandyMachine,
    candy_machine_account: Account,
) -> ParsedConfigLinesResult {
    let config_lines_start = CONFIG_ARRAY_START + 4;
    let config_lines_length = CONFIG_LINE_SIZE * candy_machine_state.data.items_available as usize;

    let config_line_data_slice: &[u8] =
        &candy_machine_account.data[config_lines_start..config_lines_start + config_lines_length];

    let config_line_count_number = u32::from_le_bytes(*array_ref![
        &candy_machine_account.data,
        CONFIG_ARRAY_START,
        4
    ]);

    let config_line_count = config_line_count_number as usize;

    let mut config_lines: Vec<ConfigLine> = Vec::with_capacity(config_line_count);

    for n in 0..config_line_count {
        let index_offset = if n == 0 { 0 } else { n * CONFIG_LINE_SIZE };

        let mut name_vec = Vec::with_capacity(MAX_NAME_LENGTH);
        let mut uri_vec = Vec::with_capacity(MAX_URI_LENGTH);

        for i in 4..4 + MAX_NAME_LENGTH {
            let index = i + index_offset;
            if config_line_data_slice[index] == 0 {
                break;
            }
            name_vec.push(config_line_data_slice[index])
        }

        for i in 8 + MAX_NAME_LENGTH..8 + MAX_NAME_LENGTH + MAX_URI_LENGTH {
            let index = i + index_offset;
            if config_line_data_slice[index] == 0 {
                break;
            }
            uri_vec.push(config_line_data_slice[index])
        }

        let config_line: ConfigLine = ConfigLine {
            name: String::from_utf8(name_vec).unwrap(),
            uri: String::from_utf8(uri_vec).unwrap(),
        };

        config_lines.push(config_line);
    }

    ParsedConfigLinesResult {
        config_lines,
        config_line_count_number,
    }
}

pub fn get_anchor_error_code_number_from_candy_error(candy_error: CandyError) -> u32 {
    let expected_candy_machine_error_code = (candy_error.enum_index()) as u32;
    expected_candy_machine_error_code + ERROR_CODE_OFFSET + BULLISTIC_CANDY_MACHINE_ERROR_OFFSET
}

pub fn assert_tx_failed_with_error_code<T: Debug>(
    tx_result: SolanaProgramTestResult<T>,
    expected_candy_error: CandyError,
) -> () {
    if let SolanaProgramTestError::TransportError(transport_error) = tx_result.unwrap_err() {
        let expected_candy_machine_error_code =
            get_anchor_error_code_number_from_candy_error(expected_candy_error);
        let tx_error_code = match transport_error {
            TransportError::TransactionError(TransactionError::InstructionError(
                _,
                InstructionError::Custom(err_num),
            )) => err_num,
            _ => 0,
        };
        assert_eq!(
            tx_error_code, expected_candy_machine_error_code,
            "Expected transaction error code to match candy error: {}",
            expected_candy_machine_error_code
        );
    }
}
