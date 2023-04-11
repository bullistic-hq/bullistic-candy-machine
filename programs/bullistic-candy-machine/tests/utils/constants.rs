pub const DEFAULT_SOL_AIRDROP_SIZE: u64 = 50;

// Keypair for the local/dev/test bot signer authority public key: antiDV8bRvF4XTeRqmyHV1jpHD4Lvz7gKBKBBRQb8ir
pub const BOT_SIGNER_AUTHORITY_SECRET: &[u8; 64] = &[
    149, 20, 248, 180, 209, 250, 93, 153, 76, 164, 228, 237, 88, 103, 194, 12, 27, 137, 124, 161,
    240, 246, 12, 48, 25, 145, 233, 236, 175, 246, 141, 89, 8, 168, 14, 115, 201, 205, 234, 225,
    64, 207, 252, 224, 51, 231, 187, 61, 142, 37, 48, 213, 190, 24, 85, 40, 11, 227, 150, 228, 142,
    127, 207, 183,
];

// Arbitrarily setting this to something low for testing. This represents the
// max length of a Vec<MerkleRoot> to send in a single update ix to the program
// and is limited in size by the total transaction size.
pub const MAX_ROOT_NUMBER_PER_APPEND_MERKLE_ALLOWLIST_TX_FOR_TEST: usize = 10;

pub static TEST_CONFIG_FILE_PATH: &'static str = "../../allowlist-config/program-test-config.json";
