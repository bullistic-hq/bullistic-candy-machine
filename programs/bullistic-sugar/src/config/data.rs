use std::{
    fmt::{self, Display},
    str::FromStr,
};

use anchor_client::solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair,
};
pub use anyhow::{anyhow, Result};
use chrono::prelude::*;
use bullistic_candy_machine::{
    Creator as CandyCreator, HiddenSettings as CandyHiddenSettings,
    SplTokenAllowlistMode as CandySplTokenAllowlistMode,
    SplTokenAllowlistSettings as CandySplTokenAllowlistSettings,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::config::errors::*;

pub struct SugarConfig {
    pub keypair: Keypair,
    pub rpc_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SolanaConfig {
    pub json_rpc_url: String,
    pub keypair_path: String,
    pub commitment: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigData {
    pub price: f64,
    pub number: u64,

    pub creators: Vec<Creator>,

    #[serde(deserialize_with = "to_pubkey")]
    #[serde(serialize_with = "to_string")]
    pub creator_authority: Pubkey,

    // #[serde(deserialize_with = "to_pubkey")]
    // #[serde(serialize_with = "to_string")]
    pub premint_price: Option<f64>,

    // #[serde(deserialize_with = "to_pubkey")]
    // #[serde(serialize_with = "to_string")]
    pub allowlist_price: Option<f64>,

    #[serde(deserialize_with = "to_option_pubkey")]
    #[serde(serialize_with = "to_option_string")]
    pub sol_treasury_account: Option<Pubkey>,

    #[serde(deserialize_with = "to_option_pubkey")]
    #[serde(serialize_with = "to_option_string")]
    pub spl_token_account: Option<Pubkey>,

    #[serde(deserialize_with = "to_option_pubkey")]
    #[serde(serialize_with = "to_option_string")]
    pub spl_token: Option<Pubkey>,

    pub allowlist_sale_start_time: Option<String>,

    pub public_sale_start_time: String,

    pub public_sale_end_time: Option<String>,

    pub spl_token_allowlist_settings: Option<SplTokenAllowlistSettings>,

    pub hidden_settings: Option<HiddenSettings>,

    pub upload_method: UploadMethod,

    pub is_mutable: bool,

    pub symbol: String,

    pub seller_fee_basis_points: u16,

    pub aws_config: Option<AwsConfig>,

    pub limit_per_address: u16,

    pub bot_protection_enabled: bool,

    pub sequential_mint_order_enabled: bool,

    #[serde(serialize_with = "to_option_string")]
    pub nft_storage_auth_token: Option<String>,

    #[serde(serialize_with = "to_option_string")]
    pub shdw_storage_account: Option<String>,
}

pub fn to_string<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    serializer.collect_str(value)
}

pub fn to_option_string<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    match value {
        Some(v) => serializer.collect_str(&v),
        None => serializer.serialize_none(),
    }
}

pub fn parse_string_as_date(date_string: &str) -> Result<String> {
    let date = dateparser::parse_with(
        date_string,
        &Local,
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    )?;

    Ok(date.to_rfc3339())
}

pub fn config_time_as_timestamp(config_time_string: &str) -> Result<i64> {
    let date = dateparser::parse(config_time_string);
    if date.is_err() {
        return Ok(0);
    }

    Ok(date.unwrap().timestamp())
}

pub fn config_time_opt_as_timestamp(config_time_opt: &Option<String>) -> Result<Option<i64>> {
    if let Some(time_string) = config_time_opt {
        let date = dateparser::parse(time_string);
        if date.is_err() {
            return Ok(None);
        }

        Ok(Some(date.unwrap().timestamp()))
    } else {
        Ok(None)
    }
}

pub fn price_as_lamports(price: f64) -> u64 {
    (price * LAMPORTS_PER_SOL as f64) as u64
}

pub fn to_pubkey<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Pubkey::from_str(&s).map_err(serde::de::Error::custom)
}

fn to_option_pubkey<'de, D>(deserializer: D) -> Result<Option<Pubkey>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = match Deserialize::deserialize(deserializer) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };

    let pubkey = Pubkey::from_str(&s).map_err(serde::de::Error::custom)?;
    Ok(Some(pubkey))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplTokenAllowlistSettings {
    mode: SplTokenAllowlistMode,
    #[serde(deserialize_with = "to_pubkey")]
    #[serde(serialize_with = "to_string")]
    mint: Pubkey,
}

impl SplTokenAllowlistSettings {
    pub fn new(mode: SplTokenAllowlistMode, mint: Pubkey) -> SplTokenAllowlistSettings {
        SplTokenAllowlistSettings { mode, mint }
    }
    pub fn to_candy_format(&self) -> CandySplTokenAllowlistSettings {
        CandySplTokenAllowlistSettings {
            mode: self.mode.to_candy_format(),
            mint: self.mint,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SplTokenAllowlistMode {
    BurnEveryTime,
    NeverBurn,
}

impl SplTokenAllowlistMode {
    pub fn to_candy_format(&self) -> CandySplTokenAllowlistMode {
        match self {
            SplTokenAllowlistMode::BurnEveryTime => CandySplTokenAllowlistMode::BurnEveryTime,
            SplTokenAllowlistMode::NeverBurn => CandySplTokenAllowlistMode::NeverBurn,
        }
    }
}

impl FromStr for SplTokenAllowlistMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "burneverytime" => Ok(SplTokenAllowlistMode::BurnEveryTime),
            "neverburn" => Ok(SplTokenAllowlistMode::NeverBurn),
            _ => Err(anyhow::anyhow!("Invalid SPL token allowlist mode: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenSettings {
    name: String,
    uri: String,
    hash: String,
}

impl HiddenSettings {
    pub fn new(name: String, uri: String, hash: String) -> HiddenSettings {
        HiddenSettings { name, uri, hash }
    }
    pub fn to_candy_format(&self) -> CandyHiddenSettings {
        CandyHiddenSettings {
            name: self.name.clone(),
            uri: self.uri.clone(),
            hash: self.hash.as_bytes().try_into().unwrap_or([0; 32]),
        }
    }
    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UploadMethod {
    Bundlr,
    #[serde(rename = "aws")]
    AWS,
    NftStorage,
    #[serde(rename = "shdw")]
    SHDW,
}

impl Display for UploadMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for UploadMethod {
    fn default() -> UploadMethod {
        UploadMethod::Bundlr
    }
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct Creator {
    #[serde(deserialize_with = "to_pubkey")]
    #[serde(serialize_with = "to_string")]
    pub address: Pubkey,
    pub share: u8,
}

impl Creator {
    pub fn to_candy_format(&self) -> Result<CandyCreator> {
        let creator = CandyCreator {
            address: self.address,
            share: self.share,
            verified: false,
        };

        Ok(creator)
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Cluster {
    Devnet,
    Mainnet,
    Unknown,
}

impl FromStr for Cluster {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "devnet" => Ok(Cluster::Devnet),
            "mainnet" => Ok(Cluster::Mainnet),
            "unknown" => Ok(Cluster::Unknown),
            _ => Err(ConfigError::InvalidCluster(s.to_string()).into()),
        }
    }
}

impl ToString for Cluster {
    fn to_string(&self) -> String {
        match self {
            Cluster::Devnet => "devnet".to_string(),
            Cluster::Mainnet => "mainnet".to_string(),
            Cluster::Unknown => "unknown".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    pub bucket: String,
    pub profile: String,
    pub directory: String,
}

impl AwsConfig {
    pub fn new(bucket: String, profile: String, directory: String) -> AwsConfig {
        AwsConfig {
            bucket,
            profile,
            directory,
        }
    }
}
