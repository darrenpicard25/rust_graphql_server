use std::sync::Arc;

mod api;
mod domain;
mod repositories;

#[tokio::main]
async fn main() {
    let db = repositories::connect_to_database()
        .await
        .expect("Error connecting to mongo");

    let repository = Arc::new(repositories::user::MongoRepository::new(db));

    println!("Playground: http://localhost:8000");
    let routes = api::make_routes(repository);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
