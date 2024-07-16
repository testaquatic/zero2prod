use std::ops::Deref;

use chrono::Utc;
use secrecy::ExposeSecret;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgQueryResult, PgSslMode},
    PgPool, Postgres,
};

use crate::{
    configuration::DatabaseSettings, database::basic::Zero2ProdDatabase, domain::NewSubscriber,
};

use super::pg_insert_subscriptions;

#[derive(Clone)]
pub struct PostgresPool {
    pg_pool: PgPool,
}

impl Zero2ProdDatabase for PostgresPool {
    type DB = Postgres;
    type ConnectOutput = Self;

    async fn connect(database_settings: &DatabaseSettings) -> Result<Self, sqlx::Error> {
        let pg_pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(Self::connect_option_with_db(database_settings));
        let postgres_pool = Self { pg_pool };
        Ok(postgres_pool)
    }

    #[allow(refining_impl_trait)]
    fn connect_option_without_db(database_settings: &DatabaseSettings) -> PgConnectOptions {
        let ssl_mod = if database_settings.require_ssl {
            PgSslMode::Require
        } else {
            // 암호화된 커넥션을 시도한다.
            // 실패하면 암호화되지 않은 커넥션을 사용한다.
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&database_settings.host)
            .username(&database_settings.username)
            .password(database_settings.password.expose_secret())
            .port(database_settings.port)
            .ssl_mode(ssl_mod)
    }

    #[allow(refining_impl_trait)]
    fn connect_option_with_db(database_settings: &DatabaseSettings) -> PgConnectOptions {
        Self::connect_option_without_db(database_settings)
            .database(&database_settings.database_name)
        // ``.log_statements`은 저자의 예시 코드에도 보이지 않는다.
        // 노이즈를 줄이려고 INFO를 TRACE로 변경하는 것이 이해가 되지 않는다.
    }

    async fn insert_subscriber(
        &self,
        new_subscriber: &NewSubscriber,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let id = uuid::Uuid::new_v4();
        let subscribed_at = Utc::now();
        let email = &new_subscriber.email;
        // 이제 `as_ref`를 사용한다.
        let name = new_subscriber.name.as_ref();
        pg_insert_subscriptions(&self.pg_pool, id, email, name, subscribed_at).await
    }
}

impl Deref for PostgresPool {
    type Target = PgPool;
    fn deref(&self) -> &Self::Target {
        &self.pg_pool
    }
}
