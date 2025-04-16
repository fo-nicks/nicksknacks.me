use std::net::SocketAddr;

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new().fallback_service(ServeDir::new("static"));

    let port = port();
    let (cert, key) = certs();
    let config = RustlsConfig::from_pem_file(cert, key).await.unwrap();

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    axum_server::bind_rustls(address, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn port() -> u16 {
    if cfg!(not(debug_assertions)) {
        return 443;
    } else {
        return 3000;
    }
}

fn certs() -> (&'static str, &'static str) {
    if cfg!(not(debug_assertions)) {
        (
            "/etc/letsencrypt/live/nicksknacks.me/fullchain.pem",
            "/etc/letsencrypt/live/nicksknacks.me/privkey.pem",
        )
    } else {
        ("certs/localhost.crt", "certs/localhost.key")
    }
}
