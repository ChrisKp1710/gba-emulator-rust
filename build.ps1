# Script di build rapido
# Esegui: .\build.ps1

param(
    [switch]$Release,
    [switch]$Run,
    [string]$Rom
)

Write-Host "🛠️  GBA Emulator - Build Script" -ForegroundColor Cyan
Write-Host ""

# Verifica SDL2
if (-not (Test-Path "SDL2.lib")) {
    Write-Host "⚠️  SDL2 not found. Running download script..." -ForegroundColor Yellow
    .\download_sdl2.ps1
}

$env:SDL2_LIB_DIR = $PSScriptRoot

# Build
$buildType = if ($Release) { "--release" } else { "" }
Write-Host "📦 Building project $buildType..." -ForegroundColor Green

if ($buildType) {
    cargo build --release
    $exePath = "target\release\gba-emulator.exe"
} else {
    cargo build
    $exePath = "target\debug\gba-emulator.exe"
}

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Build successful!" -ForegroundColor Green

# Copy SDL2.dll
$targetDir = if ($Release) { "target\release" } else { "target\debug" }
Copy-Item "SDL2.dll" -Destination $targetDir -Force

# Run se richiesto
if ($Run -and $Rom) {
    Write-Host ""
    Write-Host "🎮 Running emulator with ROM: $Rom" -ForegroundColor Cyan
    & $exePath $Rom
} elseif ($Run) {
    Write-Host ""
    Write-Host "⚠️  Please specify a ROM file with -Rom parameter" -ForegroundColor Yellow
    Write-Host "Example: .\build.ps1 -Release -Run -Rom 'pokemon.gba'" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "📍 Executable: $exePath" -ForegroundColor Blue
