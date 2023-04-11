use std::{str::FromStr, sync::Arc};

use anchor_client::solana_sdk::{
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_program, sysvar,
};
use anchor_lang::prelude::AccountMeta;
use anchor_lang::Id;
use anchor_lang::ToAccountMetas;
use anchor_spl::associated_token::AssociatedToken;
use anyhow::Result;
use chrono::Utc;
use console::style;
use bullistic_candy_machine::{
    accounts as nft_accounts, cmp_pubkeys, instruction as nft_instruction, CandyError,
    CandyMachine, CollectionPda, SplTokenAllowlistMode,
};
use mpl_token_metadata::pda::find_collection_authority_account;
use solana_client::rpc_response::Response;
use spl_associated_token_account::get_associated_token_address;
use spl_token::{state::Account, ID as TOKEN_PROGRAM_ID};
use tokio::sync::Semaphore;

use crate::{
    cache::load_cache,
    candy_machine::{CANDY_MACHINE_ID, *},
    common::*,
    config::{Cluster, SugarConfig},
    pdas::*,
    utils::*,
};

pub struct MintArgs {
    pub keypair: Option<String>,
    pub rpc_url: Option<String>,
    pub cache: String,
    pub number: Option<u64>,
    pub receiver: Option<String>,
    pub candy_machine: Option<String>,
}

pub async fn process_mint(args: MintArgs) -> Result<()> {
    let sugar_config = sugar_setup(args.keypair, args.rpc_url)?;
    let client = Arc::new(setup_client(&sugar_config)?);

    // the candy machine id specified takes precedence over the one from the cache

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

    let candy_machine_state = Arc::new(get_candy_machine_state(&sugar_config, &candy_pubkey)?);

    let collection_pda_info =
        Arc::new(get_collection_pda(&candy_pubkey, &client.program(CANDY_MACHINE_ID)).ok());

    pb.finish_with_message("Done");

    println!(
        "\n{} {}Minting from candy machine",
        style("[2/2]").bold().dim(),
        CANDY_EMOJI
    );

    let receiver_pubkey = match args.receiver {
        Some(receiver_id) => Pubkey::from_str(&receiver_id)
            .map_err(|_| anyhow!("Failed to parse receiver pubkey: {}", receiver_id))?,
        None => sugar_config.keypair.pubkey(),
    };
    println!("\nMinting to {}", &receiver_pubkey);

    let number = args.number.unwrap_or(1);
    let available = candy_machine_state.data.items_available - candy_machine_state.items_redeemed;

    if number > available || number == 0 {
        let error = anyhow!("{} item(s) available, requested {}", available, number);
        error!("{:?}", error);
        return Err(error);
    }

    info!("Minting NFT from candy machine: {}", &candy_machine_id);
    info!("Candy machine program id: {:?}", CANDY_MACHINE_ID);

    if number == 1 {
        let pb = spinner_with_style();
        pb.set_message(format!(
            "{} item(s) remaining",
            candy_machine_state.data.items_available - candy_machine_state.items_redeemed
        ));
        let config = Arc::new(sugar_config);

        let result = match mint(
            Arc::clone(&config),
            candy_pubkey,
            Arc::clone(&candy_machine_state),
            Arc::clone(&collection_pda_info),
        )
        .await
        {
            Ok(signature) => format!("{} {}", style("Signature:").bold(), signature),
            Err(err) => {
                pb.abandon_with_message(format!("{}", style("Mint failed ").red().bold()));
                error!("{:?}", err);
                return Err(err);
            }
        };

        pb.finish_with_message(result);
    } else {
        let pb = progress_bar_with_style(number);

        let mut tasks = Vec::new();
        let semaphore = Arc::new(Semaphore::new(100));
        let config = Arc::new(sugar_config);

        for _i in 0..number {
            let config = config.clone();
            let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();
            let candy_machine_state = candy_machine_state.clone();
            let collection_pda_info = collection_pda_info.clone();
            let pb = pb.clone();

            // Start tasks
            tasks.push(tokio::spawn(async move {
                let _permit = permit;
                let res = mint(
                    config,
                    candy_pubkey,
                    candy_machine_state,
                    collection_pda_info,
                )
                .await;
                pb.inc(1);
                res
            }));
        }

        let mut error_count = 0;

        // Resolve tasks
        for task in tasks {
            let res = task.await.unwrap();
            if let Err(e) = res {
                error_count += 1;
                error!("{:?}, continuing. . .", e);
            }
        }

        if error_count > 0 {
            pb.abandon_with_message(format!(
                "{} {} items failed.",
                style("Some of the items failed to mint.").red().bold(),
                error_count
            ));
            return Err(anyhow!(
                "{} {}/{} {}",
                style("Minted").red().bold(),
                number - error_count,
                number,
                style("of the items").red().bold()
            ));
        }
        pb.finish();
    }

    Ok(())
}

pub async fn mint(
    config: Arc<SugarConfig>,
    candy_machine_id: Pubkey,
    candy_machine_state: Arc<CandyMachine>,
    collection_pda_info: Arc<Option<PdaInfo<CollectionPda>>>,
) -> Result<Signature> {
    let client = setup_client(&config)?;
    let program = client.program(CANDY_MACHINE_ID);
    let buyer = program.payer();
    let treasury_wallet = candy_machine_state.treasury_wallet;

    let candy_machine_data = &candy_machine_state.data;

    if candy_machine_state.items_redeemed >= candy_machine_data.items_available {
        return Err(anyhow!(CandyError::CandyMachineEmpty));
    }

    if candy_machine_state.bullistic_authority != buyer {
        // Apply validation for regular buyers.
        // TODO[@]: Make this more consistent with omni_mint_wallets and mint_phase logic.
        let now = Utc::now().timestamp();
        let mint_enabled = candy_machine_data.public_sale_start_time < now;

        if !mint_enabled {
            return Err(anyhow!(CandyError::CandyMachinePublicSaleNotLive));
        }

        if now > candy_machine_data.public_sale_end_time {
            return Err(anyhow!(CandyError::CandyMachinePublicSaleNotLive));
        }
    }

    let nft_mint = Keypair::new();
    let metaplex_program_id = Pubkey::from_str(METAPLEX_PROGRAM_ID)?;

    let mut additional_accounts: Vec<AccountMeta> = Vec::new();

    // Check SPL token allowlist settings
    if let Some(spl_token_allowlist_settings) = &candy_machine_data.spl_token_allowlist_settings {
        let allowlist_token_account =
            get_associated_token_address(&buyer, &spl_token_allowlist_settings.mint);

        additional_accounts.push(AccountMeta {
            pubkey: allowlist_token_account,
            is_signer: false,
            is_writable: true,
        });

        if spl_token_allowlist_settings.mode == SplTokenAllowlistMode::BurnEveryTime {
            let mut token_found = false;

            match program.rpc().get_account_data(&allowlist_token_account) {
                Ok(ata_data) => {
                    if !ata_data.is_empty() {
                        let account = Account::unpack_unchecked(&ata_data)?;

                        if account.amount > 0 {
                            additional_accounts.push(AccountMeta {
                                pubkey: spl_token_allowlist_settings.mint,
                                is_signer: false,
                                is_writable: true,
                            });

                            token_found = true;
                        }
                    }
                }
                Err(err) => {
                    error!("Invalid SPL token allowlist token account: {}", err);
                    return Err(anyhow!(
                        "Uninitialized SPL token allowlist account: {allowlist_token_account}.
                         Check that you provided a valid SPL token mint for the allowlist."
                    ));
                }
            }

            if !token_found {
                return Err(anyhow!(CandyError::NoSplAllowlistToken));
            }
        }
    }

    if let Some(treasury_mint) = candy_machine_state.treasury_mint {
        let user_token_account_info = get_associated_token_address(&buyer, &treasury_mint);

        additional_accounts.push(AccountMeta {
            pubkey: user_token_account_info,
            is_signer: false,
            is_writable: true,
        });
    }

    let metadata_pda = find_metadata_pda(&nft_mint.pubkey());
    let master_edition_pda = find_master_edition_pda(&nft_mint.pubkey());
    let (candy_machine_creator_pda, creator_bump) =
        find_candy_machine_creator_pda(&candy_machine_id);

    let bot_signer_authority = get_bot_signer_keypair();

    let (buyer_info_account, buyer_info_account_bump) =
        find_buyer_info_account_pda(&candy_machine_id, &buyer);

    let buyer_token_account = get_associated_token_address(&buyer, &nft_mint.pubkey());

    let mut accounts = bullistic_candy_machine::accounts::MintNFT {
        candy_machine: candy_machine_id,
        candy_machine_creator: candy_machine_creator_pda,
        buyer,
        treasury_wallet,
        metadata: metadata_pda,
        mint: nft_mint.pubkey(),
        creator_authority: candy_machine_state.creator_authority,
        master_edition: master_edition_pda,
        token_metadata_program: metaplex_program_id,
        token_program: TOKEN_PROGRAM_ID,
        system_program: system_program::id(),
        rent: sysvar::rent::ID,
        recent_slothashes: sysvar::slot_hashes::ID,
        instruction_sysvar_account: sysvar::instructions::ID,
        buyer_info_account,
        bot_signer_authority: bot_signer_authority.pubkey(),
        buyer_token_account,
        ata_program: AssociatedToken::id(),
    }
    .to_account_metas(None);

    let bot_signer_authority_should_sign = candy_machine_state.data.bot_protection_enabled;
    if bot_signer_authority_should_sign {
        for account in accounts.iter_mut() {
            if cmp_pubkeys(&account.pubkey, &get_bot_signer_keypair().pubkey()) {
                account.is_signer = true;
            }
        }
    }

    let mint_phase = CandyMachine::get_mint_phase(&candy_machine_state, Utc::now().timestamp());
    let mint_price = CandyMachine::get_mint_price(&candy_machine_state, &mint_phase);

    let mut mint_ix = program
        .request()
        .accounts(accounts)
        .args(nft_instruction::MintNft {
            creator_bump,
            buyer_info_account_bump,
            buyer_merkle_allowlist_proof_data: None,
            expected_price: mint_price,
        });

    // Add additional accounts directly to the mint instruction otherwise it won't work.
    if !additional_accounts.is_empty() {
        mint_ix = mint_ix.accounts(additional_accounts);
    }
    let mint_ix = mint_ix.instructions()?;

    let mut builder = program
        .request()
        .instruction(mint_ix[0].clone())
        .signer(&nft_mint);

    if bot_signer_authority_should_sign {
        builder = builder.signer(&bot_signer_authority);
    }

    if let Some((collection_pda_pubkey, collection_pda)) = collection_pda_info.as_ref() {
        let collection_authority_record =
            find_collection_authority_account(&collection_pda.mint, collection_pda_pubkey).0;
        builder = builder
            .accounts(nft_accounts::SetCollectionDuringMint {
                candy_machine: candy_machine_id,
                metadata: metadata_pda,
                buyer,
                creator_authority: candy_machine_state.creator_authority,
                collection_pda: *collection_pda_pubkey,
                token_metadata_program: mpl_token_metadata::ID,
                instruction_sysvar_account: sysvar::instructions::ID,
                collection_mint: collection_pda.mint,
                collection_metadata: find_metadata_pda(&collection_pda.mint),
                collection_master_edition: find_master_edition_pda(&collection_pda.mint),
                collection_authority_record,
            })
            .args(nft_instruction::SetCollectionDuringMint {});
    }

    let sig = builder.send()?;

    if let Err(_) | Ok(Response { value: None, .. }) = program
        .rpc()
        .get_account_with_commitment(&metadata_pda, CommitmentConfig::confirmed())
    {
        let cluster_param = match get_cluster(program.rpc()).unwrap_or(Cluster::Mainnet) {
            Cluster::Devnet => "?devnet",
            _ => "",
        };
        return Err(anyhow!(
            "Minting most likely failed with a bot tax. Check the transaction link for more details: https://explorer.solana.com/tx/{}{}",
            sig.to_string(),
            cluster_param,
        ));
    }

    info!("Minted! TxId: {}", sig);

    Ok(sig)
}
