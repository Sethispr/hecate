use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use axum::routing::get;

mod routes;
mod health;

#[tokio::main]
async fn main() {
    let state = Arc::new(routes::AppState::new());

    let app = Router::new()
        .route("/api/status", get(routes::status_handler))
        .with_state(state)
        .layer(CorsLayer::permissive());

    // by default sumi uses 8080, we run uptime backend on 3001
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("uptime backend listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}
