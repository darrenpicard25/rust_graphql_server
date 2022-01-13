use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

mod user;

#[derive(MergedObject, Default)]
pub struct Query(user::UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(user::UserMutations);

pub fn build_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}
