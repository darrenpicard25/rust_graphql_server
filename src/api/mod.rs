use async_graphql::MergedObject;

mod health_check;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(user::UserQuery, health_check::HealthCheckQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(user::UserMutations);
