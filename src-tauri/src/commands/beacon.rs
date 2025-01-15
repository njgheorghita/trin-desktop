use ethportal_api::jsonrpsee::http_client::HttpClientBuilder;
use ethportal_api::light_client::finality_update::LightClientFinalityUpdate;
use ethportal_api::light_client::optimistic_update::LightClientOptimisticUpdate;
use ethportal_api::BeaconNetworkApiClient;
use log::info;

#[allow(non_snake_case)]
pub async fn portal_beaconFinalityUpdate(
    http_port: usize,
) -> Result<LightClientFinalityUpdate, String> {
    info!("portal_beaconFinalityUpdate");
    let endpoint = format!("http://localhost:{http_port}");
    let client = HttpClientBuilder::default()
        .request_timeout(std::time::Duration::from_secs(10))
        .build(&endpoint)
        .map_err(|e| e.to_string())?;
    let result = client.finality_update().await.map_err(|e| e.to_string());
    info!("portal_beaconFinalityUpdate: {:?}", result);
    result
}

#[allow(non_snake_case)]
pub async fn portal_beaconOptimisticUpdate(
    http_port: usize,
) -> Result<LightClientOptimisticUpdate, String> {
    info!("portal_beaconOptimisticUpdate");
    let endpoint = format!("http://localhost:{http_port}");
    let client = HttpClientBuilder::default()
        .request_timeout(std::time::Duration::from_secs(10))
        .build(&endpoint)
        .map_err(|e| e.to_string())?;
    let result = client.optimistic_update().await.map_err(|e| e.to_string());
    info!("portal_beaconOptimisticUpdate: {:?}", result);
    result
}
