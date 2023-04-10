use std::{str::FromStr, sync::Arc};

use anchor_lang::{prelude::Pubkey, InstructionData, ToAccountMetas};
use anyhow::{anyhow, Result};
use console::style;
use solana_program::instruction::Instruction;

use crate::{
    cache::load_cache,
    candy_machine::get_candy_machine_state,
    common::{setup_client, sugar_setup},
    merkle_allowlist::{chunk_root_list_for_update_txs, parse_merkle_allowlist_config},
};

#[derive(Debug)]
pub struct ProcessMerkleAllowlistArgs {
    pub cache: String,
    pub candy_machine: Option<String>,
    pub config: String,
    pub keypair: Option<String>,
    pub merkle_allowlist_config: String,
    pub rpc_url: Option<String>,
}

pub async fn process_merkle_allowlist(args: ProcessMerkleAllowlistArgs) -> Result<()> {
    let allowlist_config = match parse_merkle_allowlist_config(args.merkle_allowlist_config) {
        Ok(config) => config,
        Err(e) => return Err(e.into()),
    };

    let sugar_config = Arc::new(sugar_setup(args.keypair.clone(), args.rpc_url.clone())?);
    let client = setup_client(&sugar_config)?;
    let program = client.program(formfn_candy_machine::id());

    let candy_machine_id = match args.candy_machine {
        Some(candy_machine_id) => candy_machine_id,
        None => {
            let cache = load_cache(&args.cache, false)?;
            cache.program.candy_machine
        }
    };

    let candy_pubkey = match Pubkey::from_str(&candy_machine_id) {
        Ok(candy_pubkey) => candy_pubkey,
        Err(_) => {
            let error = anyhow!("Failed to parse candy machine id: {}", candy_machine_id);
            return Err(error);
        }
    };

    let candy_machine_state = Arc::new(get_candy_machine_state(&sugar_config, &candy_pubkey)?);

    let roots_to_add: Vec<[u8; 32]> = allowlist_config
        .merkle_allowlist_data
        .iter()
        .map(|config_data| config_data.root)
        .collect();
    let chunked_roots_to_add: Vec<Vec<[u8; 32]>> = chunk_root_list_for_update_txs(roots_to_add);

    let tx_count = chunked_roots_to_add.len();
    let allowlist_size = allowlist_config
        .merkle_allowlist_data
        .iter()
        .fold(0, |total, val| total + val.buyers.len());

    println!("\nReady to update candy machine merkle allowlist.\n",);
    println!("Candy machine address: {}", candy_pubkey);
    println!(
        "Candy machine creator: {}",
        candy_machine_state.formfn_authority
    );
    println!("Total allowlist addresses: {}", allowlist_size);
    println!("Total transactions required: {}", tx_count);
    println!("\nStarting allowlist update...\n");

    for roots_to_append in chunked_roots_to_add.iter() {
        let accounts = formfn_candy_machine::accounts::AppendMerkleAllowlistRoots {
            formfn_authority: candy_machine_state.formfn_authority,
            candy_machine: candy_pubkey,
        }
        .to_account_metas(None);

        let roots_to_append = roots_to_append.clone();
        let data =
            formfn_candy_machine::instruction::AppendMerkleAllowlistRoots { roots_to_append }
                .data();

        let append_allowlist_ix = Instruction {
            program_id: formfn_candy_machine::id(),
            data,
            accounts,
        };

        let builder = program.request().instruction(append_allowlist_ix);

        let sig = builder.send()?;

        println!("{} {}", style("Signature:").bold(), sig);
    }

    println!("\nSuccessfully updated merkle allowlist!",);

    Ok(())
}
