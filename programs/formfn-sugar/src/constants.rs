use console::Emoji;
pub use mpl_token_metadata::state::{
    MAX_CREATOR_LEN, MAX_CREATOR_LIMIT, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH,
};

/// Metaplex program id.
pub const METAPLEX_PROGRAM_ID: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

pub const STRING_LEN_SIZE: usize = 4;

pub const CONFIG_NAME_OFFSET: usize = STRING_LEN_SIZE;

pub const CONFIG_URI_OFFSET: usize = STRING_LEN_SIZE + CONFIG_NAME_OFFSET + MAX_NAME_LENGTH;

pub const MINT_LAYOUT: u64 = 82;

pub const DEFAULT_UUID: &str = "000000";

/// Maximum number of concurrent tasks (this is important for tasks that handle files
/// and network connections).
pub const PARALLEL_LIMIT: usize = 45;

/// Default path for assets folder.
pub const DEFAULT_ASSETS: &str = "assets";

/// Default path for cache file.
pub const DEFAULT_CACHE: &str = ".sugar-cli-run/cache.json";

/// Default path for config file.
pub const DEFAULT_CONFIG: &str = ".sugar-cli-run/config.json";

/// Default path for merkle allowlist config file.
pub const DEFAULT_MERKLE_ALLOWLIST_CONFIG: &str =
    "allowlist-config/cli-merkle-allowlist-config.json";

pub const CANDY_MACHINE_PUBKEY_FILE: &str = "allowlist-config/candy-machine-pubkey.json";

/// Default path for keypair file.
pub const DEFAULT_KEYPATH: &str = "~/.config/solana/id.json";

/// Bundlr devnet endpoint.
pub const BUNDLR_DEVNET: &str = "https://devnet.bundlr.network";

/// Bundlr mainnet endpoint.
pub const BUNDLR_MAINNET: &str = "https://node1.bundlr.network";

/// Default RPC endpoint for devnet.
pub const DEFAULT_RPC_DEVNET: &str = "https://devnet.genesysgo.net";

pub const LOOKING_GLASS_EMOJI: Emoji<'_, '_> = Emoji("🔍 ", "");

pub const CANDY_EMOJI: Emoji<'_, '_> = Emoji("🍬 ", "");

pub const COMPUTER_EMOJI: Emoji<'_, '_> = Emoji("🖥  ", "");

pub const PAPER_EMOJI: Emoji<'_, '_> = Emoji("📝 ", "");

pub const CONFETTI_EMOJI: Emoji<'_, '_> = Emoji("🎉 ", "");

pub const PAYMENT_EMOJI: Emoji<'_, '_> = Emoji("💵 ", "");

pub const UPLOAD_EMOJI: Emoji<'_, '_> = Emoji("📤 ", "");

pub const WITHDRAW_EMOJI: Emoji<'_, '_> = Emoji("🏧 ", "");

pub const ASSETS_EMOJI: Emoji<'_, '_> = Emoji("🗂  ", "");

pub const COMPLETE_EMOJI: Emoji<'_, '_> = Emoji("✅ ", "");

pub const LAUNCH_EMOJI: Emoji<'_, '_> = Emoji("🚀 ", "");

pub const COLLECTION_EMOJI: Emoji<'_, '_> = Emoji("📦 ", "");

pub const ERROR_EMOJI: Emoji<'_, '_> = Emoji("🛑 ", "");

pub const WARNING_EMOJI: Emoji<'_, '_> = Emoji("⚠️ ", "");

pub const SIGNING_EMOJI: Emoji<'_, '_> = Emoji("✍️ ", "");

// Keypair for the local/dev/test bot signer authority public key: antiDV8bRvF4XTeRqmyHV1jpHD4Lvz7gKBKBBRQb8ir
pub const BOT_SIGNER_AUTHORITY_SECRET: &[u8; 64] = &[
    149, 20, 248, 180, 209, 250, 93, 153, 76, 164, 228, 237, 88, 103, 194, 12, 27, 137, 124, 161,
    240, 246, 12, 48, 25, 145, 233, 236, 175, 246, 141, 89, 8, 168, 14, 115, 201, 205, 234, 225,
    64, 207, 252, 224, 51, 231, 187, 61, 142, 37, 48, 213, 190, 24, 85, 40, 11, 227, 150, 228, 142,
    127, 207, 183,
];
