use sqlx::PgExecutor;

pub async fn pg_insert_subscriptions(
    executor: impl PgExecutor<'_>,
    id: uuid::Uuid,
    email: &str,
    name: &str,
    subscribed_at: chrono::DateTime<chrono::Utc>,
) -> Result<(), sqlx::Error> {
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
    .await?;
    Ok(())
}
