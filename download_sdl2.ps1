# Script per scaricare SDL2 automaticamente
# Esegui: .\download_sdl2.ps1

$SDL2_VERSION = "2.30.9"
$SDL2_URL = "https://github.com/libsdl-org/SDL/releases/download/release-$SDL2_VERSION/SDL2-devel-$SDL2_VERSION-VC.zip"
$SDL2_ZIP = "SDL2.zip"
$SDL2_DIR = "SDL2-$SDL2_VERSION"

Write-Host "🎮 Downloading SDL2 $SDL2_VERSION..." -ForegroundColor Cyan

try {
    # Download SDL2
    Invoke-WebRequest -Uri $SDL2_URL -OutFile $SDL2_ZIP
    Write-Host "✓ Downloaded SDL2" -ForegroundColor Green
    
    # Extract
    Expand-Archive -Path $SDL2_ZIP -DestinationPath "." -Force
    Write-Host "✓ Extracted SDL2" -ForegroundColor Green
    
    # Copy files
    $ProjectRoot = $PSScriptRoot
    Copy-Item -Path "$SDL2_DIR\lib\x64\SDL2.lib" -Destination $ProjectRoot
    Copy-Item -Path "$SDL2_DIR\lib\x64\SDL2.dll" -Destination $ProjectRoot
    Write-Host "✓ Copied SDL2 files" -ForegroundColor Green
    
    # Cleanup
    Remove-Item $SDL2_ZIP
    Remove-Item $SDL2_DIR -Recurse -Force
    Write-Host "✓ Cleanup complete" -ForegroundColor Green
    
    # Set environment variable
    $env:SDL2_LIB_DIR = $ProjectRoot
    Write-Host "✓ SDL2_LIB_DIR set to: $ProjectRoot" -ForegroundColor Green
    
    Write-Host "`n🎉 SDL2 installation complete!" -ForegroundColor Green
    Write-Host "You can now run: cargo build --release" -ForegroundColor Yellow
    
} catch {
    Write-Host "❌ Error: $_" -ForegroundColor Red
    exit 1
}
