mod db;
mod modules;
mod state;
mod swagger;

use crate::db::init::init_app_state;
use axum::Router;
use dotenvy;
use modules::sui::services::sui_core::{generate_keypair, get_balance};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Generated address: 0xbc34b9f712893efb84c7611393a50a608b020092adfa9b880ddff8aa84245020
// Private key: <elided secret for Ed25519PrivateKey>
// Sui key pair: Ed25519(Ed25519KeyPair { public: GHYuyVUcxDUzCAKbxmZ19vShITWruoHHtY2a5n6pDfA=, private: <elided secret for Ed25519PrivateKey> })

#[tokio::main]
async fn main() {
    // generate_keypair().await;
    // get_balance("0x9d1ad814d0775fd0b6b9caaf66d16c44ca93dfed9fcc8ce105dea03f2b0dbf80").await;
    get_balance("0xae04e4caa23527565180925f2af20f23101e40ca870b9b54e05564814a9860a9").await;

    // let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // println!("Connecting to database at: {}", database_url);
    // tracing_subscriber::fmt::init();

    // let app_state = Arc::new(init_app_state().await);

    // // env_logger::init();
    // let api_doc = swagger::ApiDoc::openapi();

    // let app = Router::new()
    //     .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api_doc.clone()))
    //     .with_state(app_state.clone()); // this sets it for the top-level router only

    // let port = std::env::var("PORT")
    //     .ok()
    //     .and_then(|p| p.parse().ok())
    //     .unwrap_or(8081);

    // let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    // let ip: IpAddr = host.parse().expect("Invalid IP address in HOST env var");
    // let addr = SocketAddr::from((ip, port));

    // println!("Riddo API running at http://{}", addr);

    // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}
