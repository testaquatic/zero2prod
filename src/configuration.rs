use secrecy::{ExposeSecret, Secret};

use crate::database::{basic::Zero2ProdDatabase, postgres::pool::PostgresPool};

pub type DBPool = PostgresPool;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    // 혹시 모를 로깅에 대비해서 `Secret<T>`를 사용한다.
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

impl Settings {
    pub fn get_configuration() -> Result<Self, config::ConfigError> {
        let base_path =
            std::env::current_dir().expect("Failed to determine the current directory.");
        let configuration_directory = base_path.join("configuration");

        // 실행 환경을 식별한다.
        // 지정되지 않으면 `local`로 기본 설정한다.
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or("local".into())
            .as_str()
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT.");
        let environment_filename = format!("{}.json", environment.as_str());

        // 구성 읽기를 초기화한다.
        let settings = config::Config::builder()
            // `configuration.json`이라는 파일로부터 구성값을 추가한다.
            .add_source(config::File::from(
                configuration_directory.join("base.json"),
            ))
            .add_source(config::File::from(
                configuration_directory.join(environment_filename),
            ))
            // 환경 변수로부터 설정에 추가한다.
            // APP, `__` 접두사를 붙인다.
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        // 읽은 구성값을 Settings 타입으로 변환한다.
        settings.try_deserialize()
    }
}

/// 애플리케이션이 사용할 수 있는 런타임 환경
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<&str> for Environment {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either 'local' or 'production'.",
                other
            )),
        }
    }
}

impl DatabaseSettings {
    // 비밀번호가 포함되어 있으므로 pub를 붙이지 않고 내부에서만 사용한다.
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            &self.username,
            &self.password.expose_secret(),
            &self.host,
            &self.port,
            &self.database_name
        )
    }

    pub async fn connect(&self) -> Result<DBPool, sqlx::Error> {
        DBPool::connect(self).await
    }
}

impl ApplicationSettings {
    pub async fn get_listener(&self) -> Result<tokio::net::TcpListener, std::io::Error> {
        tokio::net::TcpListener::bind(&format!("{}:{}", &self.host, &self.port)).await
    }
}
