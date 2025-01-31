# Définir l'encodage en UTF-8
$OutputEncoding = [System.Text.Encoding]::UTF8

# Créer le dossier logs s'il n'existe pas
$logsDir = Join-Path $PSScriptRoot "..\logs"
if (-not (Test-Path $logsDir)) {
    New-Item -ItemType Directory -Path $logsDir | Out-Null
}

# Nom du fichier de log avec timestamp
$timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$logFile = Join-Path $logsDir "playwright-test-$timestamp.log"
Write-Host "Les logs seront enregistrés dans: $logFile"

# Fonction pour lire la clé API Brevo depuis le fichier .env
function Get-BrevoApiKey {
    $apiEnvFile = Join-Path $PSScriptRoot "..\..\api\.env"
    $localEnvFile = Join-Path $PSScriptRoot "..\.env"
    
    Write-Host "Vérification du fichier .env local dans: $localEnvFile"
    
    # Si le fichier .env local n'existe pas, le créer en copiant depuis l'API
    if (-not (Test-Path $localEnvFile)) {
        Write-Host "Création du fichier .env local..."
        if (Test-Path $apiEnvFile) {
            Write-Host "Copie de la clé depuis le fichier .env de l'API..."
            try {
                # Lire tout le contenu du fichier API
                $apiContent = Get-Content $apiEnvFile -Raw -ErrorAction Stop
                Write-Host "Contenu du fichier API lu avec succès"
                
                # Rechercher la ligne avec BREVO_API_KEY
                if ($apiContent -match '(?m)^BREVO_API_KEY=(.+)$') {
                    $key = $matches[1].Trim('"').Trim("'")
                    Write-Host "Clé trouvée, longueur: $($key.Length) caractères"
                    
                    # Écrire la clé dans le fichier local
                    Set-Content -Path $localEnvFile -Value "BREVO_API_KEY=$key" -Encoding UTF8
                    Write-Host "Fichier .env local créé avec succès"
                    
                    # Vérifier que le fichier a été créé correctement
                    if (Test-Path $localEnvFile) {
                        $content = Get-Content $localEnvFile -Raw
                        Write-Host "Contenu du fichier local: $content"
                    }
                }
                else {
                    Write-Host "Clé BREVO_API_KEY non trouvée dans le fichier .env de l'API"
                    return $null
                }
            }
            catch {
                Write-Host "Erreur lors de la lecture/écriture des fichiers: $_"
                return $null
            }
        }
        else {
            Write-Host "Fichier .env de l'API non trouvé: $apiEnvFile"
            return $null
        }
    }

    Write-Host "Lecture du fichier .env local..."
    try {
        $envContent = Get-Content $localEnvFile -Raw -ErrorAction Stop
        if ($envContent -match '(?m)^BREVO_API_KEY=(.+)$') {
            $key = $matches[1].Trim('"').Trim("'")
            Write-Host "Clé API Brevo trouvée dans le fichier local, longueur: $($key.Length) caractères"
            return $key
        }
    }
    catch {
        Write-Host "Erreur lors de la lecture du fichier local: $_"
        return $null
    }

    Write-Host "BREVO_API_KEY non trouvée dans le fichier .env local"
    return $null
}

# Fonction pour envoyer un email via Brevo
function Send-TestResultEmail {
    param (
        [string]$logFile,
        [bool]$success
    )

    Write-Host "Préparation de l'envoi de l'email..."

    # Récupérer la clé API Brevo depuis le fichier .env
    $brevoApiKey = Get-BrevoApiKey
    if (-not $brevoApiKey) {
        Write-Host "Impossible de récupérer la clé API Brevo"
        return
    }

    Write-Host "Lecture du fichier de log: $logFile"
    # Lire le contenu du fichier de log
    $logContent = if (Test-Path $logFile) {
        Get-Content $logFile -Raw
    } else {
        Write-Host "Fichier de log non trouvé: $logFile"
        "Aucun contenu dans le fichier de log"
    }

    # Préparer le contenu de l'email
    $status = if ($success) { "réussis" } else { "échoués" }
    Write-Host "Préparation de l'email pour les tests $status"
    
    $emailData = @{
        sender = @{
            name = "Test Runner"
            email = "no-reply@mathieu-piton.com"
        }
        to = @(
            @{
                email = "matpiton@protonmail.com"
                name = "Mathieu PITON"
            }
        )
        subject = "Tests Playwright $status - $timestamp"
        htmlContent = @"
<h1>Résultats des tests Playwright</h1>
<p><strong>Status:</strong> Tests $status</p>
<p><strong>Date:</strong> $timestamp</p>
<h2>Logs:</h2>
<pre>$logContent</pre>
"@
    }

    try {
        Write-Host "Envoi de la requête à l'API Brevo..."
        Write-Host "URL: https://api.brevo.com/v3/smtp/email"
        
        # Convertir les données en JSON
        $jsonBody = ConvertTo-Json $emailData -Depth 10
        Write-Host "Corps de la requête:"
        Write-Host $jsonBody

        # Envoyer la requête à l'API Brevo
        $response = Invoke-RestMethod `
            -Uri "https://api.brevo.com/v3/smtp/email" `
            -Method Post `
            -Headers @{
                "api-key" = $brevoApiKey
                "Content-Type" = "application/json"
            } `
            -Body $jsonBody

        Write-Host "Email envoyé avec succès"
        Write-Host "Réponse de l'API:"
        Write-Host ($response | ConvertTo-Json)
    }
    catch {
        Write-Host "Erreur lors de l'envoi de l'email:"
        Write-Host "Message d'erreur: $($_.Exception.Message)"
        Write-Host "Response: $($_.Exception.Response)"
        if ($_.Exception.Response) {
            $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
            $reader.BaseStream.Position = 0
            $reader.DiscardBufferedData()
            $responseBody = $reader.ReadToEnd()
            Write-Host "Corps de la réponse: $responseBody"
        }
    }
}

# Fonction pour vérifier si le backend est accessible
function Test-Backend {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8080/health" -Method GET -UseBasicParsing
        return $response.StatusCode -eq 200
    }
    catch {
        return $false
    }
}

# Fonction pour vérifier si le frontend est accessible
function Test-Frontend {
    try {
        $tcp = New-Object System.Net.Sockets.TcpClient
        $connect = $tcp.BeginConnect("localhost", 4321, $null, $null)
        $wait = $connect.AsyncWaitHandle.WaitOne(1000, $false)
        if ($wait) {
            $tcp.EndConnect($connect)
            $tcp.Close()
            return $true
        }
        $tcp.Close()
        return $false
    }
    catch {
        return $false
    }
}

# Fonction pour démarrer le backend si nécessaire
function Start-BackendIfNeeded {
    if (-not (Test-Backend)) {
        Write-Host "Backend non détecté, démarrage..."
        $backendPath = Join-Path $PSScriptRoot "..\..\api"
        Start-Process -FilePath "cargo" -ArgumentList "run", "--bin", "portfolio-api" -WorkingDirectory $backendPath -WindowStyle Normal
        
        # Attendre que le backend soit prêt
        $maxAttempts = 30
        $attempt = 0
        while (-not (Test-Backend) -and $attempt -lt $maxAttempts) {
            $attempt++
            Write-Host "Attente du backend... Tentative $attempt/$maxAttempts"
            Start-Sleep -Seconds 2
        }
    } else {
        Write-Host "Backend déjà en cours d'exécution et accessible via /health"
    }
}

# Fonction pour démarrer le frontend
function Start-Frontend {
    Write-Host "Démarrage du frontend..."
    $frontendPath = Join-Path $PSScriptRoot ".."
    
    # Tuer tous les processus node existants
    Get-Process | Where-Object { $_.ProcessName -eq "node" } | Stop-Process -Force -ErrorAction SilentlyContinue
    
    # Attendre que les ports soient libérés
    Start-Sleep -Seconds 2
    
    # Démarrer le frontend avec une nouvelle fenêtre PowerShell
    $frontendProcess = Start-Process powershell -ArgumentList "-NoExit", "-Command", "Set-Location '$frontendPath'; npm run dev" -PassThru
    return $frontendProcess
}

# Fonction pour attendre que les serveurs soient prêts
function Wait-ForServers {
    $maxAttempts = 30
    $attempt = 0
    $allServersReady = $false

    Write-Host "Vérification des serveurs..."

    # Vérifier/démarrer le backend
    Start-BackendIfNeeded

    # Démarrer le frontend
    $frontendProcess = Start-Frontend

    Write-Host "Attente que les serveurs soient prêts..."
    while (-not $allServersReady -and $attempt -lt $maxAttempts) {
        $attempt++
        $frontendReady = Test-Frontend
        $backendReady = Test-Backend

        if ($frontendReady -and $backendReady) {
            $allServersReady = $true
            Write-Host "Les deux serveurs sont prêts !"
            return $true
        }
        else {
            Write-Host "Tentative $attempt/$maxAttempts - Frontend: $frontendReady, Backend: $backendReady"
            Start-Sleep -Seconds 2
        }
    }

    if (-not $allServersReady) {
        Write-Host "Timeout en attendant les serveurs"
        return $false
    }
}

# Lancer les tests
Write-Host "Lancement des tests..."
try {
    # Attendre que les serveurs soient prêts
    if (-not (Wait-ForServers)) {
        Write-Host "Impossible de démarrer les serveurs, arrêt des tests"
        exit 1
    }

    Write-Host "Insertion des données de test RSS..."
    try {
        # Insérer les données directement via MongoDB
        $mongoCommand = @'
        db.portfolio.deleteMany({});
        db.portfolio.insertMany([
            {
                title: "Test Article 1",
                link: "https://test.com/1",
                pub_date: "2024-01-01T00:00:00Z",
                description: "Test Description 1",
                image_url: "https://placehold.co/600x400/grey/white/png?text=Test+Article+1"
            },
            {
                title: "Test Article 2",
                link: "https://test.com/2",
                pub_date: "2024-01-02T00:00:00Z",
                description: "Test Description 2",
                image_url: "https://placehold.co/600x400/grey/white/png?text=Test+Article+2"
            },
            {
                title: "Test Article 3",
                link: "https://test.com/3",
                pub_date: "2024-01-03T00:00:00Z",
                description: "Test Description 3",
                image_url: "https://placehold.co/600x400/grey/white/png?text=Test+Article+3"
            }
        ])
'@
        
        docker exec portfolio_mongodb mongosh "mongodb://localhost:27017/portfolio_test" --eval "$mongoCommand"
        Write-Host "Données de test RSS insérées avec succès"
    }
    catch {
        Write-Host "Erreur lors de l'insertion des données de test RSS: $_"
    }

    Write-Host "Lancement des tests avec npm..."
    $env:CI = $true  # Pour forcer le mode CI
    $process = Start-Process -FilePath "cmd.exe" -ArgumentList "/c", "npm run test:e2e" -NoNewWindow -PassThru -RedirectStandardOutput "temp_output.txt"
    
    # Lire la sortie en temps réel
    while (!$process.HasExited) {
        if (Test-Path "temp_output.txt") {
            $content = Get-Content "temp_output.txt" -Raw
            if ($content -match "Serving HTML report at http://localhost:\d+") {
                Write-Host "Rapport HTML détecté, arrêt du script..."
                Stop-Process -Id $process.Id -Force
                break
            }
        }
        Start-Sleep -Milliseconds 100
    }
    
    $testExitCode = $process.ExitCode
    Write-Host "Tests terminés avec le code de sortie: $testExitCode"
} catch {
    Write-Host "Erreur lors de l'exécution des tests: $_"
    $testExitCode = 1
} finally {
    # Nettoyer le fichier temporaire
    if (Test-Path "temp_output.txt") {
        Remove-Item "temp_output.txt"
    }
    
    # Arrêter le frontend
    if ($frontendProcess) {
        Write-Host "Arrêt du frontend..."
        Stop-Process -Id $frontendProcess.Id -Force -ErrorAction SilentlyContinue
    }
    
    # Sortir avec le code de sortie des tests
    exit $testExitCode
}
