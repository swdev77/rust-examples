mod config;
mod db;
mod state;
mod models;
mod handlers;

use axum::{
    routing::{get, },
    Router,
    http::StatusCode,
};

use crate::handlers::get_users;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let fb_config = config::FbConfig::from_env().unwrap();

    let fb_connect_options = db::get_fb_connect_options(fb_config);

    let shared_state = state::AppState {
        connect_options: fb_connect_options,
    };
    
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(about))
        .route(
            "/users",
            get(get_users))
        .with_state(shared_state);

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn about() -> Result<String, StatusCode> {
    let msg = "The Axum REST API with Firebird".to_string();
    Ok(msg)
}