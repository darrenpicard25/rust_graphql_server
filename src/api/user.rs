use std::sync::Arc;

use crate::{
    domain::user::{find_one, register},
    repositories::user::MongoRepository,
};

use async_graphql::{Context, Error, Object, Result, SimpleObject};

#[derive(SimpleObject)]
struct User {
    email: String,
}

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutations;

#[Object]
impl UserQuery {
    async fn user(&self) -> User {
        let user = find_one::execute();

        User { email: user.email }
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
            Ok(user) => Ok(User { email: user.email }),
            Err(_) => Err(Error::new("Unknown Error")),
        }
    }
}
