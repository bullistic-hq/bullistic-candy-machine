use std::str::FromStr;

use anchor_client::solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use console::style;
use bullistic_candy_machine::{accounts as nft_accounts, instruction as nft_instruction};
use mpl_token_metadata::{pda::find_collection_authority_account, state::Metadata};

use crate::{
    cache::load_cache,
    candy_machine::{CANDY_MACHINE_ID, *},
    common::*,
    config::get_config_data,
    hash::hash_and_update,
    pdas::*,
    update::{process_update, UpdateArgs},
    utils::{assert_correct_authority, spinner_with_style},
};

pub struct RemoveCollectionArgs {
    pub keypair: Option<String>,
    pub rpc_url: Option<String>,
    pub cache: String,
    pub config: String,
    pub candy_machine: Option<String>,
}

pub fn process_remove_collection(args: RemoveCollectionArgs) -> Result<()> {
    let sugar_config = sugar_setup(args.keypair.clone(), args.rpc_url.clone())?;
    let client = setup_client(&sugar_config)?;
    let program = client.program(CANDY_MACHINE_ID);
    let mut cache = Cache::new();

    // the candy machine id specified takes precedence over the one from the cache
    let candy_machine_id = match args.candy_machine {
        Some(ref candy_machine_id) => candy_machine_id.to_owned(),
        None => {
            cache = load_cache(&args.cache, false)?;
            cache.program.candy_machine.clone()
        }
    };

    let candy_pubkey = match Pubkey::from_str(&candy_machine_id) {
        Ok(candy_pubkey) => candy_pubkey,
        Err(_) => {
            let error = anyhow!("Failed to parse candy machine id: {}", candy_machine_id);
            error!("{:?}", error);
            return Err(error);
        }
    };

    println!(
        "{} {}Loading candy machine",
        style("[1/2]").bold().dim(),
        LOOKING_GLASS_EMOJI
    );
    println!("{} {}", style("Candy machine ID:").bold(), candy_machine_id);

    let pb = spinner_with_style();
    pb.set_message("Connecting...");

    let candy_machine_state = get_candy_machine_state(&sugar_config, &candy_pubkey)?;
    let (collection_pda_pubkey, collection_pda) = get_collection_pda(&candy_pubkey, &program)?;
    let collection_mint_pubkey = collection_pda.mint;
    let collection_metadata_info = get_metadata_pda(&collection_mint_pubkey, &program)?;

    pb.finish_with_message("Done");

    assert_correct_authority(
        &sugar_config.keypair.pubkey(),
        &candy_machine_state.bullistic_authority,
    )?;

    println!(
        "\n{} {}Removing collection mint for candy machine",
        style("[2/2]").bold().dim(),
        CANDY_EMOJI
    );

    let pb = spinner_with_style();
    pb.set_message("Sending remove collection transaction...");

    let remove_signature = remove_collection(
        &program,
        &candy_pubkey,
        &candy_machine_state,
        &collection_pda_pubkey,
        &collection_mint_pubkey,
        &collection_metadata_info,
    )?;

    pb.finish_with_message(format!(
        "{} {}",
        style("Remove collection signature:").bold(),
        remove_signature
    ));

    // If a candy machine id wasn't manually specified we are operating on the candy machine in the cache
    // and so need to update the cache file.
    if args.candy_machine.is_none() {
        cache.items.shift_remove("-1");
        cache.program.collection_mint = String::new();
        cache.sync_file()?;

        // If hidden settings are enabled, we update the hash value in the config file and update the candy machine on-chain.
        if candy_machine_state.data.hidden_settings.is_some() {
            let mut config_data = get_config_data(&args.config)?;
            let hidden_settings = config_data.hidden_settings.as_ref().unwrap().clone();

            println!(
                "\n{} {}",
                style("Hidden settings hash:").bold(),
                hash_and_update(hidden_settings, &args.config, &mut config_data, &args.cache,)?
            );

            println!(
                "\nCandy machine has hidden settings and cache file was updated. Updating hash value...\n"
            );

            let update_args = UpdateArgs {
                keypair: args.keypair,
                rpc_url: args.rpc_url,
                new_authority: None,
                config: args.config,
                candy_machine: Some(candy_machine_id),
            };

            process_update(update_args)?;
        }
    }

    Ok(())
}

pub fn remove_collection(
    program: &Program,
    candy_pubkey: &Pubkey,
    candy_machine_state: &CandyMachine,
    collection_pda_pubkey: &Pubkey,
    collection_mint_pubkey: &Pubkey,
    collection_metadata_info: &PdaInfo<Metadata>,
) -> Result<Signature> {
    let payer = program.payer();

    let collection_authority_record =
        find_collection_authority_account(collection_mint_pubkey, collection_pda_pubkey).0;

    let (collection_metadata_pubkey, collection_metadata) = collection_metadata_info;

    if collection_metadata.update_authority != payer {
        return Err(anyhow!(CustomCandyError::AuthorityMismatch(
            collection_metadata.update_authority.to_string(),
            payer.to_string()
        )));
    }

    if candy_machine_state.items_redeemed > 0 {
        return Err(anyhow!(
            "You can't modify the Candy Machine collection after items have been minted."
        ));
    }

    let builder = program
        .request()
        .accounts(nft_accounts::RemoveCollection {
            candy_machine: *candy_pubkey,
            bullistic_authority: payer,
            collection_pda: *collection_pda_pubkey,
            metadata: *collection_metadata_pubkey,
            mint: *collection_mint_pubkey,
            collection_authority_record,
            token_metadata_program: mpl_token_metadata::ID,
        })
        .args(nft_instruction::RemoveCollection);

    let sig = builder.send()?;

    Ok(sig)
}
