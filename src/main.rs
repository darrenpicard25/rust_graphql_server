use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use std::convert::Infallible;
use std::sync::Arc;
use warp::hyper::StatusCode;
use warp::{http::Response as HttpResponse, Filter, Rejection};

use crate::repositories::user;

mod api;
mod domain;
mod extensions;
mod repositories;

#[tokio::main]
async fn main() {
    let db = repositories::connect_to_database()
        .await
        .expect("Error connecting to mongo");

    let repository = Arc::new(user::MongoRepository::new(db));

    println!("Playground: http://localhost:8000");
    let schema = Schema::build(
        api::Query::default(),
        api::Mutation::default(),
        EmptySubscription,
    )
    .data(repository)
    .extension(extensions::authentication::Authentication)
    .finish();

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<api::Query, api::Mutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = graphql_playground
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
