use console::Emoji;
pub use mpl_token_metadata::state::{
    MAX_CREATOR_LEN, MAX_CREATOR_LIMIT, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH,
};

/// Metaplex program id.
pub const METAPLEX_PROGRAM_ID: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

/// Civic gateway program id.
pub const CIVIC: &str = "gatem74V238djXdzWnJf94Wo1DcnuGkfijbf3AuBhfs";

/// Start index of the config data in the PDA (offset calculated in bytes).
pub const CONFIG_ARRAY_START: usize = 8 +   // key
    32 +                                    // authority
    32 +                                    // wallet
    33 +                                    // token mint
    4 + 6 +                                 // uuid
    8 +                                     // price
    8 +                                     // items available
    9 +                                     // go live
    10 +                                    // end settings
    4 + MAX_SYMBOL_LENGTH +                 // u32 len + symbol
    2 +                                     // seller fee basis points
    4 + MAX_CREATOR_LIMIT*MAX_CREATOR_LEN + // optional + u32 len + actual vec
    8 +                                     // max supply
    1 +                                     // is mutable
    1 +                                     // retain authority
    1 +                                     // option for hidden setting
    4 + MAX_NAME_LENGTH +                   // name length
    4 + MAX_URI_LENGTH +                    // uri length
    32 +                                    // hash
    4 +                                     // max number of lines
    8 +                                     // items redeemed
    1 +                                     // whitelist option
    1 +                                     // whitelist mint mode
    1 +                                     // allow presale
    9 +                                     // discount price
    32 +                                    // mint key for whitelist
    1 + 32 + 1                              // gatekeeper
;

/// Default length (in bytes) of a config line.
pub const CONFIG_LINE_SIZE: usize = 4 + MAX_NAME_LENGTH + 4 + MAX_URI_LENGTH;

pub const STRING_LEN_SIZE: usize = 4;

pub const CONFIG_CHUNK_SIZE: usize = 10;

pub const CONFIG_NAME_OFFSET: usize = STRING_LEN_SIZE;

pub const CONFIG_URI_OFFSET: usize = STRING_LEN_SIZE + CONFIG_NAME_OFFSET + MAX_NAME_LENGTH;

pub const MINT_LAYOUT: u64 = 82;

pub const VALID_CATEGORIES: [&str; 5] = ["image", "video", "audio", "vr", "html"];

pub const DEFAULT_UUID: &str = "000000";

/// Maximum number of concurrent tasks (this is important for tasks that handle files
/// and network connections).
pub const PARALLEL_LIMIT: usize = 45;

/// Default path for assets folder.
pub const DEFAULT_ASSETS: &str = "assets";

/// Default path for cache file.
pub const DEFAULT_CACHE: &str = "cache.json";

/// Default path for airdrop list
pub const DEFAULT_AIRDROP_LIST: &str = "airdrop_list.json";
pub const DEFAULT_AIRDROP_LIST_HELP: &str = "Path to airdrop targets list, format: \n{\n\"address1\": number_of_tokens,\n\"address2\": number_of_tokens\n}\n";

/// Default path for config file.
pub const DEFAULT_CONFIG: &str = "config.json";

/// Default path for keypair file.
pub const DEFAULT_KEYPATH: &str = "~/.config/solana/id.json";

/// Bundlr devnet endpoint.
pub const BUNDLR_DEVNET: &str = "https://devnet.bundlr.network";

/// Bundlr mainnet endpoint.
pub const BUNDLR_MAINNET: &str = "https://node1.bundlr.network";

pub const CIVIC_NETWORK: &str = "ignREusXmGrscGNUesoU9mxfds9AiYTezUKex2PsZV6";

pub const ENCORE_NETWORK: &str = "tibePmPaoTgrs929rWpu755EXaxC7M3SthVCf6GzjZt";

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

pub const ICE_CUBE_EMOJI: Emoji<'_, '_> = Emoji("🧊 ", "");

pub const FIRE_EMOJI: Emoji<'_, '_> = Emoji("🔥 ", "");

pub const RIGHT_ARROW_EMOJI: Emoji<'_, '_> = Emoji("➡️ ", "");

pub const MONEY_BAG_EMOJI: Emoji<'_, '_> = Emoji("💰 ", "");

pub const GUARD_EMOJI: Emoji<'_, '_> = Emoji("🛡  ", "");

pub const WRAP_EMOJI: Emoji<'_, '_> = Emoji("📦 ", "");

pub const UNWRAP_EMOJI: Emoji<'_, '_> = Emoji("🔩 ", "");

pub const MAX_FREEZE_DAYS: u8 = 31;

pub const COMPUTE_UNITS: u32 = 400_000;
