use crate::domain::user::find_one;

use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
struct User {
    email: String,
    password: String,
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(&self) -> User {
        let user = find_one::execute();

        User {
            email: user.email,
            password: user.password,
        }
    }
}
