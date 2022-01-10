use async_trait::async_trait;
use mongodb::{bson::DateTime, Database};
use serde::{Deserialize, Serialize};

#[cfg(test)]
use mockall::*;
use tokio::sync::Mutex;

use crate::domain::user::entities::User;

pub enum FindOneError {
    NotFound,
    Unknown,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Repository: Send + Sync {
    async fn find_one(&self) -> Result<User, FindOneError>;
}

#[derive(Deserialize, Serialize)]
struct UserDocument {
    _id: String,
    username: String,
    password: String,
    created_at: DateTime,
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

    #[cfg(test)]
    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

#[async_trait]
impl Repository for MongoRepository {
    async fn find_one(&self) -> Result<User, FindOneError> {
        if self.error {
            return Err(FindOneError::Unknown);
        }

        let unlocked_database = self.database.lock().await;

        let results = unlocked_database
            .collection::<UserDocument>(self.collection.as_str())
            .find_one(None, None)
            .await;

        match results {
            Ok(Some(doc)) => Ok(User {
                email: doc.username,
                password: doc.password,
            }),
            Ok(None) => Err(FindOneError::NotFound),
            Err(_) => Err(FindOneError::Unknown),
        }
    }
}
