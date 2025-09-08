use axum::{
    routing::get,
    Router,
};

mod routes;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = Router::new()
        .route("/health", get(handlers::health::check))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Run it
    let addr: std::net::SocketAddr = "[::]:3000".parse().unwrap();
    tracing::info!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
