use axum::extract::{Path, State};
use axum::routing::{get, post, patch};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use shared::AppError;
use std::sync::Arc;

use crate::domain::model::Coupon;
use crate::usecase::create_coupon::CreateCouponUsecase;
use crate::usecase::validate_coupon::ValidateCouponUsecase;
use crate::usecase::list_coupons::ListCouponsUsecase;
use crate::usecase::deactivate_coupon::DeactivateCouponUsecase;

#[derive(Clone)]
pub struct MarketingUsecase {
    create_coupon: Arc<CreateCouponUsecase>,
    validate_coupon: Arc<ValidateCouponUsecase>,
    list_coupons: Arc<ListCouponsUsecase>,
    deactivate_coupon: Arc<DeactivateCouponUsecase>,
}

impl MarketingUsecase {
    pub fn new(
        create_coupon: Arc<CreateCouponUsecase>,
        validate_coupon: Arc<ValidateCouponUsecase>,
        list_coupons: Arc<ListCouponsUsecase>,
        deactivate_coupon: Arc<DeactivateCouponUsecase>,
    ) -> Self {
        Self {
            create_coupon,
            validate_coupon,
            list_coupons,
            deactivate_coupon,
        }
    }

    pub fn create_coupon(&self) -> Arc<CreateCouponUsecase> {
        self.create_coupon.clone()
    }

    pub fn validate_coupon(&self) -> Arc<ValidateCouponUsecase> {
        self.validate_coupon.clone()
    }

    pub fn list_coupons(&self) -> Arc<ListCouponsUsecase> {
        self.list_coupons.clone()
    }

    pub fn deactivate_coupon(&self) -> Arc<DeactivateCouponUsecase> {
        self.deactivate_coupon.clone()
    }
}

pub fn init() -> Router<MarketingUsecase> {
    Router::new()
        .route("/coupons", post(create_coupon_handler))
        .route("/coupons", get(list_coupons_handler))
        .route("/coupons/:code/validate", get(validate_coupon_handler))
        .route("/coupons/:code/deactivate", patch(deactivate_coupon_handler))
}

// --- Request / Response types ---

#[derive(Debug, Deserialize)]
struct CreateCouponRequest {
    code: String,
    discount_amount: i64,
    max_uses: i32,
}

#[derive(Debug, Serialize)]
struct ValidateCouponResponse {
    is_valid: bool,
    discount_amount: i64,
}

// --- Handlers ---

async fn create_coupon_handler(
    State(state): State<MarketingUsecase>,
    Json(body): Json<CreateCouponRequest>,
) -> Result<Json<Coupon>, AppError> {
    let usecase = state.create_coupon();
    let coupon = usecase
        .execute(body.code, body.discount_amount, body.max_uses)
        .await?;
    Ok(Json(coupon))
}

async fn validate_coupon_handler(
    State(state): State<MarketingUsecase>,
    Path(code): Path<String>,
) -> Result<Json<ValidateCouponResponse>, AppError> {
    let usecase = state.validate_coupon();
    let coupon = usecase.execute(&code).await?;
    
    Ok(Json(ValidateCouponResponse {
        is_valid: true,
        discount_amount: coupon.discount_amount,
    }))
}

async fn list_coupons_handler(
    State(state): State<MarketingUsecase>,
) -> Result<Json<Vec<Coupon>>, AppError> {
    let usecase = state.list_coupons();
    let coupons = usecase.execute().await?;
    Ok(Json(coupons))
}

async fn deactivate_coupon_handler(
    State(state): State<MarketingUsecase>,
    Path(code): Path<String>,
) -> Result<Json<bool>, AppError> {
    let usecase = state.deactivate_coupon();
    usecase.execute(&code).await?;
    Ok(Json(true))
}
