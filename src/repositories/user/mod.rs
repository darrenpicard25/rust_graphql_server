pub mod adapter;

use async_trait::async_trait;

#[cfg(test)]
use mockall::*;
use mongodb::Database;
use tokio::sync::Mutex;

use crate::domain::user::entities::User;

pub struct CreateInput {
    pub email: String,
    pub password: String,
}

pub enum CreateError {
    Unknown,
}

pub enum FindByIdError {
    InvalidId,
    NotFound,
    Unknown,
}

pub enum FindOneByEmailError {
    Unknown,
}

pub struct MongoRepository {
    database: Mutex<Database>,
    collection: String,
    error: bool,
}

impl MongoRepository {
    pub fn new(db: Database) -> Self {
        Self {
            error: false,
            database: Mutex::new(db),
            collection: "user".to_string(),
        }
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Repository: Send + Sync {
    async fn find_by_id(&self, id: String) -> Result<User, FindByIdError>;
    async fn find_one_by_email(&self, email: String) -> Result<Option<User>, FindOneByEmailError>;
    async fn create(&self, input: CreateInput) -> Result<User, CreateError>;
}
