pub mod error;
pub mod auth;
pub mod config;
pub mod database;
pub mod infrastructure;

// Re-export các kiểu dữ liệu dùng chung (VD: Money, Pagination)
pub use error::AppError;