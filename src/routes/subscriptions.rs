use actix_web::{web, HttpResponse};

use crate::{
    configuration::DefaultDBPool,
    database::basic::Zero2ProdDatabase,
    domain::{NewSubscriber, SubscriberName},
};

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
pub async fn subscribe(
    form: web::Form<FormData>,
    // 애플리케이션 상태에서 커넥션을 꺼낸다.
    pool: web::Data<DefaultDBPool>,
) -> HttpResponse {
    // `web::Form`은 `FormData`의 래퍼다.
    // `form.0`을 사용하면 기반 `FormData`에 접근할 수 있다.
    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        // name이 유효하지 않으면 400을 빠르게 반환한다.
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name,
    };
    // `Result`는 `Ok`와 `Err`라는 두 개의 변형을 갖는다.
    // 첫번째는 성공, 두 번째는 실패를 의미한다.
    //  `match` 구문을 사용해서 결과에 따라 무엇을 수행할지 선택한다.
    match pool.insert_subscriber(&new_subscriber).await {
        Err(e) => {
            // 이 오류 로그는 `query_span` 밖으로 떨어진다.
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
        Ok(_) => HttpResponse::Ok().finish(),
    }
}
