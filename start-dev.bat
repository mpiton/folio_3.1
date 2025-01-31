@echo off
chcp 65001 > nul

if "%1"=="sync" (
    echo 📡 Synchronisation des flux RSS...
    cd "%~dp0portfolio\api"
    cargo run --bin sync_rss
    exit /b
)

echo.
echo 🚀 Démarrage de l'environnement de développement Portfolio...
echo.

REM Set paths relative to script location
set "ROOT_PATH=%~dp0"
set "API_PATH=%ROOT_PATH%portfolio\api"
set "WEB_PATH=%ROOT_PATH%portfolio\web"

REM Start Docker services
cd "%API_PATH%"
echo 🐳 Lancement des services Docker...
make up
echo.
echo ⏳ Attente du démarrage de MongoDB...
timeout /t 5 > nul

REM Sync RSS feeds after MongoDB is ready
echo 📡 Synchronisation des flux RSS...
cd "%API_PATH%"
start /wait cmd /c "cargo run --bin sync_rss"

REM Start the backend with cargo
echo 🦀 Démarrage du backend Rust...
start cmd /k "cd /d "%API_PATH%" && cargo run --bin portfolio-api"

REM Start the frontend
echo 🌐 Démarrage du frontend Astro...
start cmd /k "cd /d "%WEB_PATH%" && npm run dev"

echo.
echo ✨ L'environnement de développement est en cours de démarrage...
echo 📝 Les logs seront affichés dans les fenêtres respectives
echo 📡 Pour forcer une synchronisation RSS, utilisez: %~nx0 sync
echo.
pause