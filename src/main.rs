use anyhow::Context;
use zero2prod::{
    configuration::Settings,
    startup::new_server,
    telemetry::{get_tracing_subscriber, init_tracing_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let tracing_subscriber =
        get_tracing_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_tracing_subscriber(tracing_subscriber);
    // 구성을 읽을 수 없으면 패닉에 빠진다.
    let configuration = Settings::get_configuration().expect("Failed to read configuration.");
    // `configuration`에 두번이나 접근할 필요 없이 더 깔끔하게 정리할 수 있지 않을까?
    let listener = configuration
        .application
        .get_listener()
        .await
        .context("Failed to get listen from configuration.")?;
    let pool = configuration
        .database
        .connect()
        .await
        .context("Failed to connect to Postgres.")?;
    let server = new_server(listener, pool).context("Failed to make new server.")?;
    server.await.context("Failed to run server.")
}
