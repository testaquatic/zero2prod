//! Postgres의 쿼리

use sqlx::{postgres::PgQueryResult, PgExecutor};

/// 구독자를 DB에 추가한다.
#[tracing::instrument(name = "Saving new subscriber details in the database.", skip_all)]
pub async fn pg_insert_subscriptions(
    executor: impl PgExecutor<'_>,
    id: uuid::Uuid,
    email: &str,
    name: &str,
    subscribed_at: chrono::DateTime<chrono::Utc>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4);
        "#,
        id,
        email,
        name,
        subscribed_at
    )
    .execute(executor)
    .await
}
