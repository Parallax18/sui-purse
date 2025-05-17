mod db;
mod state;
mod swagger;

use crate::db::init::init_app_state;
use axum::Router;
use dotenvy;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at: {}", database_url);
    tracing_subscriber::fmt::init();

    let app_state = Arc::new(init_app_state().await);

    // env_logger::init();
    let api_doc = swagger::ApiDoc::openapi();

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api_doc.clone()))
        .with_state(app_state.clone()); // this sets it for the top-level router only

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8081);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let ip: IpAddr = host.parse().expect("Invalid IP address in HOST env var");
    let addr = SocketAddr::from((ip, port));

    println!("Riddo API running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
