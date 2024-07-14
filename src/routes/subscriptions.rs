use actix_web::{web, HttpResponse};
use chrono::Utc;
use tracing::Instrument;
use uuid::Uuid;

use crate::{configuration::DBPool, database::basic::Zero2ProdDatabase};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// 단순하게 시작하자.
// 항산 200 OK를 반환한다.
pub async fn subscribe(
    form: web::Form<FormData>,
    // 애플리케이션 상태에서 커넥션을 꺼낸다.
    pool: web::Data<DBPool>,
) -> HttpResponse {
    // 여기에서는 `println`/`print`와 동일한 보간 구문을 사용한다.
    // 무작위 고유 식별자를 생성하자.
    let request_id = Uuid::new_v4();
    // Spans는 log와 같이 연관 레벨을 갖는다.
    // `info_span`은 info 레벨의 span을 생성한다.
    let request_span = tracing::info_span!("Adding a new subscriber.", %request_id, subscriber_email = %form.email, subscriber_name = %form.name);
    // async 함수에서 `enter`를 사용하면 그대로 재난이 발생한다.
    // 지금은 잠시 참아주되 집에서는 절대 하지 말자
    let _request_span_guard = request_span.enter();

    // query_span에 대해 `enter`를 호출하지 않는다.
    // `.instrument` 쿼리  퓨처 수명 주기 안에서 적절한 시점에 이를 관리한다.
    let query_span = tracing::info_span!("Saving new subscriber details in the database.");

    // `Result`는 `Ok`와 `Err`라는 두 개의 변형을 갖는다.
    // 첫번째는 성공, 두 번째는 실패를 의미한다.
    //  `match` 구문을 사용해서 결과에 따라 무엇을 수행할지 선택한다.
    match pool
        .insert_subscriptions(Uuid::new_v4(), &form.email, &form.name, Utc::now())
        // 먼저 instrument를 붙인 뒤, 대기한다.
        .instrument(query_span)
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
