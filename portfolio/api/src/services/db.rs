use anyhow::Result;
use mongodb::IndexModel;
use mongodb::{bson::doc, Client, Database};
use tokio::sync::OnceCell;

#[allow(dead_code)]
static CLEANUP: OnceCell<()> = OnceCell::const_new();

pub async fn init_db() -> Result<Database> {
    let mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL must be set");
    let client = Client::with_uri_str(&mongo_url).await?;
    let db = client.database("portfolio");
    init_collections(&db).await?;
    Ok(db)
}

async fn init_collections(db: &Database) -> Result<()> {
    let collections = ["portfolio", "contacts"];

    for collection_name in collections.iter() {
        // Créer la collection si elle n'existe pas
        if !db
            .list_collection_names()
            .await?
            .contains(&collection_name.to_string())
        {
            db.create_collection(collection_name.to_string()).await?;
        }

        // Créer les index nécessaires selon la collection
        match *collection_name {
            "portfolio" => {
                let collection = db.collection::<mongodb::bson::Document>(collection_name);

                // Index pour l'unicité et la recherche
                let index = IndexModel::builder()
                    .keys(doc! {
                        "url": 1,
                        "pub_date": 1
                    })
                    .build();
                collection.create_index(index).await?;

                // TTL index pour nettoyer les vieux articles (90 jours)
                let ttl_index = IndexModel::builder()
                    .keys(doc! { "pub_date": 1 })
                    .options(
                        mongodb::options::IndexOptions::builder()
                            .expire_after(std::time::Duration::from_secs(90 * 24 * 60 * 60))
                            .build(),
                    )
                    .build();
                collection.create_index(ttl_index).await?;
            }
            "contacts" => {
                let collection = db.collection::<mongodb::bson::Document>(collection_name);

                // Index pour l'unicité et la recherche
                let index = IndexModel::builder()
                    .keys(doc! {
                        "email": 1,
                        "created_at": -1
                    })
                    .build();
                collection.create_index(index).await?;

                // TTL index pour nettoyer les vieux contacts (180 jours)
                let ttl_index = IndexModel::builder()
                    .keys(doc! { "created_at": 1 })
                    .options(
                        mongodb::options::IndexOptions::builder()
                            .expire_after(std::time::Duration::from_secs(180 * 24 * 60 * 60))
                            .build(),
                    )
                    .build();
                collection.create_index(ttl_index).await?;
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
pub mod test_utils {
    use super::{init_collections, CLEANUP};
    use anyhow::Result;
    use futures_util::future::join_all;
    use mongodb::{Client, Database};
    use std::time::Duration;
    use tokio::time::timeout;

    const TEST_TIMEOUT: Duration = Duration::from_secs(60);

    pub async fn create_test_db() -> Result<Database> {
        dotenv::dotenv().ok();
        let mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL must be set");
        let client = Client::with_uri_str(&mongo_url).await?;
        let client_clone = client.clone();

        // Nettoyer les anciennes bases de test au démarrage des tests
        if timeout(
            TEST_TIMEOUT,
            CLEANUP.get_or_init(|| async move {
                if let Ok(db_names) = client_clone.list_database_names().await {
                    println!("Nettoyage de {} bases de test", db_names.len());
                    let futures = db_names
                        .into_iter()
                        .filter(|name| name.starts_with("test_db_"))
                        .map(|name| {
                            let client = client_clone.clone();
                            async move {
                                if let Err(e) = client.database(&name).drop().await {
                                    eprintln!(
                                        "Erreur lors de la suppression de la base {}: {}",
                                        name, e
                                    );
                                } else {
                                    println!("Base {} supprimée", name);
                                }
                            }
                        })
                        .collect::<Vec<_>>();
                    join_all(futures).await;
                }
            }),
        )
        .await
        .is_err()
        {
            eprintln!("Timeout lors du nettoyage des anciennes bases de test");
        }

        let db_name = format!("test_db_{}", mongodb::bson::oid::ObjectId::new().to_hex());
        println!("Création de la base de test {}", db_name);
        let db = client.database(&db_name);

        match timeout(TEST_TIMEOUT, init_collections(&db)).await {
            Ok(result) => {
                result?;
                println!("Collections initialisées avec succès");
            }
            Err(_) => {
                eprintln!("Timeout lors de l'initialisation des collections");
                return Err(anyhow::anyhow!(
                    "Timeout lors de l'initialisation des collections"
                ));
            }
        }

        Ok(db)
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::create_test_db;
    use futures_util::TryStreamExt;
    use std::time::Duration;
    use tokio::time::timeout;

    const TEST_TIMEOUT: Duration = Duration::from_secs(60);

    #[tokio::test]
    async fn test_db_initialization() {
        dotenv::dotenv().ok();
        println!("Démarrage du test d'initialisation de la base de données");

        // Test avec timeout
        match timeout(TEST_TIMEOUT, async {
            let db = create_test_db()
                .await
                .expect("Failed to create test database");
            println!("Base de test créée");

            // Vérifier que les collections existent
            let collections = db
                .list_collection_names()
                .await
                .expect("Failed to list collections");
            println!("Collections trouvées : {:?}", collections);
            assert!(collections.contains(&"portfolio".to_string()));
            assert!(collections.contains(&"contacts".to_string()));

            // Vérifier que les index sont créés
            let portfolio_indexes = db
                .collection::<mongodb::bson::Document>("portfolio")
                .list_indexes()
                .await
                .expect("Failed to list portfolio indexes")
                .try_collect::<Vec<_>>()
                .await
                .expect("Failed to collect portfolio indexes");

            println!("Index portfolio trouvés : {}", portfolio_indexes.len());
            assert!(
                portfolio_indexes.len() > 1,
                "Expected at least 2 indexes for portfolio collection"
            );

            let contacts_indexes = db
                .collection::<mongodb::bson::Document>("contacts")
                .list_indexes()
                .await
                .expect("Failed to list contacts indexes")
                .try_collect::<Vec<_>>()
                .await
                .expect("Failed to collect contacts indexes");

            println!("Index contacts trouvés : {}", contacts_indexes.len());
            assert!(
                contacts_indexes.len() > 1,
                "Expected at least 2 indexes for contacts collection"
            );

            // Nettoyer après le test
            println!("Nettoyage de la base de test");
            db.drop().await.ok();
            println!("Test terminé avec succès");
        })
        .await
        {
            Ok(_) => (),
            Err(_) => panic!(
                "Le test a dépassé le délai de {} secondes",
                TEST_TIMEOUT.as_secs()
            ),
        }
    }
}
