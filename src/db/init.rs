use crate::state::app_state::AppState;
use dotenvy;
use sqlx::{Executor, postgres::PgPoolOptions};

pub async fn init_app_state() -> AppState {
    dotenvy::dotenv().ok(); // Load env vars

    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let jwt_secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                conn.execute("SET statement_timeout = 10000").await?;
                Ok(())
            })
        })
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("db connection, success");

    AppState {
        db: pool,
        // jwt_secret,
    }
}
