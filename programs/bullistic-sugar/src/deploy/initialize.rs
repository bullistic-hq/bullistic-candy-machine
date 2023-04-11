use anchor_client::solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_instruction, system_program, sysvar,
};
use anchor_lang::prelude::AccountMeta;
use anyhow::Result;
use chrono::{Duration, Utc};
use bullistic_candy_machine::{
    accounts as nft_accounts, get_space_for_candy, instruction as nft_instruction,
    CandyMachineData, Creator as CandyCreator,
};
pub use mpl_token_metadata::state::{
    MAX_CREATOR_LIMIT, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH,
};
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::{candy_machine::parse_config_price, common::*, config::data::*, deploy::errors::*};

/// Create the candy machine data struct.
pub fn create_candy_machine_data(
    client: &Client,
    config: &ConfigData,
    uuid: String,
) -> Result<CandyMachineData> {
    let public_sale_start_time: i64 = config_time_as_timestamp(&config.public_sale_start_time)?;
    let allowlist_sale_start_time: Option<i64> =
        config_time_opt_as_timestamp(&config.allowlist_sale_start_time)?;
    let public_sale_end_time: Option<i64> =
        config_time_opt_as_timestamp(&config.public_sale_end_time)?;

    let public_sale_end_time = if let Some(end_time) = public_sale_end_time {
        end_time
    } else {
        Utc::now().timestamp() + Duration::days(180).num_seconds()
    };
    let spl_token_allowlist_settings = config
        .spl_token_allowlist_settings
        .as_ref()
        .map(|s| s.to_candy_format());

    let hidden_settings = config.hidden_settings.as_ref().map(|s| s.to_candy_format());

    let mut creators: Vec<CandyCreator> = Vec::new();
    let mut share = 0u32;

    for creator in &config.creators {
        let c = creator.to_candy_format()?;
        share += c.share as u32;

        creators.push(c);
    }

    if creators.is_empty() || creators.len() > MAX_CREATOR_LIMIT {
        return Err(anyhow!(
            "The number of creators must be between 1 and {}.",
            MAX_CREATOR_LIMIT,
        ));
    }

    if share != 100 {
        return Err(anyhow!(
            "Creator(s) share must add up to 100, current total {}.",
            share,
        ));
    }

    let price = parse_config_price(client, config, config.price)?;
    let premint_price = match config.premint_price {
        Some(price) => Some(parse_config_price(client, config, price)?),
        None => None,
    };
    let allowlist_price = match config.allowlist_price {
        Some(price) => Some(parse_config_price(client, config, price)?),
        None => None,
    };

    let omni_mint_wallets: Vec<Pubkey> = creators.iter().map(|creator| creator.address).collect();

    let data = CandyMachineData {
        uuid,
        price,
        premint_price,
        allowlist_price,
        symbol: config.symbol.clone(),
        seller_fee_basis_points: config.seller_fee_basis_points,
        max_supply: 0,
        is_mutable: config.is_mutable,
        public_sale_start_time,
        public_sale_end_time,
        creators,
        omni_mint_wallets,
        spl_token_allowlist_settings,
        hidden_settings,
        items_available: config.number,
        limit_per_address: config.limit_per_address,
        bot_protection_enabled: config.bot_protection_enabled,
        sequential_mint_order_enabled: config.sequential_mint_order_enabled,
        merkle_allowlist_root_list: Vec::new(),
        allowlist_sale_start_time,
    };

    Ok(data)
}

/// Send the `initialize_candy_machine` instruction to the candy machine program.
pub fn initialize_candy_machine(
    config_data: &ConfigData,
    candy_account: &Keypair,
    candy_machine_data: CandyMachineData,
    treasury_wallet: Pubkey,
    program: Program,
) -> Result<Signature> {
    let payer = program.payer();

    let candy_account_size = get_space_for_candy(candy_machine_data.clone())?;

    info!(
        "Initializing candy machine with account size of: {} and address of: {}",
        candy_account_size,
        candy_account.pubkey().to_string()
    );

    let lamports = program
        .rpc()
        .get_minimum_balance_for_rent_exemption(candy_account_size)?;

    let balance = program.rpc().get_account(&payer)?.lamports;

    if lamports > balance {
        return Err(DeployError::BalanceTooLow(
            format!("{:.3}", (balance as f64 / LAMPORTS_PER_SOL as f64)),
            format!("{:.3}", (lamports as f64 / LAMPORTS_PER_SOL as f64)),
        )
        .into());
    }

    let mut tx = program
        .request()
        .instruction(system_instruction::create_account(
            &payer,
            &candy_account.pubkey(),
            lamports,
            candy_account_size as u64,
            &program.id(),
        ))
        .signer(candy_account)
        .accounts(nft_accounts::InitializeCandyMachine {
            candy_machine: candy_account.pubkey(),
            treasury_wallet,
            bullistic_authority: payer,
            creator_authority: config_data.creator_authority,
            payer,
            system_program: system_program::id(),
            rent: sysvar::rent::ID,
        })
        .args(nft_instruction::InitializeCandyMachine {
            data: candy_machine_data,
        });

    if let Some(token) = config_data.spl_token {
        tx = tx.accounts(AccountMeta {
            pubkey: token,
            is_signer: false,
            is_writable: false,
        });
    }

    match tx.send() {
        Ok(sig) => Ok(sig),
        Err(e) => {
            println!("\nAn error occurred creating the candy machine: {:?}", e);
            Err(anyhow!(e))
        }
    }
}
