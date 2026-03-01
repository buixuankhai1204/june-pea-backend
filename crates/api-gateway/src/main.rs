use std::sync::Arc;
use sqlx::PgPool;
use axum::{Router};
use identify::infrastructure::persistence::postgres::PostgresUserRepository;
use identify::routes::{init, IdentityState};
use identify::usecase::auth::AuthUsecase;
use std::env;
use dotenv::dotenv;
use catalog::infrastructure::cache::redis::RedisCatalogCache;
use catalog::infrastructure::persistence::postgres::PostgresCatalogRepository;
use catalog::routes::CatalogUsecase;
use inventory::routes::InventoryUsecase;

mod middleware;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthUsecase>,
    pub catalog_service: Arc<CatalogUsecase>, // Placeholder cho CatalogService
    pub inventory_usecase: Arc<InventoryUsecase>
}

impl IdentityState for AppState {
    fn auth_service(&self) -> Arc<AuthUsecase> {
        self.auth_service.clone()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Khởi tạo Logging
    tracing_subscriber::fmt::init();
    dotenv().ok();
    // 2. Kết nối Database
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    print!("Connecting to database at {}... ", db_url);
    let pool = PgPool::connect(&db_url).await?;
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://".to_string());
    print!("Connecting to Redis at {}... ", redis_url);

    let user_repo = Arc::new(PostgresUserRepository::new(Arc::new(pool.clone())));
    let catalog_repo = Arc::new(PostgresCatalogRepository::new(Arc::new(pool.clone()))); // Placeholder cho CatalogRepository
    let auth_usecases = Arc::new(AuthUsecase::new(user_repo));
    let redis = Arc::new(RedisCatalogCache::new(&redis_url).await?);
    let postgrese_unit_of_work = Arc::new(shared::infrastructure::postgres::PostgresUnitOfWork::new(pool.clone()));
    let inventory_usecases = Arc::new(inventory::routes::InventoryUsecase::new(
        Arc::new(inventory::infrastructure::persistence::postgres::PostgresInventoryRepository),
        postgrese_unit_of_work.clone(),
    ));

    let catalog_usecases = Arc::new(CatalogUsecase::new(
        catalog_repo,
        redis // Placeholder cho CatalogRepository
    ));
    // 4. Đưa vào AppState
    let state = AppState {
        auth_service: auth_usecases,
        catalog_service: catalog_usecases,
        inventory_usecase: inventory_usecases,
    };

    tracing::info!("Running database migrations...");
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Migration failed: {}", e))?;

    // 5. Compose Routes (Gộp các module lại)
    let app = Router::new()
        .nest("/api/v1/auth", init())
        .layer(axum::middleware::from_fn(middleware::auth::auth_middleware))
        .with_state(state);

    // 6. Start Server
    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("🚀 Yame Ecommerce Core started at {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}