use async_trait::async_trait;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Database,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[cfg(test)]
use mockall::*;
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

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Repository: Send + Sync {
    async fn find_by_id(&self, id: String) -> Result<User, FindByIdError>;
    async fn find_one_by_email(&self, email: String) -> Result<Option<User>, FindOneByEmailError>;
    async fn create(&self, input: CreateInput) -> Result<User, CreateError>;
}

#[derive(Deserialize, Serialize, Debug)]
struct UserDocument {
    _id: ObjectId,
    email: String,
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
}

#[async_trait]
impl Repository for MongoRepository {
    async fn find_by_id(&self, id: String) -> Result<User, FindByIdError> {
        if self.error {
            return Err(FindByIdError::Unknown);
        }

        let unlocked_database = self.database.lock().await;

        let id = match ObjectId::parse_str(id) {
            Ok(id) => id,
            Err(_) => return Err(FindByIdError::InvalidId),
        };

        let results = unlocked_database
            .collection::<UserDocument>(self.collection.as_str())
            .find_one(Some(doc! { "_id": id }), None)
            .await;

        match results {
            Ok(Some(doc)) => Ok(User {
                id: doc._id.to_hex(),
                email: doc.email,
                password: doc.password,
            }),
            Ok(None) => Err(FindByIdError::NotFound),
            Err(err) => {
                println!("Error In find_by_id: {:?}", err);
                Err(FindByIdError::Unknown)
            }
        }
    }

    async fn find_one_by_email(&self, email: String) -> Result<Option<User>, FindOneByEmailError> {
        if self.error {
            return Err(FindOneByEmailError::Unknown);
        }

        let unlocked_database = self.database.lock().await;

        let results = unlocked_database
            .collection::<UserDocument>(self.collection.as_str())
            .find_one(Some(doc! { "email": email }), None)
            .await;

        println!("Doc: {:?}", results);

        match results {
            Ok(Some(doc)) => Ok(Some(User {
                id: doc._id.to_hex(),
                email: doc.email,
                password: doc.password,
            })),
            Ok(None) => Ok(None),
            Err(_) => Err(FindOneByEmailError::Unknown),
        }
    }

    async fn create(&self, input: CreateInput) -> Result<User, CreateError> {
        if self.error {
            return Err(CreateError::Unknown);
        }

        let unlocked_database = self.database.lock().await;
        let now = SystemTime::now();

        let new_doc = doc! {
            "email": input.email.clone(),
            "password": input.password.clone(),
            "created_at": DateTime::from_system_time(now)
        };

        let results = unlocked_database
            .collection(self.collection.as_str())
            .insert_one(new_doc, None)
            .await;

        match results {
            Ok(insert_result) => Ok(User {
                id: insert_result.inserted_id.as_object_id().unwrap().to_hex(),
                email: input.email,
                password: input.password,
            }),
            Err(_) => Err(CreateError::Unknown),
        }
    }
}
