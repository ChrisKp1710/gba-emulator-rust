# GBA Emulator - Rust

Un emulatore Game Boy Advance ad alte prestazioni scritto in Rust, ottimizzato per giocare a Pokémon Rubino, Smeraldo, Rosso Fuoco e altri titoli GBA.

> ⚠️ **Nota:** Questo è un progetto in fase di sviluppo iniziale. L'emulatore è funzionale ma molte funzionalità sono ancora in fase di implementazione.

## 🎮 Caratteristiche

- **Emulazione CPU ARM7TDMI** - Architettura base implementata con supporto registri e modalità
- **Sistema grafico** - PPU base con timing e vblank
- **Sistema memoria completo** - Memory mapping accurato per tutte le regioni GBA
- **Ottimizzazione massima** - Compilazione LTO, codegen ottimizzato, panic=abort
- **Caricamento ROM** - Supporto completo per ROM GBA con parsing header
- **Frontend SDL2** - Interfaccia grafica funzionale

### 🚧 In Sviluppo

- Implementazione istruzioni ARM/THUMB complete
- Rendering grafico (background, sprites)
- Audio (APU)
- Input controller
- Save States
- Supporto salvataggi (SRAM, Flash, EEPROM)

## 🏗️ Architettura

Il progetto è strutturato in crate separati per modularità e riusabilità:

```
gba-emulator-rust/
├── gba-core/           # Core dell'emulatore (bus, memoria, PPU, APU)
├── gba-arm7tdmi/       # Emulatore CPU ARM7TDMI
├── gba-frontend-sdl2/  # Frontend desktop con SDL2
└── Cargo.toml          # Workspace configuration
```

## 🚀 Compilazione

### Requisiti

- **Rust 1.75+** (edition 2021)
- **SDL2 development libraries**

### Windows (PowerShell)

```powershell
# Download e setup automatico SDL2
.\download_sdl2.ps1

# Build release
.\build.ps1 -Release

# Build e run con ROM
.\build.ps1 -Release -Run -Rom "path\to\pokemon.gba"
```

Oppure manualmente:

```powershell
# Setup SDL2 (solo prima volta)
.\download_sdl2.ps1

# Build
$env:SDL2_LIB_DIR = "C:\Users\chris\Documents\gba-emulator-rust"
cargo build --release

# L'eseguibile sarà in: target\release\gba-emulator.exe
```

### Linux

```bash
# Installa SDL2
sudo apt-get install libsdl2-dev

# Build
cargo build --release

# Run
./target/release/gba-emulator path/to/pokemon.gba
```

### macOS

```bash
# Installa SDL2
brew install sdl2

# Build
export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib"
cargo build --release

# Run
./target/release/gba-emulator path/to/pokemon.gba
```

## 📖 Uso

```bash
# Esegui con ROM
gba-emulator.exe pokemon_emerald.gba

# Con BIOS custom (opzionale)
gba-emulator.exe pokemon_emerald.gba --bios gba_bios.bin
```

### ⌨️ Comandi

- **Arrow Keys** - D-Pad
- **Z** - Button A
- **X** - Button B  
- **A** - Button L
- **S** - Button R
- **Enter** - Start
- **Backspace** - Select
- **F5** - Save State (non ancora implementato)
- **F9** - Load State (non ancora implementato)
- **ESC** - Exit

## 🎯 Roadmap

### ✅ Completato

1. Struttura del progetto modulare
2. Sistema memoria e bus completo
3. Caricamento ROM e parsing header
4. PPU base con timing
5. Frontend SDL2 funzionante
6. Sistema interrupt base

### 🚧 In Corso

1. Implementazione CPU ARM7TDMI completa
   - [ ] Tutte le istruzioni ARM
   - [ ] Tutte le istruzioni THUMB
   - [ ] Pipeline CPU accurata

### 📋 Pianificato

1. PPU (Picture Processing Unit) completa
   - [ ] Background rendering (Mode 0-2)
   - [ ] Sprite rendering
   - [ ] Modalità bitmap (Mode 3-5)
   - [ ] Effects (blending, mosaic)

2. APU (Audio Processing Unit)
   - [ ] Channel 1-4 (GB compatibili)
   - [ ] DMA audio channels
   - [ ] Audio mixing

3. Input e Periferiche
   - [ ] Controller input funzionante
   - [ ] Timer hardware
   - [ ] DMA controller

4. Salvataggi
   - [ ] Save States
   - [ ] SRAM
   - [ ] Flash
   - [ ] EEPROM

5. Ottimizzazioni Avanzate
   - [ ] JIT compilation (opzionale)
   - [ ] SIMD optimizations
   - [ ] Multi-threading

## 📚 Risorse Tecniche

- **[ARM7TDMI Technical Reference](http://infocenter.arm.com/help/topic/com.arm.doc.ddi0210c/DDI0210B.pdf)** - Documentazione ufficiale CPU
- **[GBATEK](http://problemkaputt.de/gbatek.htm)** - Documentazione GBA completa
- **[TONC](https://www.coranac.com/tonc/text/)** - GBA Development Guide
- **[cowbite spec](https://www.cs.rit.edu/~tjh8300/CowBite/CowBiteSpec.htm)** - Specifiche hardware

## 🧪 Testing

Il progetto include test unitari per i componenti principali:

```powershell
# Run tutti i test
cargo test

# Test specifici
cargo test --package gba-arm7tdmi
cargo test --package gba-core
```

## 📊 Performance

Target di performance:

- **60 FPS** costanti
- **Latenza input** < 16ms  
- **Consumo CPU** < 50% (single core moderno)
- **Memoria** < 100 MB

Ottimizzazioni implementate:

- **LTO** (Link Time Optimization) - "fat"
- **Codegen units** - 1 per massima ottimizzazione
- **Strip** - Binary stripping per ridurre dimensioni
- **Panic** - abort per evitare unwinding overhead
- **ahash** - Hash function veloce
- **parking_lot** - Lock più performanti

## 🤝 Contribuire

Contributi benvenuti! Vedi [DEVELOPMENT.md](DEVELOPMENT.md) per dettagli su:

- Architettura del progetto
- Convenzioni di codice
- Testing
- Debugging

## 📄 Licenza

MIT License - Vedi [LICENSE](LICENSE) per dettagli

---

**Note Legali:** Questo è un progetto educativo. Nintendo e Game Boy Advance sono marchi registrati di Nintendo Co., Ltd. Per utilizzare l'emulatore è necessario possedere legalmente le ROM dei giochi.

## 🙏 Ringraziamenti

Progetti di riferimento che hanno ispirato questo emulatore:

- **[rustboyadvance-ng](https://github.com/michelhe/rustboyadvance-ng)** - Eccellente emulatore GBA in Rust
- **[mGBA](https://mgba.io/)** - Emulatore GBA di riferimento
- **[NanoboyAdvance](https://github.com/fleroviux/NanoboyAdvance)** - Emulatore moderno in C++

## 📧 Contatti

Per domande, suggerimenti o bug report, apri una issue su GitHub.
