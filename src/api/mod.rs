use std::{convert::Infallible, sync::Arc};

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Request, Schema,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use warp::{
    filters::BoxedFilter, http::Response as HttpResponse, hyper::StatusCode, Filter, Rejection,
    Reply,
};

use crate::repositories::user::MongoRepository;

mod extensions;
mod routes;
mod schema;

pub fn make_routes(repo: Arc<MongoRepository>) -> BoxedFilter<(impl Reply,)> {
    let schema = schema::build_schema()
        .data(repo)
        .extension(extensions::authentication::Authentication)
        .finish();

    let health = warp::path::end().and_then(routes::health);

    let graphql_handler = warp::post().and(warp::path("graphql").and(
        async_graphql_warp::graphql(schema).and_then(
            |(schema, request): (Schema<_, _, _>, Request)| async move {
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            },
        ),
    ));

    let graphql_playground = warp::get().and(warp::path("playground")).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    health
        .or(graphql_handler)
        .or(graphql_playground)
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
        })
        .boxed()
}
