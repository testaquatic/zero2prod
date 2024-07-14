use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::{configuration::Settings, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // 이전에 있던 `env_logger` 행을 제거했다.

    // 모든 `log`의 이벤트를 구독자에게 리다이렉팅 한다.
    LogTracer::init().expect("Failed to set logger.");

    // RUST_LOG 환경 변수가 설정되어 있지 않으면 info레벨 및 그 이상의 모든 span을 출력한다.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    // 포맷이 적용된 span들을 stdout으로 출력한다.
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    // `with` 메서드는 `SubscriberExt`에서 제공한다.
    // `SubscriberExt`는 `Subscriber`의 확장 트레이트이며, `tracing_subscriber`에 의해 노출된다.
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    // 애플리케이션에서 `set_global_default`를 사용해서 span을 처리하기 위해 어떤 subscriber를 사용해야 하는지 지정할 수 있다.
    set_global_default(subscriber).expect("Failed to set subscriber.");

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
