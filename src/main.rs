use zero2prod::{
    configuration::Settings,
    startup::run,
    telemetry::{get_tracing_subscriber, init_tracing_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let tracing_subscriber =
        get_tracing_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_tracing_subscriber(tracing_subscriber);
    // 구성을 읽을 수 없으면 패닉에 빠진다.
    let configuration = Settings::get_configuration().expect("Failed to read configuration.");
    let pg_pool = configuration
        .database
        .connect()
        .await
        .expect("Failed to connect database.");
    // 하드 코딩했던 `8000`을 제거했다.
    // 해당 값은 세팅에서 얻는다.
    let listener = configuration.get_listener().await?;
    run(listener, pg_pool)?.await
}
