use hostname::get;
use serde::{Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
pub struct EchoInfo {
    epoch: u64,
    time_isoformat: String,
    hostname: String,
    server_hostname: String,
    request_uri: String,
    headers: HashMap<String, String>,
    body: Option<String>,
    query_params: HashMap<String, String>,
    cookies: HashMap<String, String>,
    client_ip: String,
}

impl EchoInfo {
    pub fn new(
        request_uri: String,
        headers: HashMap<String, String>,
        body: Option<String>,
        query_params: HashMap<String, String>,
        cookies: HashMap<String, String>,
        client_ip: String,
        server_hostname: String,
    ) -> Self {
        Self {
            request_uri,
            headers,
            body,
            query_params,
            cookies,
            client_ip,
            server_hostname,
            epoch: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            time_isoformat: chrono::Utc::now().to_rfc3339(),
            hostname: get().unwrap().into_string().unwrap(),
        }
    }
}
