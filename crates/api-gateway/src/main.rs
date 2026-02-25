use std::sync::Arc;
use sqlx::PgPool;
use axum::{Router};
use identify::infrastructure::persistence::postgres::PostgresUserRepository;
use identify::routes::{init, IdentityState};
use identify::usecase::auth::AuthUsecase;
use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthUsecase>,
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

    // 3. Khởi tạo Layered Architecture cho Identity Module
    // Repo (Infra) -> Usecase (Logic)
    let user_repo = Arc::new(PostgresUserRepository::new(Arc::new(pool.clone())));
    let auth_usecase = Arc::new(AuthUsecase::new(user_repo));

    // 4. Đưa vào AppState
    let state = AppState {
        auth_service: auth_usecase,
    };

    // 5. Compose Routes (Gộp các module lại)
    let app = Router::new()
        .nest("/api/v1/auth", init()) // Route đăng ký/đăng nhập
        .with_state(state);

    // 6. Start Server
    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("🚀 Yame Ecommerce Core started at {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}