use actix_web::{get, web, App, HttpRequest, HttpServer};

mod types;
use types::EchoInfo;

#[get("/")]
async fn index(req: HttpRequest) -> web::Json<EchoInfo> {
    tracing::debug!("Received request: {:?}", req);
    let headers = req
        .headers()
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_str().unwrap().to_string()))
        .collect();
    tracing::debug!("Headers: {:?}", headers);
    let cookies = req
        .cookies()
        .iter()
        .flat_map(|cookies| {
            cookies
                .iter()
                .map(|cookie| (cookie.name().to_string(), cookie.value().to_string()))
        })
        .collect();
    tracing::debug!("Cookies: {:?}", cookies);
    let query_params = req
        .query_string()
        .split('&')
        .filter_map(|param| {
            let mut split = param.split('=');
            match (split.next(), split.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect();
    tracing::debug!("Query params: {:?}", query_params);
    let client_ip = req.connection_info().peer_addr().unwrap().to_string();
    // get server ip from the request
    let server_hostname = req.connection_info().host().to_string();

    let echo = EchoInfo::new(
        req.uri().to_string(),
        headers,
        None,
        query_params,
        cookies,
        client_ip,
        server_hostname,
    );

    tracing::info!("Echo: {:?}", echo);
    web::Json(echo)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting server at 0.0.0.0:3000");

    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
