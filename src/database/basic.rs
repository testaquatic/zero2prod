use std::ops::Deref;

use chrono::{DateTime, Utc};
use sqlx::types::Uuid;

use crate::configuration::DatabaseSettings;

#[trait_variant::make(Send)]
pub trait Zero2ProdDatabase: Deref<Target = sqlx::Pool<Self::DB>> {
    type ConnectOutput: Zero2ProdDatabase;
    type DB: sqlx::Database;

    /// DB에 연결한다.
    async fn connect(
        database_settings: &DatabaseSettings,
    ) -> Result<Self::ConnectOutput, sqlx::Error>;

    /// 구독자를 DB에 추가한다.
    /// 반환은 Result<'rows_affected', _> 이다.
    async fn insert_subscriptions(
        &self,
        id: Uuid,
        email: &str,
        name: &str,
        subscribed_at: DateTime<Utc>,
    ) -> Result<<Self::DB as sqlx::Database>::QueryResult, sqlx::Error>;
}
