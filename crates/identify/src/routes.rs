use crate::dto::auth::{LoginRequest, LoginResponse, RegisterRequest};
use axum::{extract::State, routing::post, Json, Router};
use shared::AppError;
use std::sync::Arc;
use crate::usecase::auth::AuthUsecase;

pub trait IdentityState: Send + Sync {
    fn auth_service(&self) -> Arc<AuthUsecase>;
}

pub fn init<S>() -> Router<S>
where
    S: IdentityState + Clone + Send + Sync + 'static
{
    Router::new()
        .route("/register", post(register_handler::<S>))
        .route("/login", post(login_handler::<S>))
}

async fn register_handler<S>(
    State(state): State<S>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, AppError>
where
    S: IdentityState
{
    if payload.password != payload.password_confirm {
        return Err(AppError::Validation("Passwords do not match".into()));
    }
    let auth_svc = state.auth_service();
    auth_svc.register(payload.email, payload.password).await?;
    Ok(Json(serde_json::json!({ "status": "ok" })))
}

async fn login_handler<S>(
    State(state): State<S>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError>
where
    S: IdentityState
{
    let auth_svc = state.auth_service();
    let token = auth_svc.login(&payload.email, &payload.password).await?;
    Ok(Json(LoginResponse { token }))
}
