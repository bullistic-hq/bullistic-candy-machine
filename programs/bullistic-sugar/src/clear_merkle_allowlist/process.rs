use std::{str::FromStr, sync::Arc};

use anchor_lang::{prelude::Pubkey, InstructionData, ToAccountMetas};
use anyhow::{anyhow, Result};
use console::style;
use solana_program::instruction::Instruction;

use crate::{
    cache::load_cache,
    candy_machine::get_candy_machine_state,
    common::{setup_client, sugar_setup},
};

#[derive(Debug)]
pub struct ClearMerkleAllowlistArgs {
    pub cache: String,
    pub candy_machine: Option<String>,
    pub config: String,
    pub keypair: Option<String>,
    pub rpc_url: Option<String>,
}

pub async fn clear_merkle_allowlist(args: ClearMerkleAllowlistArgs) -> Result<()> {
    let sugar_config = Arc::new(sugar_setup(args.keypair.clone(), args.rpc_url.clone())?);
    let client = setup_client(&sugar_config)?;
    let program = client.program(bullistic_candy_machine::id());

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

    let number_of_allowlist_merkle_roots =
        candy_machine_state.data.merkle_allowlist_root_list.len();

    let accounts = bullistic_candy_machine::accounts::ClearMerkleAllowlistRoots {
        bullistic_authority: candy_machine_state.bullistic_authority,
        candy_machine: candy_pubkey,
    }
    .to_account_metas(None);

    let data = bullistic_candy_machine::instruction::ClearMerkleAllowlistRoots {}.data();

    let clear_ix = Instruction {
        program_id: bullistic_candy_machine::id(),
        data,
        accounts,
    };

    let builder = program.request().instruction(clear_ix);

    let sig = builder.send()?;

    println!("{} {}", style("Signature:").bold(), sig);

    println!(
        "\nSuccessfully cleared allowlist, {} merkle roots removed.",
        number_of_allowlist_merkle_roots
    );

    Ok(())
}
