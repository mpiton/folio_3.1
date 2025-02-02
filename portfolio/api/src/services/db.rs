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
    let mongo_url = format!("{}?authSource={}", base_mongo_url, mongo_db);

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

/// Test-specific database cleaner
///
/// # Usage
/// Call before/after tests to ensure clean state
///
/// # Panics
/// If connection to test database fails
pub async fn clean_test_collections(db: &Database) -> Result<()> {
    // List of collections to empty
    let collections = vec!["contacts", "portfolio"];

    for coll_name in collections {
        let collection = db.collection::<Document>(coll_name);
        collection.delete_many(doc! {}).await?;
    }

    Ok(())
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use anyhow::Result;
    use mongodb::{Client, Database};
    use std::time::Duration;
    use tokio::time::timeout;

    const TEST_TIMEOUT: Duration = Duration::from_secs(120);

    /// Creates a test database with the necessary collections and indexes.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - `MONGO_URL` environment variable is not defined
    /// - Connection to `MongoDB` fails
    /// - Collection initialization fails
    /// - Timeout is reached
    ///
    /// # Panics
    ///
    /// Panics if `MONGO_URL` environment variable is not defined
    pub async fn create_test_db(test_name: &str) -> Result<Database> {
        // Load environment variables from .env.test file
        std::env::set_var("DOTENV_FILE", ".env.test");
        match dotenvy::from_filename(".env.test") {
            Ok(_) => println!("Loaded .env.test file successfully"),
            Err(e) => println!("Error loading .env.test file: {}", e),
        }

        let base_mongo_url = match std::env::var("MONGO_URL") {
            Ok(url) => {
                println!("Base MongoDB URL: {}", url);
                url
            }
            Err(e) => {
                println!("Error retrieving MONGO_URL: {}", e);
                return Err(anyhow::anyhow!("MONGO_URL is not defined"));
            }
        };

        let mongo_db = match std::env::var("MONGO_DB") {
            Ok(db) => {
                println!("MongoDB database: {}", db);
                db
            }
            Err(e) => {
                println!("Error retrieving MONGO_DB: {}", e);
                return Err(anyhow::anyhow!("MONGO_DB is not defined"));
            }
        };

        let mongo_url = format!("{}?authSource={}", base_mongo_url, mongo_db);
        println!("Complete MongoDB URL: {}", mongo_url);

        println!("Connecting to MongoDB Atlas for testing...");
        let client = match Client::with_uri_str(&mongo_url).await {
            Ok(client) => {
                println!("MongoDB client created successfully");
                client
            }
            Err(e) => {
                println!("Error creating MongoDB client: {}", e);
                return Err(anyhow::anyhow!("Failed to connect to MongoDB: {}", e));
            }
        };

        // Use the portfolio_test database
        let db = client.database("portfolio_test");
        println!("Using portfolio_test database");

        // Verify the connection
        match db.list_collection_names().await {
            Ok(collections) => {
                println!(
                    "Connected to database successfully. Existing collections: {:?}",
                    collections
                );
            }
            Err(e) => {
                println!("Error verifying connection: {}", e);
                return Err(anyhow::anyhow!("Failed to list collections: {}", e));
            }
        }

        // Initialize collections with a longer timeout for Atlas
        match timeout(TEST_TIMEOUT, init_test_collections(&db, test_name)).await {
            Ok(result) => {
                result?;
                println!("Collections initialized successfully");
                Ok(db)
            }
            Err(_) => {
                eprintln!(
                    "Timeout reached during collection initialization ({}s)",
                    TEST_TIMEOUT.as_secs()
                );
                Err(anyhow::anyhow!(
                    "Timeout reached during collection initialization on Atlas"
                ))
            }
        }
    }

    /// Initializes test collections with unique names.
    async fn init_test_collections(db: &Database, test_name: &str) -> Result<()> {
        let collections = [
            format!("portfolio_{}", test_name),
            format!("contacts_{}", test_name),
        ];
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
                db.create_collection(collection_name).await?;
            }
        }

        println!("Collections created successfully");

        // Second step: drop existing indexes and create new ones
        for collection_name in collections.iter() {
            let collection = db.collection::<Document>(collection_name);
            println!("Dropping existing indexes for {collection_name}");
            collection.drop_indexes().await?;

            if collection_name.starts_with("portfolio") {
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
            } else if collection_name.starts_with("contacts") {
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
        }

        println!("Collection initialization completed successfully");
        Ok(())
    }

    /// Cleans test database by emptying all collections.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Document deletion fails
    /// - MongoDB operation fails
    pub async fn clean_collections(db: &Database, test_name: &str) -> Result<()> {
        println!("Cleaning test database {}", db.name());

        // Instead of dropping the database, empty the test-specific collections
        let collections = [
            format!("portfolio_{}", test_name),
            format!("contacts_{}", test_name),
        ];

        for collection_name in collections.iter() {
            let collection = db.collection::<Document>(collection_name);
            collection.delete_many(doc! {}).await?;
            println!("Collection {collection_name} emptied");
        }

        println!("Cleaning completed");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::{clean_collections, create_test_db};
    use futures_util::TryStreamExt;
    use mongodb::bson::Document;
    use std::time::Duration;
    use tokio::time::timeout;

    const TEST_TIMEOUT: Duration = Duration::from_secs(120);

    #[tokio::test]
    async fn test_db_initialization() {
        std::env::set_var("DOTENV_FILE", ".env.test");
        dotenvy::from_filename(".env.test").ok();
        println!("Starting database initialization test");

        // Run the test with a global timeout
        match timeout(TEST_TIMEOUT, async {
            let db = create_test_db("test_db_initialization")
                .await
                .expect("Failed to create test database");
            println!("Test database created");

            // Verify that the collections exist
            let collections = db
                .list_collection_names()
                .await
                .expect("Failed to list collections");
            println!("Found collections: {collections:?}");
            assert!(collections.contains(&format!("portfolio_{}", "test_db_initialization")));
            assert!(collections.contains(&format!("contacts_{}", "test_db_initialization")));

            // Verify that the indexes are created
            let portfolio_indexes = db
                .collection::<Document>(&format!("portfolio_{}", "test_db_initialization"))
                .list_indexes()
                .await
                .expect("Failed to list portfolio indexes")
                .try_collect::<Vec<_>>()
                .await
                .expect("Failed to collect portfolio indexes");

            println!("Found portfolio indexes: {}", portfolio_indexes.len());
            assert!(
                portfolio_indexes.len() > 1,
                "Expected at least 2 indexes for portfolio collection"
            );

            let contacts_indexes = db
                .collection::<Document>(&format!("contacts_{}", "test_db_initialization"))
                .list_indexes()
                .await
                .expect("Failed to list contacts indexes")
                .try_collect::<Vec<_>>()
                .await
                .expect("Failed to collect contacts indexes");

            println!("Found contacts indexes: {}", contacts_indexes.len());
            assert!(
                contacts_indexes.len() > 1,
                "Expected at least 2 indexes for contacts collection"
            );

            // Clean up the test database
            clean_collections(&db, "test_db_initialization")
                .await
                .expect("Failed to clean up test database");

            println!("Test completed successfully");
        })
        .await
        {
            Ok(()) => (),
            Err(e) => panic!("Test timed out after {TEST_TIMEOUT:?}: {e}"),
        }
    }
}
