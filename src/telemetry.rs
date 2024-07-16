//! 로그를 관리한다.
use std::sync::Once;

use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

/// 여러 레이어들을 하나의 `tracing`의 subscriber로 구성한다.
///
/// # 구현노트
///
/// `impl Subscriber`를 반환 타입으로 사용해서 반환된 subscriber의 실제 타입에 관한 설명을 피한다.
/// 반환된 subscriber를 `init_subscriber`로 나중에 전당하기 위해 명시적으로 `Send`이고 `Sync`임을 알려야 한다.
pub fn get_tracing_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // 이 "이상한" 구문은 higher-ranked trait bound이다.
    // 기본적으로 Sink가 모든 라이프타임 파라미터 `'a`에 대해 `MakeWriter` 트레이트를 구현한다는 것을 의미한다.
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // RUST_LOG 환경 변수가 설정되어 있지 않으면 info레벨 및 그 이상의 모든 span을 출력한다.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new(env_filter));
    // 포맷이 적용된 span들을 stdout으로 출력한다.
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    // `with` 메서드는 `SubscriberExt`에서 제공한다.
    // `SubscriberExt`는 `Subscriber`의 확장 트레이트이며, `tracing_subscriber`에 의해 노출된다.
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// subscriber를 글로벌 기본값으로 등록해서 span 데이터를 처리한다.
pub fn init_tracing_subscriber(tracing_subscriber: impl Subscriber + Send + Sync) {
    // `std::sync::Once`를 사용해서 `tracing` 스택이 한 번만 초기화되는 것을 보장한다.
    static INIT_TRACING_SUBSCRIBER: Once = Once::new();

    INIT_TRACING_SUBSCRIBER.call_once(|| {
        // `get_subscriber`의 출력을 `TEST_LOG`의 값에 기반해서 변수에 할당할 수 없다.
        // 왜냐하면 해당 sink는 `get_subscriber`에 의해 반환된 타입의 일부이고, 그들의 타입이 같지 않기 때문이다.
        // 이 상황을 회피할 수는 있지만 이 방법이 이후의 과정을 진행할 수 있는 가장 직관적인 방법이다.

        // 이전에 있던 `env_logger` 행을 제거했다.
        // 모든 `log`의 이벤트를 구독자에게 리다이렉팅 한다.
        LogTracer::init().expect("Failed to set logger.");
        // 애플리케이션에서 `set_global_default`를 사용해서 span을 처리하기 위해 어떤 subscriber를 사용해야 하는지 지정할 수 있다.
        set_global_default(tracing_subscriber).expect("Failed to set subscriber.");
    });
}
