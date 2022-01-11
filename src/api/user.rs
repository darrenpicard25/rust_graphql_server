use std::sync::Arc;

use crate::{
    domain::user::{find_one, register},
    repositories::user::MongoRepository,
};

use async_graphql::{Context, Error, Object, Result, SimpleObject};

#[derive(SimpleObject)]
struct User {
    id: String,
    email: String,
}

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutations;

#[Object]
impl UserQuery {
    async fn user(&self, ctx: &Context<'_>, id: String) -> Result<User> {
        let repo = ctx.data::<Arc<MongoRepository>>().unwrap();

        let result = find_one::execute(repo.clone(), id).await;

        match result {
            Ok(user) => Ok(User {
                id: user.id,
                email: user.email,
            }),
            Err(find_one::FindOneError::NotFound) => Err(Error::new("Not Found")),
            Err(find_one::FindOneError::InvalidId) => Err(Error::new("Invalid Input")),
            Err(find_one::FindOneError::Unknown) => Err(Error::new("Unknown")),
        }
    }
}

#[Object]
impl UserMutations {
    async fn register(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<User> {
        let repo = ctx.data::<Arc<MongoRepository>>().unwrap();

        let result = register::execute(
            repo.clone(),
            register::Input {
                email: username,
                password,
            },
        )
        .await;

        match result {
            Ok(user) => Ok(User {
                id: user.id,
                email: user.email,
            }),
            Err(register::RegisterError::AlreadyExists) => Err(Error::new("Already Exists")),
            Err(register::RegisterError::Unknown) => Err(Error::new("Unknown Error")),
        }
    }
}
