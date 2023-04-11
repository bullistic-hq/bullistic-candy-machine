// Note: The (merkle allowlist size / leaf count) needs to not be larger than program
// NUMBER_OF_MERKLE_ROOTS_TO_STORE constant (programs/bullistic-candy-machine/src/constants.rs).
export const MERKLE_ALLOWLIST_SIZE = 100;

export const MERKLE_TREE_LEAF_COUNT_FOR_TESTS = 20;

export const DEFAULT_MERKLE_ALLOWLIST_AMOUNT = 1;

export const METADATA_URIS_FILENAME = "create/metadata_uris.txt";

export const CREATE_CONFIG_FILENAME = "create/config.json";

export const SPL_ALLOWLIST_SIZE_FOR_TEST = 25;

// Note: Requests above this size may fail.
export const DEVNET_AIRDROP_SIZE_SOL = 2;

export const API_HEADERS = { check: "fofu" };
