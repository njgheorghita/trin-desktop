use crate::types::config::TrinConfig;
use alloy::primitives::{Address, B256, U256};
use alloy::rpc::types::{Block, BlockId, BlockNumberOrTag, RpcBlockHash};
use ethportal_api::jsonrpsee::http_client::HttpClientBuilder;
use ethportal_api::utils::bytes::hex_decode;
use ethportal_api::EthApiClient;
use log::info;
use std::str::FromStr;

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
    _block_number: u64,
) -> Result<U256, String> {
    info!("eth_getBalance: {:?} @ 1,000,000", address);
    let endpoint = format!("http://localhost:{}", trin_config.httpPort);
    let block_hash_one_million =
        B256::from_str("0x8e38b4dbf6b11fcc3b9dee84fb7986e29ca0a02cecd8977c161ff7333329681e")
            .unwrap();
    let client = HttpClientBuilder::default()
        .build(&endpoint)
        .map_err(|e| e.to_string())?;
    let raw_address = hex_decode(&address).map_err(|e| e.to_string())?;
    let address = Address::from_slice(&raw_address);
    let block_id = BlockId::Hash(RpcBlockHash::from(block_hash_one_million));
    client
        .get_balance(address, block_id)
        .await
        .map_err(|e| e.to_string())
}
