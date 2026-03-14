use std::sync::Arc;
use crate::domain::user_repository::UserRepository;
use crate::domain::model::User;
use shared::error::AppError;

pub struct ListUsersUsecase {
    repo: Arc<dyn UserRepository>,
}

impl ListUsersUsecase {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> Result<Vec<User>, AppError> {
        self.repo.list_users().await
    }
}
