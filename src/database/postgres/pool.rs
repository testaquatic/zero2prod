use std::ops::Deref;

use sqlx::PgPool;

use crate::database::basic::Zero2ProdDatabase;

use super::pg_insert_subscriptions;

#[derive(Clone)]
pub struct PostgresPool {
    executor: PgPool,
}

impl Zero2ProdDatabase for PostgresPool {
    type Output = PostgresPool;
    async fn connect(pg_address: &str) -> Result<Self, sqlx::Error> {
        let executor = PgPool::connect(pg_address).await?;
        let postgres_connection = Self { executor };
        Ok(postgres_connection)
    }

    async fn insert_subscriptions(
        &self,
        id: uuid::Uuid,
        email: &str,
        name: &str,
        subscribed_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), sqlx::Error> {
        pg_insert_subscriptions(&self.executor, id, email, name, subscribed_at).await?;
        Ok(())
    }
}

impl Deref for PostgresPool {
    type Target = PgPool;
    fn deref(&self) -> &Self::Target {
        &self.executor
    }
}
