use actix_web::{web, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

use crate::{configuration::DefaultDBPool, database::basic::Zero2ProdDatabase};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip_all,
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
// 단순하게 시작하자.
// 항산 200 OK를 반환한다.
pub async fn subscribe(
    form: web::Form<FormData>,
    // 애플리케이션 상태에서 커넥션을 꺼낸다.
    pool: web::Data<DefaultDBPool>,
) -> HttpResponse {
    // `Result`는 `Ok`와 `Err`라는 두 개의 변형을 갖는다.
    // 첫번째는 성공, 두 번째는 실패를 의미한다.
    //  `match` 구문을 사용해서 결과에 따라 무엇을 수행할지 선택한다.
    match pool
        .insert_subscriptions(Uuid::new_v4(), &form.email, &form.name, Utc::now())
        .await
    {
        Err(e) => {
            // 이 오류 로그는 `query_span` 밖으로 떨어진다.
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(_) => HttpResponse::Ok().finish(),
    }
    // `_request_span_guard`는 해당 span에서 이탈하는 시점인 `subscribe`의 끝에서 해제된다.
}
