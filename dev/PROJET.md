# Plan de projet : Portfolio de Mathieu Piton, Développeur Web, Mobile et Software

## 1. Objectifs

L'objectif principal de ce projet est de créer un portfolio en ligne pour Mathieu Piton, un développeur spécialisé en web, mobile et software. Le portfolio permettra à Mathieu de se présenter, de partager ses compétences et de faciliter la prise de contact. Le projet sera développé en utilisant les meilleures pratiques de Test Driven Development (TDD) et de Domain-Driven Design (DDD) pour garantir une architecture robuste, testable et évolutive.

## 2. Approche technique

Le projet sera développé à 100% en Rust, en utilisant les technologies suivantes :

- Frontend : Rust avec le framework Yew ou Percy pour le développement de l'interface utilisateur
- Backend : Rust avec un framework web comme Rocket, Actix Web ou Warp pour l'API et la logique métier
- Base de données : SQLite pour le stockage des données (flux RSS, messages de contact, etc.)
- Tests : Tests unitaires et d'intégration en Rust avec les outils intégrés et des bibliothèques comme reqwest ou httpmock

L'approche TDD sera suivie tout au long du développement, avec l'écriture des tests avant le code de production. Les principes DDD seront appliqués pour organiser le code autour du domaine métier.

## 3. Structure du projet

Le projet sera organisé selon l'arborescence suivante :

```
portfolio/
  ├── api/
  │   ├── src/
  │   │   ├── main.rs
  │   │   ├── routes/
  │   │   ├── models/
  │   │   └── services/
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
  │   └── mongo/
  ├── .gitignore
  └── README.md
```

- Le dossier `api` contiendra le code backend en Rust, avec le framework choisi (Rocket, Actix Web ou Warp).
- Le dossier `web` contiendra le code frontend en Rust, avec le framework Yew ou Percy.
- Le dossier `db` contiendra les scripts et configurations pour la base de données MongoDB.

## 4. Fonctionnalités

Les principales fonctionnalités du portfolio seront :

1. Page d'accueil avec une présentation de Mathieu (bio, compétences, technologies)
2. Section "A propos" avec plus de détails sur la carrière de Mathieu
3. Formulaire de contact fonctionnel
4. Intégration de flux RSS pour afficher les derniers articles de blog ou projets de Mathieu
5. Animations 3D interactives avec Three.js pour un design immersif
6. Design responsive et accessible
7. Support multilingue (français et anglais) pour toutes les pages et fonctionnalités

## 5. Intégration de SQLite pour les flux RSS

Pour gérer l'affichage des flux RSS sur le portfolio, nous utiliserons une base de données SQLite. Voici comment cela fonctionnera :

1. Un script Rust sera créé pour récupérer régulièrement les flux RSS des blogs et projets de Mathieu.
2. Les données des flux seront parsées et stockées dans des tables SQLite dédiées.
3. L'API backend exposera un endpoint pour récupérer les X derniers articles/projets.
4. Le frontend appellera cet endpoint pour afficher les flux RSS sur la page d'accueil ou une page dédiée.

Cette approche permettra de découpler la récupération des flux de leur affichage, et d'avoir de meilleures performances en servant les données depuis la base de données plutôt que de faire des appels RSS à chaque chargement de page.

## 6. Déploiement

Le déploiement du portfolio se fera de la manière suivante :

1. Le code sera poussé sur un dépôt GitHub
2. Une intégration continue sera mise en place avec GitHub Actions
3. À chaque push, les tests seront exécutés automatiquement
4. Si les tests passent, le code sera déployé sur Vercel, avec une configuration spécifique pour les applications Rust
5. La base de données SQLite sera déployée avec le reste de l'application sur Vercel

## 7. Planning

Le développement du portfolio se déroulera sur X semaines, avec les étapes suivantes :

- Semaine 1 : Mise en place de l'architecture, choix des technologies, création des dépôts
- Semaine 2 : Développement de l'API backend avec intégration de MongoDB
- Semaine 3 : Développement des composants frontend principaux
- Semaine 4 : Intégration frontend/backend, développement des fonctionnalités avancées
- Semaine 5 : Finalisation du design, tests d'accessibilité, optimisations
- Semaine 6 : Tests utilisateur, corrections de bugs, préparation du déploiement
- Semaine 7 : Déploiement en production, rédaction de la documentation

Ce planning est indicatif et pourra être ajusté en fonction de l'avancement réel du projet.

## 8. Livrables

Les livrables du projet seront :

- Le code source complet du portfolio, frontend et backend
- La documentation technique et utilisateur
- Un accès à la version de production du portfolio pour validation avant mise en ligne officielle

## 9. Critères de validation

Le projet sera considéré comme terminé une fois les critères suivants remplis :

- Toutes les fonctionnalités décrites sont implémentées et fonctionnelles dans les deux langues (français et anglais)
- Le code est documenté et testé, avec une couverture de tests supérieure à 80%
- Le portfolio est accessible et affiche un design responsive sur tous les supports
- Les performances sont optimisées, avec un temps de chargement inférieur à 2 secondes
- La version de production est déployée et accessible publiquement
- La documentation est complète et à jour, incluant les instructions pour la gestion des traductions
- Le client (Mathieu Piton) a validé le résultat final dans les deux langues

## 10. Conventions de codage

Pour assurer la cohérence et la lisibilité du code, les conventions suivantes seront appliquées :

- Nommage des variables, fonctions et fichiers en `snake_case`
- Nommage des composants en `PascalCase`
- Indentation avec 4 espaces
- Largeur de ligne maximale de 100 caractères
- Commentaires en anglais
- Tests unitaires dans le même fichier que le code testé
- Messages de commit descriptifs en anglais, préfixés par le type de changement (feat, fix, docs, etc)

L'application de ces conventions sera vérifiée lors des revues de code et via des outils d'analyse statique comme Clippy.

## 11. Étapes de développement (TDD)

1. Mettre en place la structure de base du projet Rust avec Cargo
2. Configurer l'intégration continue (GitHub Actions) pour les tests et le déploiement
3. Définir les User Stories principales
4. Pour chaque User Story :
   1. Écrire un test d'acceptation
   2. Écrire les tests unitaires des composants impliqués
   3. Implémenter le code pour faire passer les tests
   4. Refactorer si besoin en gardant les tests au vert
5. Développer le backend Rust :
   1. Définir les routes de l'API
   2. Écrire les tests des handlers
   3. Implémenter les handlers
6. Intégrer SQLite pour les flux RSS
   1. Définir la structure des données des flux RSS dans SQLite
   2. Écrire les tests pour la récupération des flux depuis SQLite
   3. Implémenter le code d'interaction avec SQLite et faire passer les tests
   4. Intégrer les données réelles des flux dans la page RSS
7. Mettre en place le formulaire de contact
   1. Écrire les tests du formulaire (validation, envoi email)
   2. Implémenter le formulaire et faire passer les tests
8. Développer le frontend Rust avec Yew ou Percy :
   1. Mettre en place la structure des composants
   2. Intégrer un framework d'internationalisation (i18n) pour gérer les traductions
   3. Écrire les tests des composants, y compris la vérification du changement de langue
   4. Implémenter les composants et la logique de changement de langue
   5. Gérer les interactions et la communication avec le backend
9. Intégrer le design avec CSS/Sass
   1. Écrire les tests des règles CSS principales
   2. Implémenter les styles et faire passer les tests
10. Ajouter les éléments d'animation avec Three.js
    1. Écrire les tests des animations
    2. Implémenter les animations et faire passer les tests
11. Optimiser les performances et l'accessibilité
    1. Écrire les tests de performance et d'accessibilité
    2. Optimiser le code pour faire passer les tests
12. Rédiger la documentation du projet, y compris les instructions pour SQLite
13. Déployer en production sur Vercel

Ce plan de projet servira de base pour le développement du portfolio. Il pourra être amené à évoluer au fur et à mesure de l'avancement et des retours du client. L'objectif est de livrer un portfolio de qualité, reflétant les compétences de Mathieu Piton, dans les temps et le budget impartis.
