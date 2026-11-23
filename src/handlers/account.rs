use crate::{
    error::ServiceError,
    models::{Account, AdjustBalancePayload, CreateAccountPayload},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_account(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateAccountPayload>,
) -> Result<impl IntoResponse, ServiceError> {
    let clean_username = payload.username.trim();
    if clean_username.is_empty() {
        return Err(ServiceError::BadRequest(
            "Username cannot be empty".to_string(),
        ));
    }

    let initial_balance = payload.initial_balance.unwrap_or(0);
    if initial_balance < 0 {
        return Err(ServiceError::BadRequest(
            "Initial balance cannot be negative".to_string(),
        ));
    }

    let account = sqlx::query_as::<_, Account>(
        r#"
        INSERT INTO accounts (id, username, balance, created_at, updated_at)
        VALUES ($1, $2, $3, NOW(), NOW())
        RETURNING id, username, balance, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(clean_username)
    .bind(initial_balance)
    .fetch_one(pool.as_ref())
    .await?;

    Ok((StatusCode::CREATED, Json(account)))
}

pub async fn get_account(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ServiceError> {
    let account = sqlx::query_as::<_, Account>(
        r#"
        SELECT id, username, balance, created_at, updated_at
        FROM accounts
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool.as_ref())
    .await?
    .ok_or_else(|| ServiceError::NotFound(format!("Account {id} not found")))?;

    Ok((StatusCode::OK, Json(account)))
}

pub async fn adjust_balance(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<AdjustBalancePayload>,
) -> Result<impl IntoResponse, ServiceError> {
    let mut tx = pool.begin().await?;

    let current = sqlx::query_as::<_, Account>(
        r#"
        SELECT id, username, balance, created_at, updated_at
        FROM accounts
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| ServiceError::NotFound(format!("Account {id} not found")))?;

    let new_balance = current.balance + payload.amount;
    if new_balance < 0 {
        return Err(ServiceError::BadRequest(
            "Insufficient funds for transaction".to_string(),
        ));
    }

    let updated = sqlx::query_as::<_, Account>(
        r#"
        UPDATE accounts
        SET balance = $1, updated_at = NOW()
        WHERE id = $2
        RETURNING id, username, balance, created_at, updated_at
        "#,
    )
    .bind(new_balance)
    .bind(id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok((StatusCode::OK, Json(updated)))
}
