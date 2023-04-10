use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use formfn_candy_machine::constants::MAX_ROOT_NUMBER_PER_APPEND_MERKLE_ALLOWLIST_TX;

use super::MerkleAllowlistError;

pub type MerkleRoot = [u8; 32];
pub type MerkleProof = [u8; 32];

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MerkleAllowlistConfig {
    pub candy_machine_keypair: KeypairStruct,
    pub merkle_allowlist_data: Vec<MerkleAllowlistConfigData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MerkleAllowlistConfigData {
    pub buyers: Vec<MerkleAllowlistBuyer>,
    pub root: MerkleRoot,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MerkleAllowlistBuyer {
    pub address: String,
    pub amount: u16,
    pub proof: Vec<MerkleProof>,
    pub merkle_tree_index: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeypairStruct {
    pub public_key: String,
    pub secret_key: Vec<u8>,
}

pub fn parse_merkle_allowlist_config(
    merkle_allowlist_config_path: String,
) -> Result<MerkleAllowlistConfig, MerkleAllowlistError> {
    let file = File::open(&merkle_allowlist_config_path);
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            let read_result = file.read_to_string(&mut contents);
            if let Err(e) = read_result {
                return Err(MerkleAllowlistError::ParseConfigError(e.to_string()));
            }

            let parsed_config = serde_json::from_str(&contents);
            match parsed_config {
                Ok(config) => Ok(config),
                Err(e) => Err(MerkleAllowlistError::ParseConfigError(e.to_string())),
            }
        }
        Err(_) => Err(MerkleAllowlistError::MissingFileError(
            merkle_allowlist_config_path,
        )),
    }
}

/**
 * Chunk the total roots list into smaller chunks. The chunk size is limited by
 * how many roots can find in a single transaction.
 */
pub fn chunk_root_list_for_update_txs(roots_to_append: Vec<MerkleRoot>) -> Vec<Vec<MerkleRoot>> {
    let chunked_roots_to_add: Vec<Vec<MerkleRoot>> = roots_to_append
        .chunks(MAX_ROOT_NUMBER_PER_APPEND_MERKLE_ALLOWLIST_TX)
        .map(|x| x.to_vec())
        .collect();

    chunked_roots_to_add
}
