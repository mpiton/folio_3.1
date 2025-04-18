#!/bin/bash

# Arrête le script si une commande échoue
set -e

# Fonction pour arrêter les serveurs lancés en arrière-plan et Docker
cleanup() {
    echo "Arrêt des serveurs et des services Docker..."
    # Tuer les processus enfants lancés par ce script (API, Web)
    # pkill -P $$ envoie SIGTERM aux processus enfants directs du script actuel ($$)
    # Le '|| true' évite une erreur si aucun processus n'est trouvé
    pkill -P $$ || true

    # Arrêter Docker Compose (si make down est défini dans le Makefile)
    echo "Arrêt des services Docker (via make down)..."
    (cd portfolio/api && make down) || echo "Avertissement: 'make down' a échoué ou n'est pas défini."

    echo "Nettoyage terminé."
}

# Intercepte le signal d'interruption (Ctrl+C) et de terminaison pour appeler cleanup
trap cleanup SIGINT SIGTERM

# Chemins relatifs au script
ROOT_PATH=$(dirname "$(realpath "$0")") # Chemin absolu du répertoire du script
API_PATH="$ROOT_PATH/portfolio/api"
WEB_PATH="$ROOT_PATH/portfolio/web"

# Vérifie si le premier argument est "sync"
if [[ "$1" == "sync" ]]; then
    echo "📡 Lancement de la synchronisation RSS seule..."
    (cd "$API_PATH" && cargo run --bin sync_rss)
    echo "Synchronisation terminée."
    exit 0
fi

# Comportement par défaut : lancer l'environnement complet

echo "🚀 Démarrage de l'environnement de développement Portfolio..."
echo ""

# Lancer Docker via make
echo "🐳 Lancement des services Docker (via make up)..."
(cd "$API_PATH" && make up)
echo ""
echo "⏳ Attente du démarrage de MongoDB (5 secondes)..."
sleep 5

# Synchroniser les flux RSS après le démarrage de la DB
echo "📡 Synchronisation des flux RSS..."
(cd "$API_PATH" && cargo run --bin sync_rss)
echo "Synchronisation RSS terminée."
echo ""

# Lancer le backend API en arrière-plan
echo "🦀 Lancement du serveur API (backend Rust) en arrière-plan..."
(cd "$API_PATH" && cargo watch -x 'run --bin portfolio-api') &
API_PID=$!
echo "Serveur API lancé (PID: $API_PID)"
echo ""

# Lancer le frontend en arrière-plan
echo "🌐 Lancement du serveur Web (frontend Astro) en arrière-plan..."
(cd "$WEB_PATH" && npm run dev) &
WEB_PID=$!
echo "Serveur Web lancé (PID: $WEB_PID)"
echo ""

echo "✨ L'environnement de développement est en cours de démarrage..."
echo "📝 Les logs des serveurs API et Web s'affichent ci-dessous."
echo "🐳 Les logs Docker peuvent être consultés avec 'docker compose logs -f' dans $API_PATH"
echo "🛑 Appuyez sur Ctrl+C pour arrêter tous les services (API, Web, Docker)."
echo ""

# Attend que les processus API et Web se terminent.
# Le script restera actif ici jusqu'à ce que Ctrl+C soit pressé (ce qui déclenche cleanup)
# ou que les processus se terminent d'eux-mêmes.
wait $API_PID
wait $WEB_PID

# La fonction cleanup sera appelée automatiquement à la sortie (normale ou par signal)
