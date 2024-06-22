use serde::Serialize;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
pub struct ClientAddr {
    host: String,
    port: u16,
}

impl ClientAddr {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
}

#[derive(Debug, Serialize)]
pub struct ServerAddr {
    host: String,
    scheme: String,
}

impl ServerAddr {
    pub fn new(host: String, scheme: String) -> Self {
        Self { host, scheme }
    }
}

#[derive(Debug, Serialize)]
pub struct Time {
    epoch: u64,
    isoformat: String,
}

#[derive(Debug, Serialize)]
pub struct EchoInfo {
    time: Time,
    server: ServerAddr,
    endpoint: String,
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
    cookies: HashMap<String, String>,
    client: ClientAddr,
    method: String,
}

impl EchoInfo {
    pub fn new(
        endpoint: String,
        headers: HashMap<String, String>,
        query_params: HashMap<String, String>,
        cookies: HashMap<String, String>,
        client: ClientAddr,
        server: ServerAddr,
        method: String,
    ) -> Self {
        Self {
            endpoint,
            headers,
            method,
            params: query_params,
            cookies,
            client,
            server,
            time: Time {
                epoch: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                isoformat: chrono::Utc::now().to_rfc3339(),
            },
        }
    }
}
