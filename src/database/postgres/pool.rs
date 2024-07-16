use std::ops::Deref;

use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    PgPool, Postgres,
};

use crate::{configuration::DatabaseSettings, database::basic::Zero2ProdDatabase};

use super::pg_insert_subscriptions;

#[derive(Clone)]
pub struct PostgresPool {
    pg_pool: PgPool,
}

impl Zero2ProdDatabase for PostgresPool {
    type DB = Postgres;
    type ConnectOutput = Self;

    async fn connect(database_settings: &DatabaseSettings) -> Result<Self, sqlx::Error> {
        // `?` 연산자를 사용해서 함수가 실패하면, 조기에 sqlx::Error를 반환한다.
        // 풀이 처음 사용될 때만 커넥션 연결을 시도한다.
        let pg_pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy(&database_settings.connection_string())?;
        let postgres_pool = Self { pg_pool };
        Ok(postgres_pool)
    }

    async fn insert_subscriptions(
        &self,
        id: uuid::Uuid,
        email: &str,
        name: &str,
        subscribed_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<PgQueryResult, sqlx::Error> {
        pg_insert_subscriptions(&self.pg_pool, id, email, name, subscribed_at).await
    }
}

impl Deref for PostgresPool {
    type Target = PgPool;
    fn deref(&self) -> &Self::Target {
        &self.pg_pool
    }
}
