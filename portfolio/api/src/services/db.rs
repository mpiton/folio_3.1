use anyhow::Result;
use mongodb::IndexModel;
use mongodb::{
    bson::{doc, Document},
    Client, Database,
};
use std::time::Duration;

/// Initialise la base de données en créant les collections et les index nécessaires.
///
/// # Errors
///
/// Cette fonction retourne une erreur dans les cas suivants :
/// - La variable d'environnement `MONGO_URL` n'est pas définie
/// - La connexion à `MongoDB` échoue
/// - La création des collections ou des index échoue
///
/// # Panics
///
/// Cette fonction panique si la variable d'environnement `MONGO_URL` n'est pas définie
pub async fn initialize() -> Result<()> {
    let mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL must be set");
    println!("Connexion à MongoDB Atlas...");
    let client = Client::with_uri_str(&mongo_url).await?;
    let db = client.database("portfolio");
    init_collections(&db).await?;
    Ok(())
}

/// Initialise les collections de la base de données avec leurs index.
///
/// # Errors
///
/// Cette fonction retourne une erreur si :
/// - La création des collections échoue
/// - La création des index échoue
/// - Une opération `MongoDB` échoue
async fn init_collections(db: &Database) -> Result<()> {
    let collections = ["portfolio", "contacts"];
    println!("Début de l'initialisation des collections");

    // Première étape : créer les collections
    for collection_name in collections.iter() {
        println!("Vérification de la collection {collection_name}");
        if !db
            .list_collection_names()
            .await?
            .contains(&collection_name.to_string())
        {
            println!("Création de la collection {collection_name}");
            db.create_collection(*collection_name).await?;
        }
    }

    println!("Collections créées avec succès");

    // Deuxième étape : créer les index
    for collection_name in collections.iter() {
        match *collection_name {
            "portfolio" => {
                let collection = db.collection::<Document>(collection_name);
                println!("Configuration des index pour portfolio");

                // Index pour l'unicité et la recherche
                println!("Création de l'index url/pub_date pour portfolio");
                let index = IndexModel::builder()
                    .keys(doc! {
                        "url": 1,
                        "pub_date": 1
                    })
                    .build();
                collection.create_index(index).await?;
                println!("Index url/pub_date créé avec succès");

                // TTL index pour nettoyer les vieux articles (90 jours)
                println!("Création du TTL index sur pub_date pour portfolio");
                let ttl_index = IndexModel::builder()
                    .keys(doc! { "pub_date": 1 })
                    .options(
                        mongodb::options::IndexOptions::builder()
                            .expire_after(Duration::from_secs(90 * 24 * 60 * 60))
                            .build(),
                    )
                    .build();
                collection.create_index(ttl_index).await?;
                println!("TTL index créé avec succès pour portfolio");
            }
            "contacts" => {
                let collection = db.collection::<Document>(collection_name);
                println!("Configuration des index pour contacts");

                // Index pour l'unicité et la recherche
                println!("Création de l'index email/created_at pour contacts");
                let index = IndexModel::builder()
                    .keys(doc! {
                        "email": 1,
                        "created_at": -1
                    })
                    .build();
                collection.create_index(index).await?;
                println!("Index email/created_at créé avec succès");

                // TTL index pour nettoyer les vieux contacts (180 jours)
                println!("Création du TTL index sur created_at pour contacts");
                let ttl_index = IndexModel::builder()
                    .keys(doc! { "created_at": 1 })
                    .options(
                        mongodb::options::IndexOptions::builder()
                            .expire_after(Duration::from_secs(180 * 24 * 60 * 60))
                            .build(),
                    )
                    .build();
                collection.create_index(ttl_index).await?;
                println!("TTL index créé avec succès pour contacts");
            }
            _ => {}
        }
    }

    println!("Initialisation des collections terminée avec succès");
    Ok(())
}

/// Nettoie la base de données de test en vidant toutes les collections.
///
/// # Errors
///
/// Cette fonction retourne une erreur si :
/// - La suppression des documents échoue
/// - Une opération `MongoDB` échoue
pub async fn clean_test_collections(db: &Database) -> Result<()> {
    // Liste des collections à vider
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

    /// Crée une base de données de test avec les collections et index nécessaires.
    ///
    /// # Errors
    ///
    /// Cette fonction retourne une erreur si :
    /// - La variable d'environnement `MONGO_URL` n'est pas définie
    /// - La connexion à `MongoDB` échoue
    /// - L'initialisation des collections échoue
    /// - Le timeout est atteint
    ///
    /// # Panics
    ///
    /// Cette fonction panique si la variable d'environnement `MONGO_URL` n'est pas définie
    pub async fn create_test_db() -> Result<Database> {
        dotenv::dotenv().ok();
        let mongo_url = std::env::var("MONGO_URL").expect("MONGO_URL must be set");
        println!("Connexion à MongoDB Atlas pour les tests...");
        let client = Client::with_uri_str(&mongo_url).await?;

        // Utiliser une base de test fixe
        let db = client.database("portfolio_test");
        println!("Utilisation de la base de test portfolio_test");

        // Initialiser les collections avec un timeout plus long pour Atlas
        match timeout(TEST_TIMEOUT, init_collections(&db)).await {
            Ok(result) => {
                result?;
                println!("Collections initialisées avec succès");
                Ok(db)
            }
            Err(_) => {
                eprintln!(
                    "Timeout lors de l'initialisation des collections ({}s)",
                    TEST_TIMEOUT.as_secs()
                );
                Err(anyhow::anyhow!(
                    "Timeout lors de l'initialisation des collections sur Atlas"
                ))
            }
        }
    }

    /// Nettoie la base de données de test en vidant toutes les collections.
    ///
    /// # Errors
    ///
    /// Cette fonction retourne une erreur si :
    /// - La suppression des documents échoue
    /// - Une opération `MongoDB` échoue
    pub async fn clean_collections(db: &Database) -> Result<()> {
        println!("Nettoyage de la base de test {}", db.name());

        // Au lieu de supprimer la base, on vide les collections
        for collection_name in ["portfolio", "contacts"].iter() {
            let collection = db.collection::<Document>(collection_name);
            collection.delete_many(doc! {}).await?;
            println!("Collection {collection_name} vidée");
        }

        println!("Nettoyage terminé");
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
        dotenv::dotenv().ok();
        println!("Démarrage du test d'initialisation de la base de données");

        // Exécuter le test avec un timeout global
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
                .collection::<Document>("portfolio")
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
                .collection::<Document>("contacts")
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

            // Nettoyer la base de test
            clean_collections(&db)
                .await
                .expect("Failed to cleanup test database");

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
