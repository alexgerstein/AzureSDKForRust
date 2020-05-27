use azure_sdk_cosmos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;
    let cosmos_client = ClientBuilder::new(account.clone(), authorization_token)?;
    //let databases = cosmos_client.list_databases().execute().await?;
    //println!("databases == {:#?}", databases);

    let database_client = cosmos_client.with_database("pollo".to_owned());
    println!("database_name == {}", database_client.database_name());

    let collections = database_client.list_collections().execute().await?;
    println!("collections == {:#?}", collections);

    //let collection_client = database_client.with_collection(&"cnt");

    Ok(())
}
