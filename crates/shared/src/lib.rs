pub mod error;
pub mod auth;
pub mod config;

// Re-export các kiểu dữ liệu dùng chung (VD: Money, Pagination)
pub use error::AppError;