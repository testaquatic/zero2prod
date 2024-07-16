use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, Executor};
use std::sync::Once;
use tracing::Subscriber;
use uuid::Uuid;
use zero2prod::{
    configuration::Settings,
    startup::new_server,
    telemetry::{get_tracing_subscriber, init_tracing_subscriber},
};

/// `TEST_LOG` 값이 설정되어 있으면 `stdout`에 출력하는 tracing_subscriber를 생성한다.
/// 그렇지 않으면 버린다.
/// 한번만 초기화 된다.
fn init_test_tracing_subscriber() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let default_filter_level = "info".to_string();
        let tracing_subscriber_name = "test".to_string();
        let boxed_tracing_subscriber: Box<dyn Subscriber + Send + Sync> =
            if std::env::var("TEST_LOG").is_ok() {
                let tracing_subscriber = get_tracing_subscriber(
                    tracing_subscriber_name,
                    default_filter_level,
                    std::io::stdout,
                );
                Box::new(tracing_subscriber)
            } else {
                let tracing_subscriber = get_tracing_subscriber(
                    tracing_subscriber_name,
                    default_filter_level,
                    std::io::sink,
                );
                Box::new(tracing_subscriber)
            };
        init_tracing_subscriber(boxed_tracing_subscriber);
    })
}

pub struct TestApp {
    pub configuration: Settings,
}

impl TestApp {
    // 백그라운드에서 애플리케이션을 구동한다.
    // 이 함수는 이제 비동기이다.
    pub async fn spawn_app() -> Self {
        init_test_tracing_subscriber();

        // 설정을 읽어온다.
        let configuration = Settings::get_configuration().expect("Failed to read configuration.");
        let mut app = TestApp { configuration };

        // 데이터베이스를 설정한다.
        app.set_database().await;
        let db_pool = app
            .configuration
            .database
            .connect()
            .await
            .expect("Failed to set database.");

        // TcpListener를 설정한다.
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind random port.");
        app.configuration.application.host = "127.0.0.1".to_string();
        // OS가 할당한 포트 번호를 추출한다.
        app.configuration.application.port = listener.local_addr().unwrap().port();

        // 반짝반짝한 새 서버를 생성한다.
        let server = new_server(listener, db_pool).unwrap();

        // 서버를 백그라운드로 구동한다.
        // tokio::spawn은 생성된 퓨처에 대한 핸들을 반환한다.
        // 하지면 여기에서는 사용하지 않으므로 let을 바인딩하지 않는다.
        let _ = tokio::spawn(server);

        app
    }

    /// 데이터 베이스를 설정한다.
    pub async fn set_database(&mut self) {
        self.create_random_database().await;
        self.migrate_database().await;
    }

    /// 테스트를 위한 무작위 데이터베이스를 생성한다.
    async fn create_random_database(&mut self) {
        let database = &mut self.configuration.database;
        // 데이터베이스를 생성한다.
        database.database_name = Uuid::new_v4().to_string();
        let db_url = format!(
            "postgres://{}:{}@{}:{}",
            &database.username,
            &database.password.expose_secret(),
            &database.host,
            &database.port
        );
        let pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect(&db_url)
            .await
            .expect("Failed to connect to Postgres.");
        pool.execute(format!(r#"CREATE DATABASE "{}""#, database.database_name).as_str())
            .await
            .expect("Failed to create database.");
    }

    /// 데이터베이스를 마이그레이션 한다.
    async fn migrate_database(&self) {
        // 데이터베이스를 마이그레이션 한다.
        let pool = self
            .configuration
            .database
            .connect()
            .await
            .expect("Failed to connect to Postgres.");
        sqlx::migrate!("./migrations")
            .run(&*pool)
            .await
            .expect("Failed to migrate the database.");
    }

    fn http_address(&self) -> String {
        format!(
            "http://{}:{}",
            &self.configuration.application.host, &self.configuration.application.port
        )
    }

    pub fn subcriptions_url(&self) -> String {
        format!("{}/subscriptions", &self.http_address())
    }
}

// `tokio::test`는 테스팅에 있어서 `tokio::main`과 동등하다.
// `#[test]` 속성을 지정하는 수고를 덜 수 있다.
//
// `cargo expand --test health_check`을 사용해서 코드가 무엇을 생성하는지 확인할 수 있다.
#[tokio::test]
async fn health_check_works() {
    // 준비
    let app = TestApp::spawn_app().await;
    // `reqwest`를 사용해서 애플리케이션에 대한 HTTP 요청을 수행한다.
    let client = reqwest::Client::new();

    // 실행
    let response = client
        // 반환된 애플리케이션 주소를 사용한다.
        .get(&format!("{}/health_check", &app.http_address()))
        .send()
        .await
        .expect("Failed to execute request.");

    // 확인
    // 응답이 200 OK인지 확인한다.
    assert!(response.status().is_success());
    // 응답 본문의 길이가 0인지 확인한다.
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // 준비
    let app = TestApp::spawn_app().await;
    let client = reqwest::Client::new();

    // 실행
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(app.subcriptions_url())
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // 확인
    // 응답이 200 OK인지 확인한다.
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let db_pool = app.configuration.database.connect().await.unwrap();
    let saved = sqlx::query!(
        r#"
        SELECT email, name
        FROM subscriptions
        "#,
    )
    .fetch_one(&*db_pool)
    .await
    .expect("Failed to fetch save subcriptions.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // 준비
    let app = TestApp::spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_messages) in test_cases {
        // 실행
        let response = client
            .post(&app.subcriptions_url())
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // 확인
        // 잘못된 바디가 전송됐으므로 `BAD_REQUEST` 응답을 받아야 한다.
        assert_eq!(
            response.status(),
            reqwest::StatusCode::BAD_REQUEST,
            // 테스트 실패시 출력할 커스터마이즈된 추가 오류 메시지
            "The API did not fail with 400 BAD_REQUEST when the payload was {}.",
            error_messages
        );
    }
}
