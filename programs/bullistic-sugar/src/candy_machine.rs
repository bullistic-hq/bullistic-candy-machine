use anchor_client::{solana_sdk::pubkey::Pubkey, Client, ClientError};
use anyhow::{anyhow, Result};
pub use bullistic_candy_machine::ID as CANDY_MACHINE_ID;
use bullistic_candy_machine::{
    CandyMachine, CandyMachineData, SplTokenAllowlistMode, SplTokenAllowlistSettings,
};
use spl_token::id as token_program_id;

use crate::{
    common::FloatConversionError,
    config::{data::SugarConfig, price_as_lamports, ConfigData},
    setup::setup_client,
    utils::{check_spl_token, f64_to_u64_safe},
};

// To test a custom candy machine program, comment the bullistic_candy_machine::ID line
// above and use the following lines to declare the id to use:
//
//use solana_program::declare_id;
//declare_id!("<YOUR CANDY MACHINE ID>");
//pub use self::ID as CANDY_MACHINE_ID;

#[derive(Debug)]
pub struct ConfigStatus {
    pub index: u32,
    pub on_chain: bool,
}

pub fn parse_config_price(
    client: &Client,
    config: &ConfigData,
    price_to_parse: f64,
) -> Result<u64> {
    let parsed_price = if let Some(spl_token) = config.spl_token {
        let token_program = client.program(token_program_id());
        let token_mint = check_spl_token(&token_program, &spl_token.to_string())?;

        let mutliplier = 10f64.powf(token_mint.decimals as f64);
        let config_price_base_units = price_to_parse * mutliplier;

        match f64_to_u64_safe(config_price_base_units) {
            Ok(price) => price,
            Err(e) => {
                match e {
                    FloatConversionError::Fractional => {
                        return Err(anyhow!(
                            "Can't convert price to u64: price may have more decimals than SPL token. Price: {}, decimals: {}.",
                            price_to_parse,
                            token_mint.decimals
                        ))
                    },
                    FloatConversionError::Overflow => {
                        return Err(anyhow!(
                            "Can't convert price to u64 because of overflow: price is too large or too many SPL token decimals. Price: {}, decimals: {}.",
                            price_to_parse,
                            token_mint.decimals
                        ))
                    },
                }
            }
        }
    } else {
        price_as_lamports(price_to_parse)
    };

    Ok(parsed_price)
}

pub fn get_candy_machine_state(
    sugar_config: &SugarConfig,
    candy_machine_id: &Pubkey,
) -> Result<CandyMachine> {
    let client = setup_client(sugar_config)?;
    let program = client.program(CANDY_MACHINE_ID);

    program.account(*candy_machine_id).map_err(|e| match e {
        ClientError::AccountNotFound => anyhow!("Candy Machine does not exist!"),
        _ => anyhow!(
            "Failed to deserialize Candy Machine account {}: {}",
            candy_machine_id.to_string(),
            e
        ),
    })
}

pub fn get_candy_machine_data(
    sugar_config: &SugarConfig,
    candy_machine_id: &Pubkey,
) -> Result<CandyMachineData> {
    let candy_machine = get_candy_machine_state(sugar_config, candy_machine_id)?;
    Ok(candy_machine.data)
}

pub fn print_candy_machine_state(state: CandyMachine) {
    println!("Authority {:?}", state.bullistic_authority);
    println!("Wallet {:?}", state.treasury_wallet);
    println!("Token mint: {:?}", state.treasury_mint);
    println!("Items redeemed: {:?}", state.items_redeemed);
    print_candy_machine_data(&state.data);
}

pub fn print_candy_machine_data(data: &CandyMachineData) {
    println!("Uuid: {:?}", data.uuid);
    println!("Price: {:?}", data.price);
    println!("Symbol: {:?}", data.symbol);
    println!(
        "Seller fee basis points: {:?}",
        data.seller_fee_basis_points
    );
    println!("Max supply: {:?}", data.max_supply);
    println!("Is mutable: {:?}", data.is_mutable);
    println!("Go live date: {:?}", data.public_sale_start_time);
    println!("Items available: {:?}", data.items_available);

    print_spl_token_allowlist_settings(&data.spl_token_allowlist_settings);
}

fn print_spl_token_allowlist_settings(settings: &Option<SplTokenAllowlistSettings>) {
    if let Some(settings) = settings {
        match settings.mode {
            SplTokenAllowlistMode::BurnEveryTime => println!("Mode: Burn every time"),
            SplTokenAllowlistMode::NeverBurn => println!("Mode: Never burn"),
        }
        println!("Mint: {:?}", settings.mint);
    } else {
        println!("No SPL token allowlist mint settings");
    }
}
