use actix_web::{dev::Server, web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::DBPool,
    routes::{greet, health_check, subscribe},
};

// `run`을 `public`으로 마크해야 한다.
// 번쩍번쩍 아름다운 새로운 서버
pub fn new_server(
    listener: tokio::net::TcpListener,
    pool: DBPool,
) -> Result<Server, std::io::Error> {
    // web::Data로 pool을 감싼다.
    // Arc 스마트 포인터로 요약된다.
    let pool = web::Data::new(pool);
    // 주변 환경으로부터 `connection`을 잡아낸다.
    let server = HttpServer::new(move || {
        App::new()
            // `App`에 대해 `wrap` 메서드를 사용해서 미들웨어들을 추가한다.
            // `Logger::default`를 대신한다.
            .wrap(TracingLogger::default())
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            // POST /subcriptions 요청에 대한 라우팅 테이블의 새 엔트리 포인트
            .route("/subscriptions", web::post().to(subscribe))
            // 커넥션을 애플리케이션 상태의 일부로 등록한다.
            // 포인터 사본을 얻어 애플리케이션 상태에 추가한다.
            .app_data(pool.clone())
    })
    .listen(listener.into_std()?)?
    .run();
    Ok(server)
}
