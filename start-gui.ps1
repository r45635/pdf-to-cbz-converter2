# Rechargement du PATH pour inclure Node.js
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

# Initialisation de l'environnement MSVC
Write-Host "Initialisation de l'environnement MSVC..." -ForegroundColor Yellow
$vcvarsPath = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

if (Test-Path $vcvarsPath) {
    # Exécuter vcvars64.bat et capturer les variables d'environnement
    cmd /c "`"$vcvarsPath`" && set" | ForEach-Object {
        if ($_ -match "^(.*?)=(.*)$") {
            Set-Item -Force -Path "ENV:\$($matches[1])" -Value $matches[2]
        }
    }
    Write-Host "Environnement MSVC initialisé avec succès" -ForegroundColor Green
} else {
    Write-Host "ATTENTION: vcvars64.bat non trouvé - la compilation pourrait échouer" -ForegroundColor Red
}

# Exécution du preflight script
Write-Host "Exécution du script preflight..." -ForegroundColor Cyan
node scripts/preflight.mjs

# Lancement de l'application Tauri
Write-Host "Lancement de l'application GUI..." -ForegroundColor Green
npx tauri dev
