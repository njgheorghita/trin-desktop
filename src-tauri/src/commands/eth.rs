use crate::types::config::TrinConfig;
use alloy::primitives::B256;
use alloy::rpc::types::{Block, BlockNumberOrTag};
use ethportal_api::jsonrpsee::http_client::HttpClientBuilder;
use ethportal_api::EthApiClient;
use log::{info, warn};
use std::str::FromStr;

#[tauri::command]
#[allow(non_snake_case)]
pub async fn eth_getBlockByHash(
    trin_config: TrinConfig,
    block_hash: String,
) -> Result<Block, String> {
    info!("eth_getBlockByHash: {:?}", block_hash);
    let endpoint = format!("http://localhost:{}", trin_config.httpPort);
    let client = HttpClientBuilder::default().build(&endpoint).unwrap();
    let block_hash = B256::from_str(&block_hash).unwrap();
    let block = client.get_block_by_hash(block_hash, false).await;
    match block {
        Ok(block) => {
            info!("eth_getBlockByNumber: {:?}", block);
            Ok(block)
        }
        Err(e) => {
            warn!("eth_getBlockByHash: {:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
#[allow(non_snake_case)]
pub async fn eth_getBlockByNumber(
    trin_config: TrinConfig,
    block_number: u64,
) -> Result<Block, String> {
    info!("eth_getBlockByNumber: {:?}", block_number);
    let endpoint = format!("http://localhost:{}", trin_config.httpPort);
    let client = HttpClientBuilder::default().build(&endpoint).unwrap();
    let block_number = BlockNumberOrTag::Number(block_number.into());
    let block = client.get_block_by_number(block_number, false).await;
    match block {
        Ok(block) => {
            info!("eth_getBlockByNumber: {:?}", block);
            Ok(block)
        }
        Err(e) => {
            warn!("eth_getBlockByNumber: {:?}", e);
            Err(e.to_string())
        }
    }
}
