// src/state.rs or wherever your AppState is

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    // pub jwt_secret: String,
}
