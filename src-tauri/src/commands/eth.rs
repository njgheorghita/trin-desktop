use crate::types::config::TrinConfig;
use alloy::primitives::{Address, B256, U256};
use alloy::rpc::types::{Block, BlockId, BlockNumberOrTag};
use ethportal_api::jsonrpsee::http_client::HttpClientBuilder;
use ethportal_api::utils::bytes::hex_decode;
use ethportal_api::EthApiClient;
use log::info;
use std::str::FromStr;

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum BlockNumberOrTagInput {
    Number(u64),
    Tag(String),
}

// todo: any component that uses the blocknumber needs to be updated 
// to use the new blockNumberOrTagInput once trin supports it
#[derive(Debug)]
pub struct InvalidBlockTagError(String);

impl std::fmt::Display for InvalidBlockTagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid block tag: {}", self.0)
    }
}

impl std::error::Error for InvalidBlockTagError {}

impl TryFrom<BlockNumberOrTagInput> for BlockNumberOrTag {
    type Error = InvalidBlockTagError;

    fn try_from(input: BlockNumberOrTagInput) -> Result<Self, Self::Error> {
        match input {
            BlockNumberOrTagInput::Number(n) => Ok(BlockNumberOrTag::Number(n)),
            BlockNumberOrTagInput::Tag(s) => BlockNumberOrTag::from_str(&s)
                .map_err(|_| InvalidBlockTagError(s)),
        }
    }
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn eth_getBlockByHash(
    trin_config: TrinConfig,
    block_hash: String,
) -> Result<Block, String> {
    info!("eth_getBlockByHash: {:?}", block_hash);
    let endpoint = format!("http://localhost:{}", trin_config.httpPort);
    let client = HttpClientBuilder::default()
        .build(&endpoint)
        .map_err(|e| e.to_string())?;
    let block_hash = B256::from_str(&block_hash).map_err(|e| e.to_string())?;
    client
        .get_block_by_hash(block_hash, false)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn eth_getBlockByNumber(
    trin_config: TrinConfig,
    block_number: u64,
) -> Result<Block, String> {
    info!("eth_getBlockByNumber: {:?}", block_number);
    let endpoint = format!("http://localhost:{}", trin_config.httpPort);
    let client = HttpClientBuilder::default()
        .build(&endpoint)
        .map_err(|e| e.to_string())?;
    let block_number = BlockNumberOrTag::Number(block_number.into());
    client
        .get_block_by_number(block_number, false)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn eth_getBalance(
    trin_config: TrinConfig,
    address: String,
    block_number: BlockNumberOrTagInput,
) -> Result<U256, String> {
    info!("eth_getBalance: {:?} @ {:?}", address, block_number);
    let endpoint = format!("http://localhost:{}", trin_config.httpPort);
    let client = HttpClientBuilder::default()
        .build(&endpoint)
        .map_err(|e| e.to_string())?;
    let raw_address = hex_decode(&address).map_err(|e| e.to_string())?;
    let address = Address::from_slice(&raw_address);
    let block_number: BlockNumberOrTag = block_number.try_into().map_err(|e: InvalidBlockTagError| e.to_string())?;
    let block_id = BlockId::Number(block_number);
    client
        .get_balance(address, block_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn eth_getCode(
    trin_config: TrinConfig,
    address: String,
    block_number: BlockNumberOrTagInput,
) -> Result<String, String> {
    info!("eth_getCode: {:?} @ {:?}", address, block_number);
    let endpoint = format!("http://localhost:{}", trin_config.httpPort);
    let client = HttpClientBuilder::default()
        .build(&endpoint)
        .map_err(|e| e.to_string())?;
    let raw_address = hex_decode(&address).map_err(|e| e.to_string())?;
    let address = Address::from_slice(&raw_address);
    let block_number: BlockNumberOrTag = block_number.try_into().map_err(|e: InvalidBlockTagError| e.to_string())?;
    let block_id = BlockId::Number(block_number);
    client
        .get_code(address, block_id)
        .await
        .map_err(|e| e.to_string())
        .map(|bytes| format!("0x{}", hex::encode(bytes.as_ref())))
}