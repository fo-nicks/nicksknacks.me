use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Create a router to serve static files
    let app = Router::new().fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
