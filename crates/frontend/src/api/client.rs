use gloo_net::http::{Request, RequestBuilder};
use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use super::types::ApiErrorBody;

const TOKEN_KEY: &str = "auth_token";

fn base_url() -> String {
    web_sys::window()
        .and_then(|w| w.location().origin().ok())
        .map(|origin| {
            if origin.contains("127.0.0.1") || origin.contains("localhost") {
                "http://127.0.0.1:3000".to_string()
            } else {
                origin
            }
        })
        .unwrap_or_else(|| "http://127.0.0.1:3000".to_string())
}

#[derive(Error, Debug, Clone)]
pub enum ApiError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Network error: {0}")]
    Network(String),
}

impl ApiError {
    pub fn user_message(&self) -> &str {
        match self {
            Self::Unauthorized(m) => m,
            Self::NotFound(m) => m,
            Self::Validation(m) => m,
            Self::Conflict(m) => m,
            Self::Server(_) => "Something went wrong. Please try again.",
            Self::Network(_) => "Could not connect to server.",
        }
    }
}

pub fn get_token() -> Option<String> {
    LocalStorage::get::<String>(TOKEN_KEY).ok()
}

pub fn set_token(token: &str) {
    let _ = LocalStorage::set(TOKEN_KEY, token.to_string());
}

pub fn clear_token() {
    LocalStorage::delete(TOKEN_KEY);
}

fn apply_headers(req: RequestBuilder) -> RequestBuilder {
    let req = req
        .header("Content-Type", "application/json")
        .header("Accept", "application/json");

    if let Some(token) = get_token() {
        req.header("Authorization", &format!("Bearer {}", token))
    } else {
        req
    }
}

async fn handle_response<T: DeserializeOwned>(
    resp: gloo_net::http::Response,
) -> Result<T, ApiError> {
    let status = resp.status();

    if (200..300).contains(&status) {
        resp.json::<T>()
            .await
            .map_err(|e| ApiError::Network(format!("Failed to parse response: {}", e)))
    } else {
        let body = resp
            .json::<ApiErrorBody>()
            .await
            .map(|b| b.error)
            .unwrap_or_else(|_| "Unknown error".into());

        Err(match status {
            401 => ApiError::Unauthorized(body),
            404 => ApiError::NotFound(body),
            400 => ApiError::Validation(body),
            409 => ApiError::Conflict(body),
            _ => ApiError::Server(body),
        })
    }
}

pub async fn get<T: DeserializeOwned>(path: &str) -> Result<T, ApiError> {
    let url = format!("{}{}", base_url(), path);
    let req = apply_headers(Request::get(&url));
    let resp = req
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn post<T: DeserializeOwned, B: Serialize>(path: &str, body: &B) -> Result<T, ApiError> {
    let url = format!("{}{}", base_url(), path);
    let builder = apply_headers(Request::post(&url));
    let request = builder
        .json(body)
        .map_err(|e| ApiError::Network(e.to_string()))?;
    let resp = request
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn delete<T: DeserializeOwned>(path: &str) -> Result<T, ApiError> {
    let url = format!("{}{}", base_url(), path);
    let req = apply_headers(Request::delete(&url));
    let resp = req
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}

pub async fn patch<T: DeserializeOwned>(path: &str) -> Result<T, ApiError> {
    let url = format!("{}{}", base_url(), path);
    let req = apply_headers(Request::patch(&url));
    let resp = req
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    handle_response(resp).await
}
