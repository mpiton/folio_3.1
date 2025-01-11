# Plan de projet : Portfolio de Mathieu Piton, Développeur Web, Mobile et Software

## 1. Objectifs

L'objectif principal de ce projet est de créer un portfolio en ligne pour Mathieu Piton, un développeur spécialisé en web, mobile et software. Le portfolio permettra à Mathieu de se présenter, de partager ses compétences et de faciliter la prise de contact. Le projet sera développé en utilisant les meilleures pratiques de Test Driven Development (TDD) et de Domain-Driven Design (DDD) pour garantir une architecture robuste, testable et évolutive.

## 2. Approche technique

Le projet sera développé à 100% en Rust, en utilisant les technologies suivantes :

- Frontend : Rust avec le framework Dioxus pour le développement de l'interface utilisateur
- Backend : Axum pour l'API et la logique métier (migration depuis Actix Web terminée)
- Base de données : MongoDB pour le stockage des données (flux RSS, messages de contact, etc.)
- Tests : Tests unitaires et d'intégration avec wiremock pour les tests HTTP
- Email : Service Brevo (anciennement Sendinblue) pour l'envoi d'emails
- Cache : Cache en mémoire avec tokio pour les flux RSS

## 3. Structure du projet

Le projet est organisé selon l'arborescence suivante :

```
portfolio/
  ├── api/
  │   ├── src/
  │   │   ├── main.rs
  │   │   ├── routes/
  │   │   │   ├── health.rs
  │   │   │   ├── contact.rs
  │   │   │   └── rss.rs
  │   │   ├── models/
  │   │   │   ├── contact.rs
  │   │   │   └── rss.rs
  │   │   └── services/
  │   │       ├── contact.rs
  │   │       ├── db.rs
  │   │       ├── email_queue.rs
  │   │       ├── email_templates.rs
  │   │       └── rss.rs
  │   ├── tests/
  │   └── Cargo.toml
  ├── web/
  │   ├── src/
  │   │   ├── main.rs
  │   │   ├── components/
  │   │   ├── pages/
  │   │   └── services/
  │   ├── static/
  │   ├── tests/
  │   └── Cargo.toml
  ├── db/
  │   └── mongodb/
  ├── .gitignore
  └── README.md
```

## 4. Fonctionnalités implémentées

### Backend (API)

1. Service RSS ✅
   - Synchronisation avec la base de données RSS
   - Mise en cache des flux RSS
   - Endpoint pour récupérer les articles récents
   - Synchronisation périodique automatique
   - Tests unitaires et d'intégration

2. Service Contact ✅
   - Validation des formulaires de contact
   - Protection anti-spam (rate limiting, détection de spam)
   - File d'attente d'emails asynchrone
   - Templates d'emails HTML
   - Intégration avec Brevo
   - Tests unitaires et d'intégration

3. Base de données ✅
   - Connexion MongoDB
   - Collections pour les flux RSS et les articles
   - Indexation pour les performances
   - Tests avec base de données de test

### Frontend (Web)

[En attente - Migration vers Dioxus]

## 5. Tests

Les tests sont écrits en suivant l'approche TDD et couvrent :

1. Tests unitaires ✅
   - Validation des formulaires
   - Détection de spam
   - Parsing des flux RSS
   - Templates d'emails

2. Tests d'intégration ✅
   - Endpoints API
   - Synchronisation RSS
   - File d'attente d'emails

3. Mocks ✅
   - Requêtes HTTP avec wiremock
   - Base de données de test

## 6. Sécurité

1. Protection anti-spam ✅
   - Rate limiting par IP
   - Détection de mots-clés spam
   - Validation des champs de formulaire
   - Délai minimum entre les soumissions

2. Base de données ✅
   - Indexes uniques pour éviter les doublons
   - Validation des données

## 7. Performance

1. Cache ✅
   - Mise en cache des flux RSS
   - Actualisation périodique en arrière-plan

2. Base de données ✅
   - Indexes optimisés
   - Requêtes paginées
   - Upsert pour éviter les doublons

## 8. Prochaines étapes

1. Frontend
   - Migration vers Dioxus
   - Mise en place de l'architecture
   - Développement des composants
   - Tests unitaires et d'intégration

2. Optimisations
   - Documentation API OpenAPI/Swagger
   - Monitoring temps réel
   - Système de backup automatique

3. Sécurité
   - Audit de sécurité
   - Tests de pénétration
   - Monitoring de sécurité

## 9. Déploiement

Le déploiement se fera sur :
- Backend : VPS avec Docker
- Frontend : Vercel
- Base de données : MongoDB Atlas

## 10. Maintenance

1. Logs ✅
   - Logs d'erreurs pour la synchronisation RSS
   - Logs d'envoi d'emails
   - Logs de détection de spam

2. Monitoring ✅
   - Surveillance de la synchronisation RSS
   - Surveillance de la file d'attente d'emails
   - Métriques de performance

## 11. Conventions de codage

Pour assurer la cohérence et la lisibilité du code, les conventions suivantes seront appliquées :

1. Style de code
- Nommage des variables, fonctions et fichiers en `snake_case`
   - Nommage des types, traits et structures en `PascalCase`
   - Nommage des constantes en `SCREAMING_SNAKE_CASE`
- Indentation avec 4 espaces
- Largeur de ligne maximale de 100 caractères

2. Documentation
   - Commentaires en français
   - Documentation des fonctions publiques obligatoire
   - Exemples de code dans la documentation
   - Tests comme documentation vivante

3. Organisation du code
   - Un module par fichier
   - Tests dans le même fichier que le code testé
   - Imports groupés et ordonnés (std, externes, crate)
   - Utilisation des modules pour organiser le code

4. Tests
   - Tests unitaires pour chaque fonction publique
   - Tests d'intégration pour les fonctionnalités complètes
   - Tests de documentation comme exemples
   - Utilisation de fixtures pour les données de test

5. Gestion des erreurs
   - Utilisation de `Result` et `Option`
   - Messages d'erreur descriptifs
   - Propagation des erreurs avec `?`
   - Types d'erreur personnalisés quand nécessaire

6. Performance
   - Éviter les allocations inutiles
   - Utiliser des références quand possible
   - Optimiser les requêtes MongoDB
   - Mettre en cache les données fréquemment utilisées

7. Sécurité
   - Validation des entrées utilisateur
   - Protection contre les injections
   - Gestion sécurisée des secrets
   - Logs sécurisés (pas d'informations sensibles)

8. Git
   - Messages de commit descriptifs en français
   - Une fonctionnalité par branche
   - Revue de code obligatoire
   - Tests passants avant merge

## 12. Étapes de développement (TDD)

1. Pour chaque fonctionnalité :
   - Écrire les tests d'acceptation
   - Écrire les tests unitaires
   - Implémenter le code minimal
   - Refactorer si nécessaire
   - Documenter le code
   - Revue de code

2. Cycle de développement :
   - Branche feature
   - Tests en rouge
   - Implémentation minimale
   - Tests en vert
   - Refactoring
   - Documentation
   - Pull request
   - Revue
   - Merge

## 13. Métriques de qualité

1. Code
   - Couverture de tests > 80%
   - Pas d'avertissements clippy
   - Documentation complète
   - Performance optimale

2. Performance
   - Temps de réponse API < 100ms
   - Utilisation mémoire stable
   - Charge CPU raisonnable
   - Temps de synchronisation RSS optimal

3. Base de données
   - Temps de requête < 50ms
   - Indexes optimisés
   - Utilisation mémoire contrôlée
   - Backup régulier

4. Sécurité
   - Pas de vulnérabilités connues
   - Protection anti-spam efficace
   - Données sensibles protégées
   - Logs sécurisés
