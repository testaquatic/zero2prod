use actix_web::{HttpResponse, Responder};

// `curl -v http://127.0.0.1:8000/health_check` => 200 Ok
// `/health_check`에 대해 `GET`요청을 받으면, 바디가 없는 200 OK 응답을 반환한다.
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
