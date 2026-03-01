use async_trait::async_trait;
use futures::future::BoxFuture;
use std::any::Any;
use crate::error::AppError;

/// The Bridge: Every DB transaction or connection must implement this.
pub trait DbExecutor: Send + Sync {
}

/// The Orchestrator: This trait is dyn-compatible.
#[async_trait]
pub trait UnitOfWork: Send + Sync {
    async fn run_atomic(
        &self,
        // Change: We wrap the function in an Arc or use a reference 
        // that matches the lifetime of the future.
        f: Box<dyn for<'a> FnOnce(&'a mut dyn DbExecutor) -> BoxFuture<'a, Result<(), AppError>> + Send>,
    ) -> Result<(), AppError>;
}