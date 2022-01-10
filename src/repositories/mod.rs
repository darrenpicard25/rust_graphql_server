use mongodb::{options::ClientOptions, Client, Database};

pub async fn connect_to_database() -> mongodb::error::Result<Database> {
    println!("Connecting to Mongo");

    let app_name = "authenticationService";

    let mut client_options =
        ClientOptions::parse("mongodb://localhost:27017/authenticationService?retryWrites=true")
            .await?;
    // Manually set an option
    client_options.app_name = Some(app_name.to_string());

    let client = Client::with_options(client_options)?;

    let db = client.database(app_name);

    for collection in db.list_collection_names(None).await? {
        println!("{}", collection);
    }
    println!("Connected successfully.");

    Ok(db)
}
