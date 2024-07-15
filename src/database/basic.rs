use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

#[trait_variant::make(Zero2ProdDatabase: Send)]
pub trait LocalZero2ProdDatabase {
    type ConnectOutput: Zero2ProdDatabase;
    type QueryResult;

    /// DB에 연결한다.
    async fn connect(address: &str) -> Result<Self::ConnectOutput, sqlx::Error>;

    /// 구독자를 DB에 추가한다.
    async fn insert_subscriptions(
        &self,
        id: Uuid,
        email: &str,
        name: &str,
        subscribed_at: DateTime<Utc>,
    ) -> Result<Self::QueryResult, sqlx::Error>;
}
