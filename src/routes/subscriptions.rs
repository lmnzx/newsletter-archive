use {
    crate::email_client::send_mail,
    axum::{
        extract::{Form, State},
        http::StatusCode,
        response::IntoResponse,
    },
    chrono::Utc,
    sqlx::postgres::PgPool,
    uuid::Uuid,
};

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(
    State(db_pool): State<PgPool>,
    Form(form_data): Form<FormData>,
) -> impl IntoResponse {
    tracing::debug!("{:#?}", form_data);
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form_data.email,
        form_data.name,
        Utc::now()
    )
    .execute(&db_pool)
    .await
    .expect("Database error");
    send_mail(form_data.email, form_data.name);
    (StatusCode::OK, "subscriptions endpoint")
}
