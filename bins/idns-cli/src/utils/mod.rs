use idns_eth_sdk::idns_eth_request_raw;
use std::collections::HashMap;

pub fn idns_request(
    service_name: &str,
    method_name: &str,
    headers: HashMap<String, String>,
    data: Vec<u8>,
) -> (i32, String, Vec<u8>) {
    idns_eth_request_raw(service_name, method_name, headers, data)
}
