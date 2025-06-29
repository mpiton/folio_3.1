use anyhow::Result;
use mongodb::IndexModel;
use mongodb::{
    bson::{doc, Document},
    Client, Database,
};
use std::time::Duration;

/// Initializes MongoDB connection and configures collections
///
/// # Collection Setup
/// - Creates missing collections
/// - Configures indexes:
///   - Unique compound indexes for query optimization
///   - TTL indexes for automatic data expiration
///
/// # Errors
/// Returns error if:
/// - MongoDB connection fails
/// - Index creation fails
pub async fn initialize() -> Result<()> {
    let base_mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL must be set");
    let mongo_db = std::env::var("MONGO_DB").expect("MONGO_DB must be set");
    let mongo_url = format!("{base_mongo_url}?authSource={mongo_db}");

    println!("Connecting to MongoDB Atlas...");
    let client = Client::with_uri_str(&mongo_url).await?;
    let db = client.database("portfolio");
    init_collections(&db).await?;
    Ok(())
}

/// Initializes the database collections with their indexes.
///
/// # Errors
///
/// This function returns an error if:
/// - Collection creation fails
/// - Index creation fails
/// - MongoDB operation fails
async fn init_collections(db: &Database) -> Result<()> {
    let collections = ["portfolio", "contacts"];
    println!("Starting collection initialization");

    // First step: create collections
    for collection_name in collections.iter() {
        println!("Checking collection {collection_name}");
        if !db
            .list_collection_names()
            .await?
            .contains(&collection_name.to_string())
        {
            println!("Creating collection {collection_name}");
            db.create_collection(*collection_name).await?;
        }
    }

    println!("Collections created successfully");

    // Second step: drop existing indexes and create new ones
    for collection_name in collections.iter() {
        let collection = db.collection::<Document>(collection_name);
        println!("Dropping existing indexes for {collection_name}");
        collection.drop_indexes().await?;

        match *collection_name {
            "portfolio" => {
                println!("Configuring indexes for portfolio");

                // Index for uniqueness and search
                println!("Creating url/pub_date index for portfolio");
                let index = IndexModel::builder()
                    .keys(doc! {
                        "url": 1,
                        "pub_date": 1
                    })
                    .build();
                collection.create_index(index).await?;
                println!("url/pub_date index created successfully");

                // TTL index to clean up old articles (90 days)
                println!("Creating TTL index on pub_date for portfolio");
                let ttl_index = IndexModel::builder()
                    .keys(doc! { "pub_date": 1 })
                    .options(
                        mongodb::options::IndexOptions::builder()
                            .expire_after(Duration::from_secs(90 * 24 * 60 * 60))
                            .build(),
                    )
                    .build();
                collection.create_index(ttl_index).await?;
                println!("TTL index created successfully for portfolio");
            }
            "contacts" => {
                println!("Configuring indexes for contacts");

                // Index for uniqueness and search
                println!("Creating email/created_at index for contacts");
                let index = IndexModel::builder()
                    .keys(doc! {
                        "email": 1,
                        "created_at": -1
                    })
                    .build();
                collection.create_index(index).await?;
                println!("email/created_at index created successfully");

                // TTL index to clean up old contacts (180 days)
                println!("Creating TTL index on created_at for contacts");
                let ttl_index = IndexModel::builder()
                    .keys(doc! { "created_at": 1 })
                    .options(
                        mongodb::options::IndexOptions::builder()
                            .expire_after(Duration::from_secs(180 * 24 * 60 * 60))
                            .build(),
                    )
                    .build();
                collection.create_index(ttl_index).await?;
                println!("TTL index created successfully for contacts");
            }
            _ => {}
        }
    }

    println!("Collection initialization completed successfully");
    Ok(())
}
