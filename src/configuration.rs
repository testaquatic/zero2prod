use crate::database::{basic::Zero2ProdDatabase, postgres::pool::PostgresPool};

pub type DBPool = PostgresPool;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl Settings {
    pub fn get_configuration() -> Result<Self, config::ConfigError> {
        // 구성 읽기를 초기화한다.
        let settings = config::Config::builder()
            // `configuration.json`이라는 파일로부터 구성값을 추가한다.
            .add_source(config::File::new(
                "configuration.json",
                config::FileFormat::Json,
            ))
            .build()?;
        // 읽은 구성값을 Settings 타입으로 변환한다.
        settings.try_deserialize()
    }

    pub async fn get_listener(&self) -> Result<tokio::net::TcpListener, std::io::Error> {
        tokio::net::TcpListener::bind(&format!("127.0.0.1:{}", &self.application_port)).await
    }
}

impl DatabaseSettings {
    fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port,
        )
    }

    fn connection_string(&self) -> String {
        format!(
            "{}/{}",
            &self.connection_string_without_db(),
            &self.database_name
        )
    }

    pub async fn connect(&self) -> Result<DBPool, sqlx::Error> {
        DBPool::connect(&self.connection_string()).await
    }

    pub async fn connect_without_db(&self) -> Result<DBPool, sqlx::Error> {
        DBPool::connect(&self.connection_string_without_db()).await
    }
}
