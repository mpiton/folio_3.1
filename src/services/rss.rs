#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::db::test_utils::{cleanup_collections, create_test_db};
    use tokio::time::{timeout, Duration};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    const TEST_TIMEOUT: Duration = Duration::from_secs(30);
    const MOCK_DELAY: Duration = Duration::from_millis(10);
    const CACHE_CHECK_DELAY: Duration = Duration::from_millis(50);

    #[tokio::test]
    async fn test_fetch_and_store_feed() -> Result<(), Box<dyn std::error::Error>> {
        println!("Démarrage du test RSS");
        let result = timeout(TEST_TIMEOUT, async {
            // Créer la base de test d'abord
            let db = create_test_db().await?;
            println!("Base de test créée");

            // Démarrer le mock server
            let mock_server = MockServer::start().await;
            println!("Mock server démarré sur {}", mock_server.uri());

            Mock::given(method("GET"))
                .and(path("/feed"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(
                            r#"<?xml version="1.0" encoding="UTF-8"?>
                            <rss version="2.0">
                                <channel>
                                    <title>Test Feed</title>
                                    <link>http://example.com</link>
                                    <description>Test Description</description>
                                    <item>
                                        <title>Test Item</title>
                                        <link>http://example.com/item</link>
                                        <description>Test Item Description</description>
                                        <pubDate>Tue, 15 Nov 2023 12:00:00 GMT</pubDate>
                                    </item>
                                </channel>
                            </rss>"#,
                            "application/xml",
                        )
                        .set_delay(MOCK_DELAY),
                )
                .expect(1)
                .mount(&mock_server)
                .await;

            let feed_url = format!("{}/feed", &mock_server.uri());
            println!("URL du flux RSS de test: {}", feed_url);

            // Premier appel pour remplir le cache
            println!("Premier appel pour remplir le cache");
            let items = fetch_and_store_feed(&db, &feed_url).await?;
            assert_eq!(items.len(), 1, "Le premier appel devrait retourner 1 item");
            assert_eq!(
                items[0].title, "Test Item",
                "Le titre de l'item devrait correspondre"
            );

            // Attente minimale pour s'assurer que le cache est utilisé
            println!("Attente pour la mise en cache");
            tokio::time::sleep(CACHE_CHECK_DELAY).await;

            // Deuxième appel pour vérifier le cache
            println!("Deuxième appel pour vérifier le cache");
            let cached_items = fetch_and_store_feed(&db, &feed_url).await?;
            assert_eq!(cached_items.len(), 1, "Le cache devrait contenir 1 item");
            assert_eq!(
                cached_items[0].title, "Test Item",
                "Le titre dans le cache devrait correspondre"
            );

            // Nettoyer
            println!("Nettoyage des collections de test");
            cleanup_collections(&db).await?;
            println!("Test RSS terminé avec succès");

            Ok::<_, anyhow::Error>(())
        })
        .await;

        match result {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => {
                println!("Erreur dans le test RSS: {}", e);
                Err(e.into())
            }
            Err(e) => {
                println!(
                    "Timeout dans le test RSS après {} secondes",
                    TEST_TIMEOUT.as_secs()
                );
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    format!(
                        "Test RSS timed out after {} seconds: {}",
                        TEST_TIMEOUT.as_secs(),
                        e
                    ),
                )))
            }
        }
    }
}
