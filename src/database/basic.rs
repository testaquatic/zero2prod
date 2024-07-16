use std::ops::Deref;

use sqlx::{ConnectOptions, Database};

use crate::{configuration::DatabaseSettings, domain::NewSubscriber};

/// 데이터베이스 변경을 편하게 하기 위한 트레이트
#[trait_variant::make()]
pub trait Zero2ProdDatabase: Deref<Target = sqlx::Pool<Self::DB>> {
    type DB: sqlx::Database;
    type ConnectOutput: Zero2ProdDatabase<DB = Self::DB>;

    /// DB에 연결한다.
    async fn connect(
        database_settings: &DatabaseSettings,
    ) -> Result<Self::ConnectOutput, sqlx::Error>;

    /// 구독자를 DB에 추가한다.
    async fn insert_subscriber(
        &self,
        new_subscriber: &NewSubscriber,
    ) -> Result<<Self::DB as sqlx::Database>::QueryResult, sqlx::Error>;

    fn connect_option_without_db(
        database_settings: &DatabaseSettings,
    ) -> impl ConnectOptions<Connection = <Self::DB as Database>::Connection>;

    fn connect_option_with_db(
        database_settings: &DatabaseSettings,
    ) -> impl ConnectOptions<Connection = <Self::DB as Database>::Connection>;
}
