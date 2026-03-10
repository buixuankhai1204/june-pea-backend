use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use shared::AppError;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::model::{NewOrderItem, Order};
use crate::usecase::cancel_order::CancelOrderUsecase;
use crate::usecase::get_order::GetOrderUsecase;
use crate::usecase::list_orders::ListOrdersUsecase;
use crate::usecase::place_order::PlaceOrderUsecase;

#[derive(Clone)]
pub struct OrderingUsecase {
    place_order: Arc<PlaceOrderUsecase>,
    cancel_order: Arc<CancelOrderUsecase>,
    get_order: Arc<GetOrderUsecase>,
    list_orders: Arc<ListOrdersUsecase>,
}

impl OrderingUsecase {
    pub fn new(
        place_order: Arc<PlaceOrderUsecase>,
        cancel_order: Arc<CancelOrderUsecase>,
        get_order: Arc<GetOrderUsecase>,
        list_orders: Arc<ListOrdersUsecase>,
    ) -> Self {
        Self {
            place_order,
            cancel_order,
            get_order,
            list_orders,
        }
    }

    pub fn place_order(&self) -> Arc<PlaceOrderUsecase> {
        self.place_order.clone()
    }

    pub fn cancel_order(&self) -> Arc<CancelOrderUsecase> {
        self.cancel_order.clone()
    }

    pub fn get_order(&self) -> Arc<GetOrderUsecase> {
        self.get_order.clone()
    }

    pub fn list_orders(&self) -> Arc<ListOrdersUsecase> {
        self.list_orders.clone()
    }
}

pub fn init() -> Router<OrderingUsecase> {
    Router::new()
        .route("/orders", post(place_order_handler))
        .route("/orders/:id", get(get_order_handler))
        .route("/orders/:id", delete(cancel_order_handler))
        .route("/orders/customer/:customer_id", get(list_orders_handler))
}

// --- Request / Response types ---

#[derive(Debug, Deserialize)]
struct PlaceOrderRequest {
    customer_id: Uuid,
    items: Vec<NewOrderItem>,
}

#[derive(Debug, Serialize)]
struct PlaceOrderResponse {
    order_id: Uuid,
}

// --- Handlers ---

async fn place_order_handler(
    State(state): State<OrderingUsecase>,
    Json(body): Json<PlaceOrderRequest>,
) -> Result<Json<PlaceOrderResponse>, AppError> {
    let usecase = state.place_order();
    let order_id = usecase.execute(body.customer_id, body.items).await?;
    Ok(Json(PlaceOrderResponse { order_id }))
}

async fn get_order_handler(
    State(state): State<OrderingUsecase>,
    Path(id): Path<Uuid>,
) -> Result<Json<Order>, AppError> {
    let usecase = state.get_order();
    let order = usecase.execute(id).await?;
    Ok(Json(order))
}

async fn cancel_order_handler(
    State(state): State<OrderingUsecase>,
    Path(id): Path<Uuid>,
) -> Result<Json<bool>, AppError> {
    let usecase = state.cancel_order();
    usecase.execute(id).await?;
    Ok(Json(true))
}

async fn list_orders_handler(
    State(state): State<OrderingUsecase>,
    Path(customer_id): Path<Uuid>,
) -> Result<Json<Vec<Order>>, AppError> {
    let usecase = state.list_orders();
    let orders = usecase.execute(customer_id).await?;
    Ok(Json(orders))
}
