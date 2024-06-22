use actix_web::dev::{ConnectionInfo, PeerAddr};
use actix_web::http::Method;
use actix_web::{web, App, HttpRequest, HttpServer};

mod types;
use types::{ClientAddr, EchoInfo, ServerAddr};

async fn index(
    req: HttpRequest,
    peer: PeerAddr,
    method: Method,
    conn_info: ConnectionInfo,
) -> web::Json<EchoInfo> {
    tracing::debug!("Received request: {:?}", req);
    let client: ClientAddr =
        ClientAddr::new(peer.into_inner().ip().to_string(), peer.into_inner().port());
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
    let params = req
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
    tracing::debug!("Query params: {:?}", params);
    let server = ServerAddr::new(
        conn_info.host().to_string(),
        conn_info.scheme().to_string().to_uppercase(),
    );

    let echo = EchoInfo::new(
        req.path().to_string(),
        headers,
        params,
        cookies,
        client,
        server,
        method.to_string(),
    );

    tracing::info!("Echo: {:?}", echo);
    web::Json(echo)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting server at 0.0.0.0:3000");

    HttpServer::new(|| App::new().service(web::resource("{tail}*").to(index)))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}
