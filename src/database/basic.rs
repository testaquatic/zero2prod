use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[trait_variant::make(Zero2ProdDatabase: Send)]
pub trait LocalZero2ProdDatabase {
    type Output: Zero2ProdDatabase;
    async fn connect(address: &str) -> Result<Self::Output, sqlx::Error>;
    async fn insert_subscriptions(
        &self,
        id: Uuid,
        email: &str,
        name: &str,
        subscribed_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error>;
}
