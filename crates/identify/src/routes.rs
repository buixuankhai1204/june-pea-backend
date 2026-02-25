use crate::dto::auth::RegisterRequest;
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
    Router::new().route("/register", post(register_handler::<S>))
}


async fn register_handler<S>(
    State(state): State<S>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, AppError>
where
    S: IdentityState
{
    let auth_svc = state.auth_service();
    auth_svc.register(payload.email, payload.password).await?;
    Ok(Json(serde_json::json!({ "status": "ok" })))
}
