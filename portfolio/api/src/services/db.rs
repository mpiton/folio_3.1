use anyhow::Result;
use mongodb::IndexModel;
use mongodb::{bson::doc, Client, Database};

pub async fn init_db() -> Result<Database> {
    dotenv::dotenv().ok();
    let mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL must be set");

    let client = Client::with_uri_str(&mongo_url).await?;
    let db = client.database("portfolio");

    // Vérifier si les collections existent, sinon les créer
    ensure_collection_exists(&db, "portfolio").await?;
    ensure_collection_exists(&db, "contacts").await?;

    Ok(db)
}

async fn ensure_collection_exists(db: &Database, collection_name: &str) -> Result<()> {
    let collections = db.list_collection_names().await?;
    if !collections.contains(&collection_name.to_string()) {
        db.create_collection(collection_name).await?;

        // Créer les index nécessaires selon la collection
        match collection_name {
            "portfolio" => {
                let collection = db.collection::<mongodb::bson::Document>(collection_name);
                let index = IndexModel::builder()
                    .keys(doc! {
                        "url": 1,
                        "pub_date": 1
                    })
                    .build();
                collection.create_index(index).await?;
            }
            "contacts" => {
                let collection = db.collection::<mongodb::bson::Document>(collection_name);
                let index = IndexModel::builder()
                    .keys(doc! {
                        "email": 1,
                        "created_at": -1
                    })
                    .build();
                collection.create_index(index).await?;
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_initialization() {
        dotenv::dotenv().ok();
        let mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL must be set");
        let client = Client::with_uri_str(&mongo_url)
            .await
            .expect("Failed to connect to MongoDB");

        let db = client.database("portfolio_test");

        // Nettoyer la base de test
        db.drop().await.ok();

        // Initialiser la base
        let db = init_db().await.expect("Failed to initialize database");

        // Vérifier que les collections existent
        let collections = db
            .list_collection_names()
            .await
            .expect("Failed to list collections");
        assert!(collections.contains(&"portfolio".to_string()));
        assert!(collections.contains(&"contacts".to_string()));

        // Vérifier qu'on peut insérer un document dans chaque collection
        let portfolio_collection = db.collection("portfolio");
        portfolio_collection
            .insert_one(doc! { "test": "value" })
            .await
            .expect("Failed to insert document in portfolio");

        let contacts_collection = db.collection("contacts");
        contacts_collection
            .insert_one(doc! { "test": "value" })
            .await
            .expect("Failed to insert document in contacts");

        // Nettoyer après le test
        db.drop().await.ok();
    }
}
