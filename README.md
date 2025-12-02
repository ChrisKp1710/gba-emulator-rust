# GBA Emulator - Rust

Un emulatore Game Boy Advance ad alte prestazioni scritto in Rust, ottimizzato per giocare a Pokémon Rubino, Smeraldo, Rosso Fuoco e altri titoli GBA.

> ⚠️ **Nota:** Questo è un progetto in fase di sviluppo iniziale. L'emulatore è funzionale ma molte funzionalità sono ancora in fase di implementazione.

## 🎮 Caratteristiche

### ✅ Completate

- **✅ CPU ARM7TDMI Completa**
  - Set istruzioni ARM (32-bit) completo - 40+ istruzioni
  - Set istruzioni THUMB (16-bit) completo - 100+ varianti
  - Tutti i 19 formati THUMB implementati
  - Switch ARM↔THUMB funzionante
  - Condition codes, barrel shifter, flag NZCV
  - **10 test unitari** che verificano correttezza ✅
- **✅ PPU Mode 3 Funzionante**
  - Rendering bitmap RGB555 240x160 pixel
  - I/O registers: DISPCNT, DISPSTAT, VCOUNT
  - VBlank interrupt integrato
  - **4 test unitari** per rendering (pixel, gradiente, barre colorate) ✅
- **✅ Input Controller Completo**
  - KEYINPUT register (0x04000130)
  - D-Pad, A/B, L/R, Start/Select
  - Mappatura SDL2 completa
- **✅ Sistema Memoria Completo** - Memory mapping accurato per tutte le regioni GBA
- **✅ Sistema Interrupt** - Controller interrupt con IE/IF/IME
- **✅ Caricamento ROM** - Supporto completo con parsing header
- **✅ Frontend SDL2** - Interfaccia grafica 60 FPS con conversione RGB555→RGB888
- **✅ Ottimizzazione Massima** - LTO fat, single codegen unit, strip

### 🚧 In Sviluppo

- **PPU Advanced Modes**
  - Mode 0 (tile-based) per giochi Pokémon
  - Mode 1-2 (affine backgrounds)
  - Sprite rendering (OAM)
  - Window effects
- **Audio (APU)** - Sistema audio completo
- **Save States** - Salvataggio/caricamento stato
- **Supporto Salvataggi** - SRAM, Flash, EEPROM

## 🏗️ Architettura

Il progetto è strutturato in crate separati per modularità e riusabilità:

```
gba-emulator-rust/
├── gba-core/           # Core dell'emulatore (bus, memoria, PPU, APU)
├── gba-arm7tdmi/       # Emulatore CPU ARM7TDMI
├── gba-frontend-sdl2/  # Frontend desktop con SDL2
└── Cargo.toml          # Workspace configuration
```

> 📘 **Per capire in dettaglio l'architettura e come modificare il codice:**
> Leggi [GUIDA_ARCHITETTURA.md](GUIDA_ARCHITETTURA.md) - Spiega step-by-step ogni componente!

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

Il progetto include una suite di test completa per garantire correttezza:

```powershell
# Run tutti i test
cargo test

# Test CPU ARM7TDMI (10 test unitari)
cargo test --package gba-arm7tdmi

# Test core emulator
cargo test --package gba-core
```

### Test CPU - 10/10 Passano ✅

La CPU include 10 test unitari che verificano:

**ARM (32-bit):**

- ✅ `test_mov_instruction` - MOV con immediato
- ✅ `test_add_instruction` - ADD tra registri
- ✅ `test_branch_instruction` - Branch (B)
- ✅ `test_ldr_str_instructions` - LDR/STR memoria
- ✅ `test_cpu_creation` e `test_cpu_reset` - Base CPU

**THUMB (16-bit):**

- ✅ `test_thumb_mov_immediate` - MOV immediato
- ✅ `test_thumb_add_subtract` - ADD/SUB registri
- ✅ `test_thumb_ldr_str` - LDR/STR con offset
- ✅ `test_thumb_branch` - Branch incondizionale

Tutti i test passano con successo verificando la correttezza dell'implementazione.
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
```
