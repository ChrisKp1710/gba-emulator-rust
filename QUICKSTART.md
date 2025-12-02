# Quick Start Guide

## Prima Installazione

### 1. Clona o scarica il progetto

```powershell
git clone <repository-url> gba-emulator-rust
cd gba-emulator-rust
```

### 2. Installa Rust

Scarica e installa Rust da https://rustup.rs/

Verifica l'installazione:

```powershell
rustc --version
cargo --version
```

### 3. Setup SDL2 (Automatico)

```powershell
.\download_sdl2.ps1
```

### 4. Build

```powershell
.\build.ps1 -Release
```

## Uso Rapido

### Eseguire con una ROM

```powershell
.\target\release\gba-emulator.exe "C:\path\to\pokemon_emerald.gba"
```

### Opzioni Avanzate

```powershell
# Con BIOS custom
.\target\release\gba-emulator.exe pokemon.gba --bios gba_bios.bin

# Con logging dettagliato
$env:RUST_LOG = "debug"
.\target\release\gba-emulator.exe pokemon.gba

# Con logging di trace (molto verboso)
$env:RUST_LOG = "trace"
.\target\release\gba-emulator.exe pokemon.gba
```

## Controlli di Default

| Tasto     | Funzione GBA |
| --------- | ------------ |
| ↑ ↓ ← →   | D-Pad        |
| Z         | Button A     |
| X         | Button B     |
| A         | Button L     |
| S         | Button R     |
| Enter     | Start        |
| Backspace | Select       |
| ESC       | Exit         |

## Troubleshooting

### L'emulatore non si avvia

1. Verifica che SDL2.dll sia presente:
   ```powershell
   Test-Path .\target\release\SDL2.dll
   ```
2. Se manca, copia manualmente:
   ```powershell
   Copy-Item SDL2.dll -Destination target\release\
   ```

### Errore "file di input SDL2.lib"

```powershell
# Riesegui lo script di download
.\download_sdl2.ps1

# Imposta la variabile d'ambiente
$env:SDL2_LIB_DIR = "C:\Users\chris\Documents\gba-emulator-rust"

# Ricompila
cargo build --release
```

### Performance scadenti

1. Assicurati di usare build release:

   ```powershell
   cargo build --release
   ```

2. Verifica che non ci siano altri processi intensivi

3. Abilita logging per debug:
   ```powershell
   $env:RUST_LOG = "info"
   ```

### ROM non si carica

1. Verifica che la ROM sia valida (almeno 16 KB)
2. Verifica l'estensione (.gba)
3. Controlla i permessi del file

## Prossimi Passi

- Leggi [README.md](README.md) per panoramica completa
- Leggi [DEVELOPMENT.md](DEVELOPMENT.md) per sviluppo
- Controlla le [Issues](https://github.com/yourrepo/issues) per problemi noti

## Dove trovare ROM di test

⚠️ **Importante:** Usa solo ROM di giochi che possiedi legalmente.

Per testing è possibile usare:

- Demo ROM homebrew
- ROM di test pubbliche (AGS Aging Cartridge, Tonc demos)
- Backup personali di cartucce possedute

## Performance Attese

Con build release su hardware moderno:

- **FPS:** Target 60 FPS (attualmente placeholder)
- **CPU Usage:** < 50% single core
- **RAM:** < 100 MB
- **Latenza:** < 16ms

## Features Attualmente Implementate

✅ Caricamento ROM
✅ Parsing header GBA
✅ Memory mapping completo
✅ PPU base (timing)
✅ Sistema interrupt base
✅ Frontend SDL2

## Features in Sviluppo

🚧 CPU ARM7TDMI (istruzioni complete)
🚧 Rendering grafico
🚧 Audio
🚧 Input controller
🚧 Save states

## Supporto

Per domande o problemi:

1. Controlla questo documento
2. Leggi la documentazione completa
3. Apri una Issue su GitHub
