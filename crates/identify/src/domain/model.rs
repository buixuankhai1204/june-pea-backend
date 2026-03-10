use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            role: "customer".to_string(),
        }
    }
}