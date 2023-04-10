use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use solana_sdk::signature::Keypair;

use formfn_candy_machine::BuyerMerkleAllowlistProofData;

use super::constants::{
    MAX_ROOT_NUMBER_PER_APPEND_MERKLE_ALLOWLIST_TX_FOR_TEST, TEST_CONFIG_FILE_PATH,
};

pub type MerkleTreeNode = [u8; 32];
pub type MerkleRoot = MerkleTreeNode;
pub type MerkleProof = MerkleTreeNode;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MerkleAllowlistTestConfig {
    pub candy_machine_keypair: KeypairStruct,
    pub merkle_allowlist_data: Vec<MerkleAllowlistTestConfigData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MerkleAllowlistTestConfigData {
    pub buyers: Vec<MerkleAllowlistBuyer>,
    pub root: MerkleRoot,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MerkleAllowlistBuyer {
    pub address: String,
    pub amount: u16,
    pub proof: Vec<MerkleProof>,
    pub merkle_tree_index: u16,
    pub keypair_object: KeypairStruct,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeypairStruct {
    pub public_key: String,
    pub secret_key: Vec<u8>,
}

fn parse_merkle_allowlist_test_config() -> MerkleAllowlistTestConfig {
    let file = File::open(TEST_CONFIG_FILE_PATH);
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let config: MerkleAllowlistTestConfig = serde_json::from_str(&contents).unwrap();
            return config;
        }
        Err(_e) => {
            panic!(
                "Could not find merkle allowlist test config file at path: {}. Did you generate the test config using the yarn script?",
                TEST_CONFIG_FILE_PATH
            );
        }
    }
}

/**
 * Chunk the total roots list into smaller chunks. The chunk size is limited by
 * how many roots can find in a single transaction.
 */
fn chunk_root_list_for_update_txs(roots_to_append: Vec<MerkleRoot>) -> Vec<Vec<MerkleRoot>> {
    let chunked_roots_to_add: Vec<Vec<MerkleRoot>> = roots_to_append
        .chunks(MAX_ROOT_NUMBER_PER_APPEND_MERKLE_ALLOWLIST_TX_FOR_TEST)
        .map(|x| x.to_vec())
        .collect();

    chunked_roots_to_add
}

pub struct AllowlistConfig {
    pub candy_machine_keypair: Keypair,
    pub first_minter_keypair: Keypair,
    pub total_number_of_buyers: u64,
    pub total_mint_amount: u64,
    pub chunked_roots_to_add: Vec<Vec<MerkleRoot>>,
    pub allowlist_buyers: Vec<MerkleAllowlistBuyer>,
    pub merkle_allowlist_data: Vec<MerkleAllowlistTestConfigData>,
}

// Helper function which processes the allowlist test config data to make it
// easier to use in the tests.
pub fn get_allowlist_config_data() -> AllowlistConfig {
    let merkle_allowlist_config = parse_merkle_allowlist_test_config();
    let candy_machine_keypair =
        Keypair::from_bytes(&merkle_allowlist_config.candy_machine_keypair.secret_key).unwrap();
    let first_minter_keypair = Keypair::from_bytes(
        &merkle_allowlist_config.merkle_allowlist_data[0].buyers[0]
            .keypair_object
            .secret_key,
    )
    .unwrap();

    let total_number_of_buyers: u64 = merkle_allowlist_config
        .merkle_allowlist_data
        .iter()
        .fold(0u64, |total, item| total + item.buyers.len() as u64);

    let total_mint_amount: u64 =
        merkle_allowlist_config
            .merkle_allowlist_data
            .iter()
            .fold(0u64, |total, item| {
                total
                    + item
                        .buyers
                        .iter()
                        .fold(0u64, |total, buyer| total + buyer.amount as u64)
            });

    let roots_to_add: Vec<MerkleRoot> = merkle_allowlist_config
        .merkle_allowlist_data
        .iter()
        .map(|config_data| config_data.root)
        .collect();

    let chunked_roots_to_add: Vec<Vec<MerkleRoot>> = chunk_root_list_for_update_txs(roots_to_add);

    let allowlist_buyers = merkle_allowlist_config
        .merkle_allowlist_data
        .iter()
        .map(|allowlist_section| allowlist_section.buyers.clone())
        .flatten()
        .collect();

    let merkle_allowlist_data = merkle_allowlist_config.merkle_allowlist_data;

    return AllowlistConfig {
        candy_machine_keypair,
        first_minter_keypair,
        total_number_of_buyers,
        total_mint_amount,
        chunked_roots_to_add,
        allowlist_buyers,
        merkle_allowlist_data,
    };
}

pub fn get_empty_merkle_tree_node() -> MerkleTreeNode {
    [0; 32]
}

/**
 * Creates invalid allowlist proof data for testing. Tries to be creative and
 * if the current (valid) buyer index is:
 *  even it uses an adjacent buyer proof
 *  odd it changes the amount encoded in the proof.
 */
pub fn get_invalid_merkle_allowlist_proof_data(
    buyers_section: &Vec<MerkleAllowlistBuyer>,
    current_index: usize,
) -> BuyerMerkleAllowlistProofData {
    let current_buyer = &buyers_section[current_index];

    let adjacent_buyer_index = if current_index < buyers_section.len() - 1 {
        current_index + 1
    } else {
        current_index - 1
    };

    match current_index % 2 {
        0 => BuyerMerkleAllowlistProofData {
            amount: current_buyer.amount,
            proof: buyers_section[adjacent_buyer_index].proof.clone(),
            root_index_for_proof: current_buyer.merkle_tree_index,
        },
        _ => BuyerMerkleAllowlistProofData {
            amount: if current_buyer.amount > 1 {
                current_buyer.amount - 1
            } else {
                current_buyer.amount + 1
            },
            proof: current_buyer.proof.clone(),
            root_index_for_proof: current_buyer.merkle_tree_index,
        },
    }
}
