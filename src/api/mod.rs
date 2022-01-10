use async_graphql::{MergedObject, Object};

mod health_check;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(user::UserQuery, health_check::HealthCheckQuery);
pub struct Mutation;

#[Object]
impl Mutation {
    async fn hello(&self) -> String {
        String::from("Hello Mutation")
    }
}
