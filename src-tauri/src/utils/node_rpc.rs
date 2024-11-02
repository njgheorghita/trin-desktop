use ethportal_api::jsonrpsee::http_client::HttpClientBuilder;
use ethportal_api::Web3ApiClient;

// this is the jsonrpc request used to make sure
// that the trin node is running
// ... hmm. ok this might not be the best way to check that
// the trin node is running. eg. the node will respond even if
// it is not connected to the network (aka error binding to udp socket)
pub async fn check_trin_status(http_port: &usize) -> bool {
    let endpoint = format!("http://localhost:{}", http_port);
    let client = HttpClientBuilder::default().build(&endpoint).unwrap();
    client.client_version().await.is_ok()
}
