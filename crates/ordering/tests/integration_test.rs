//! Integration tests for ordering usecases.
//! Uses an in-memory MockOrderRepository — no database required.

use async_trait::async_trait;
use futures::future::BoxFuture;
use ordering::domain::{
    model::{NewOrderItem, Order, OrderItem, OrderStatus},
    repository::OrderRepository,
};
use ordering::usecase::{cancel_order::CancelOrderUsecase, place_order::PlaceOrderUsecase};
use shared::{
    database::{DbExecutor, UnitOfWork},
    error::AppError,
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// ─────────────────────────────────────────────
// In-memory mock repository
// ─────────────────────────────────────────────

#[derive(Default)]
struct MockStore {
    orders: Vec<Order>,
}

struct MockOrderRepository {
    store: Arc<Mutex<MockStore>>,
}

impl MockOrderRepository {
    fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(MockStore::default())),
        }
    }
}

#[async_trait]
impl OrderRepository for MockOrderRepository {
    async fn create_order(
        &self,
        _exec: &mut dyn DbExecutor,
        order: &Order,
        _items: &[OrderItem],
    ) -> Result<(), AppError> {
        self.store.lock().unwrap().orders.push(order.clone());
        Ok(())
    }

    async fn get_order_by_id(
        &self,
        _exec: &mut dyn DbExecutor,
        id: Uuid,
    ) -> Result<Order, AppError> {
        self.store
            .lock()
            .unwrap()
            .orders
            .iter()
            .find(|o| o.id == id)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("Order {} not found", id)))
    }

    async fn update_order_status(
        &self,
        _exec: &mut dyn DbExecutor,
        id: Uuid,
        status: OrderStatus,
    ) -> Result<(), AppError> {
        let mut store = self.store.lock().unwrap();
        let order = store
            .orders
            .iter_mut()
            .find(|o| o.id == id)
            .ok_or_else(|| AppError::NotFound(format!("Order {} not found", id)))?;
        order.status = status;
        Ok(())
    }
}

// ─────────────────────────────────────────────
// No-op UnitOfWork: runs the closure directly without a real transaction
// ─────────────────────────────────────────────

struct NoOpExecutor;
impl DbExecutor for NoOpExecutor {}

struct NoOpUnitOfWork;

#[async_trait]
impl UnitOfWork for NoOpUnitOfWork {
    async fn run_atomic(
        &self,
        f: Box<
            dyn for<'a> FnOnce(&'a mut dyn DbExecutor) -> BoxFuture<'a, Result<(), AppError>>
                + Send,
        >,
    ) -> Result<(), AppError> {
        let mut exec = NoOpExecutor;
        f(&mut exec).await
    }
}

// ─────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────

fn make_repo() -> Arc<MockOrderRepository> {
    Arc::new(MockOrderRepository::new())
}

fn make_uow() -> Arc<NoOpUnitOfWork> {
    Arc::new(NoOpUnitOfWork)
}

fn item(quantity: i32) -> NewOrderItem {
    NewOrderItem {
        variant_id: Uuid::new_v4(),
        quantity,
        unit_price: 1000,
    }
}

// ─────────────────────────────────────────────
// PlaceOrder usecase tests
// ─────────────────────────────────────────────

#[tokio::test]
async fn place_order_stores_order_and_returns_id() {
    let repo = make_repo();
    let uow = make_uow();
    let usecase = PlaceOrderUsecase::new(repo.clone() as Arc<dyn OrderRepository>, uow);

    let customer = Uuid::new_v4();
    let order_id = usecase
        .execute(customer, vec![item(2), item(3)])
        .await
        .unwrap();

    let store = repo.store.lock().unwrap();
    assert_eq!(store.orders.len(), 1);
    assert_eq!(store.orders[0].id, order_id);
    assert_eq!(store.orders[0].customer_id, customer);
    assert_eq!(store.orders[0].status, OrderStatus::Pending);
    assert_eq!(store.orders[0].total, 5000);
}

#[tokio::test]
async fn place_order_with_empty_items_fails() {
    let usecase = PlaceOrderUsecase::new(make_repo() as Arc<dyn OrderRepository>, make_uow());
    let result = usecase.execute(Uuid::new_v4(), vec![]).await;
    assert!(matches!(result, Err(AppError::Validation(_))));
}

#[tokio::test]
async fn place_order_with_zero_quantity_fails() {
    let usecase = PlaceOrderUsecase::new(make_repo() as Arc<dyn OrderRepository>, make_uow());
    let result = usecase.execute(Uuid::new_v4(), vec![item(0)]).await;
    assert!(matches!(result, Err(AppError::Validation(_))));
}

// ─────────────────────────────────────────────
// CancelOrder usecase tests
// ─────────────────────────────────────────────

#[tokio::test]
async fn cancel_pending_order_sets_status_to_cancelled() {
    let repo = make_repo();
    let uow = make_uow();

    // Place first
    let place = PlaceOrderUsecase::new(repo.clone() as Arc<dyn OrderRepository>, uow.clone());
    let order_id = place.execute(Uuid::new_v4(), vec![item(1)]).await.unwrap();

    // Then cancel
    let cancel = CancelOrderUsecase::new(repo.clone() as Arc<dyn OrderRepository>, uow);
    cancel.execute(order_id).await.unwrap();

    let store = repo.store.lock().unwrap();
    assert_eq!(store.orders[0].status, OrderStatus::Cancelled);
}

#[tokio::test]
async fn cancel_already_cancelled_order_fails() {
    let repo = make_repo();
    let uow = make_uow();

    let place = PlaceOrderUsecase::new(repo.clone() as Arc<dyn OrderRepository>, uow.clone());
    let order_id = place.execute(Uuid::new_v4(), vec![item(1)]).await.unwrap();

    let cancel = CancelOrderUsecase::new(repo.clone() as Arc<dyn OrderRepository>, uow.clone());
    cancel.execute(order_id).await.unwrap();

    // Second cancel must fail
    let cancel2 = CancelOrderUsecase::new(repo.clone() as Arc<dyn OrderRepository>, uow);
    let result = cancel2.execute(order_id).await;
    assert!(matches!(result, Err(AppError::Validation(_))));
}

#[tokio::test]
async fn cancel_nonexistent_order_returns_not_found() {
    let cancel = CancelOrderUsecase::new(make_repo() as Arc<dyn OrderRepository>, make_uow());
    let result = cancel.execute(Uuid::new_v4()).await;
    assert!(matches!(result, Err(AppError::NotFound(_))));
}
