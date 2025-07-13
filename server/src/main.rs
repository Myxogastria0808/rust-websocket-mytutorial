use crate::{
    errors::root::RootError,
    handlers::{ping::ping_handler, ws::websocket_handler},
};
use axum::{Router, extract::DefaultBodyLimit, routing::get};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

pub mod errors;
pub mod handlers;
pub mod models;

// Domain
const IP_ADDRESS: &str = "localhost";
const PORT: u16 = 7000;

#[tokio::main]
async fn main() -> Result<(), RootError> {
    // shared object
    let shared_state = Arc::new(RwLock::new(0));
    // tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    // cors
    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);

    // router
    let app: Router<()> = Router::new()
        .route("/", get(ping_handler))
        .route("/ws", get(websocket_handler))
        .layer(cors)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 100)) //100MB
        .with_state(Arc::clone(&shared_state));

    // server
    let listener = tokio::net::TcpListener::bind(format!("{IP_ADDRESS}:{PORT}")).await?;

    //* start server *//
    tracing::info!("listening on ws://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
