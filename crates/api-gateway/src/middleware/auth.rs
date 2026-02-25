use axum::{
    extract::Request,
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use shared::auth::decode_token;
use shared::AppError;

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req.headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized).unwrap_or_default();


    let token = &auth_header[7..]; // Cắt bỏ "Bearer "
    let claims = decode_token(token)?;

    // Inject thông tin user vào Request extensions để các Module sau sử dụng
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}