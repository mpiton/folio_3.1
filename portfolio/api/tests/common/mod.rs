/// Common test utilities and fixtures for the API test suite
///
/// This module provides helper functions for setting up test environments,
/// mocking external services, and creating realistic test data.
pub mod fixtures;

use anyhow::Result;
use mongodb::Client as MongoClient;
use mongodb::Database;
use wiremock::MockServer;

/// MongoDB container configuration for integration testing
///
/// Connects to a MongoDB instance (assumes MongoDB is running locally or via container)
/// For testing, this uses environment variables if available, otherwise falls back to default
#[allow(dead_code)]
pub async fn setup_mongodb() -> Result<(MongoClient, Database)> {
    // Build connection string from environment variables or use defaults
    let mongo_user = std::env::var("MONGO_INITDB_ROOT_USERNAME").unwrap_or_default();
    let mongo_password = std::env::var("MONGO_INITDB_ROOT_PASSWORD").unwrap_or_default();
    let mongo_host = std::env::var("MONGO_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let mongo_port = std::env::var("MONGO_PORT").unwrap_or_else(|_| "27017".to_string());
    let mongo_db = std::env::var("MONGO_DB").unwrap_or_else(|_| "portfolio_test".to_string());

    // Construct connection string with credentials if provided
    let connection_string = if !mongo_user.is_empty() && !mongo_password.is_empty() {
        format!(
            "mongodb://{}:{}@{}:{}/{}?authSource=admin",
            mongo_user, mongo_password, mongo_host, mongo_port, mongo_db
        )
    } else {
        format!("mongodb://{}:{}", mongo_host, mongo_port)
    };

    // Create client with timeout for connection attempts
    let mut client_options = mongodb::options::ClientOptions::parse(&connection_string).await?;
    client_options.connect_timeout = Some(std::time::Duration::from_secs(5));
    let client = MongoClient::with_options(client_options)?;

    // Verify connection with a ping command
    use mongodb::bson::doc;
    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await?;

    let db = client.database(&mongo_db);

    Ok((client, db))
}

/// Sets up a mock RSS feed server using WireMock
///
/// # Example
/// ```ignore
/// let server = mock_rss_feed_server().await;
/// let url = server.uri();
/// // Use url in tests
/// ```
#[allow(dead_code)]
pub async fn mock_rss_feed_server() -> Result<MockServer> {
    let server = MockServer::start().await;
    Ok(server)
}

/// Cleans up test database collections
///
/// # Arguments
/// * `db` - MongoDB database instance
/// * `collections` - Names of collections to clean
#[allow(dead_code)]
pub async fn cleanup_db(db: &Database, collections: &[&str]) -> Result<()> {
    for collection_name in collections {
        let collection = db.collection::<mongodb::bson::Document>(collection_name);
        collection.delete_many(mongodb::bson::doc! {}).await?;
    }
    Ok(())
}

/// Provides common test utilities for async operations
pub mod async_utils {
    use tokio::time::Duration;

    /// Wait for a condition with timeout
    #[allow(dead_code)]
    pub async fn wait_for<F, Fut>(mut condition: F, timeout_secs: u64) -> bool
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(timeout_secs);

        loop {
            if condition().await {
                return true;
            }

            if start.elapsed() > timeout {
                return false;
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Helper for retrying operations
    #[allow(dead_code)]
    pub async fn retry_async<F, Fut, T>(mut operation: F, max_attempts: u32) -> anyhow::Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = anyhow::Result<T>>,
    {
        let mut attempts = 0;
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(_e) if attempts < max_attempts - 1 => {
                    attempts += 1;
                    tokio::time::sleep(Duration::from_millis(100 * attempts as u64)).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
