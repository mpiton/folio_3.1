async fn cleanup_test_dbs() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client().await?;
    let db_names = client.list_database_names(None, None).await?;

    let test_dbs: Vec<_> = db_names
        .iter()
        .filter(|name| name.starts_with("test_db_"))
        .collect();

    println!("Nettoyage de {} bases de test", test_dbs.len());

    for db_name in test_dbs {
        match client.database(db_name).drop(None).await {
            Ok(_) => println!("Base {} supprimée avec succès", db_name),
            Err(e) => {
                if e.to_string()
                    .contains("not allowed to do action [dropDatabase]")
                {
                    // Si l'erreur est due aux permissions, on essaie de vider la base
                    let db = client.database(db_name);
                    let collection_names = db.list_collection_names(None).await?;
                    for coll_name in collection_names {
                        if let Err(e) = db
                            .collection::<mongodb::bson::Document>(&coll_name)
                            .drop(None)
                            .await
                        {
                            println!(
                                "Erreur lors de la suppression de la collection {}: {}",
                                coll_name, e
                            );
                        } else {
                            println!("Collection {} supprimée", coll_name);
                        }
                    }
                } else {
                    println!("Erreur lors de la suppression de {}: {}", db_name, e);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use anyhow::Result;
    use futures_util::StreamExt;
    use mongodb::{Client, Database};
    use std::time::Duration;
    use tokio::time::timeout;

    const TEST_TIMEOUT: Duration = Duration::from_secs(60);
    const TEST_DB_NAME: &str = "test_db";

    pub async fn cleanup_collections(db: &Database) -> Result<()> {
        let mut collections = db.list_collection_names(None).await?;
        println!("Nettoyage de {} collections", collections.len());

        for coll_name in collections.drain(..) {
            if let Err(e) = db
                .collection::<mongodb::bson::Document>(&coll_name)
                .drop(None)
                .await
            {
                println!(
                    "Erreur lors de la suppression de la collection {}: {}",
                    coll_name, e
                );
            } else {
                println!("Collection {} supprimée", coll_name);
            }
        }

        Ok(())
    }

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

        let db = client.database(TEST_DB_NAME);

        // Nettoyer les collections existantes
        if let Err(e) = timeout(TEST_TIMEOUT, cleanup_collections(&db)).await {
            println!("Timeout lors du nettoyage des collections: {}", e);
        }

        // Initialiser les collections
        match timeout(TEST_TIMEOUT, init_collections(&db)).await {
            Ok(Ok(_)) => {
                println!("Collections initialisées avec succès");
                Ok(db)
            }
            Ok(Err(e)) => {
                println!("Erreur lors de l'initialisation: {}", e);
                Err(e)
            }
            Err(e) => {
                println!("Timeout lors de l'initialisation: {}", e);
                Err(anyhow::anyhow!("Timeout: {}", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::*;
    use futures_util::StreamExt;
    use std::time::Duration;
    use tokio::time::timeout;

    const TEST_TIMEOUT: Duration = Duration::from_secs(60);

    #[tokio::test]
    async fn test_db_initialization() -> Result<(), Box<dyn std::error::Error>> {
        println!("Démarrage du test d'initialisation de la base de données");

        // Création et test de la base
        let result = timeout(TEST_TIMEOUT, async {
            let db = create_test_db().await?;

            // Vérifier les collections
            let collections = db.list_collection_names(None).await?;
            println!("Collections trouvées : {:?}", collections);
            assert!(collections.contains(&"portfolio".to_string()));
            assert!(collections.contains(&"contacts".to_string()));

            // Vérifier les index
            let portfolio_indexes = db
                .collection::<mongodb::bson::Document>("portfolio")
                .list_indexes()
                .await?
                .collect::<Vec<_>>()
                .await;

            println!("Index portfolio trouvés : {}", portfolio_indexes.len());
            assert!(portfolio_indexes.len() > 1);

            let contacts_indexes = db
                .collection::<mongodb::bson::Document>("contacts")
                .list_indexes()
                .await?
                .collect::<Vec<_>>()
                .await;

            println!("Index contacts trouvés : {}", contacts_indexes.len());
            assert!(contacts_indexes.len() > 1);

            // Nettoyer
            cleanup_collections(&db).await?;

            Ok::<_, anyhow::Error>(())
        })
        .await;

        match result {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(e.into()),
            Err(e) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                format!(
                    "Test timed out after {} seconds: {}",
                    TEST_TIMEOUT.as_secs(),
                    e
                ),
            ))),
        }
    }
}
