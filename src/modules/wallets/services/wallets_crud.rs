use sqlx::PgPool;

pub struct WalletsCrud<'a> {
    pub db: &'a PgPool,
}

impl<'a> WalletsCrud<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    pub async fn create_wallet(&self, wallet: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO wallets (wallet) VALUES ($1)")
            .bind(wallet)
            .execute(self.db)
            .await?;
        Ok(())
    }
}
