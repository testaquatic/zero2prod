use std::ops::Deref;

use sqlx::{postgres::PgQueryResult, PgPool};

use crate::database::basic::Zero2ProdDatabase;

use super::pg_insert_subscriptions;

#[derive(Clone)]
pub struct PostgresPool {
    pg_pool: PgPool,
}

impl Zero2ProdDatabase for PostgresPool {
    type ConnectOutput = PostgresPool;
    type QueryResult = PgQueryResult;

    async fn connect(pg_address: &str) -> Result<Self, sqlx::Error> {
        // `?` 연산자를 사용해서 함수가 실패하면, 조기에 sqlx::Error를 반환한다.
        let pg_pool = PgPool::connect(pg_address).await?;
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
