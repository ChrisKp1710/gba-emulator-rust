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
  - **Codice professionale**: 0 warning Clippy ✅
- **✅ PPU Mode 3 Funzionante**
  - Rendering bitmap RGB555 240x160 pixel
  - I/O registers: DISPCNT, DISPSTAT, VCOUNT
  - VBlank interrupt integrato
  - **4 test unitari** per rendering (pixel, gradiente, barre colorate) ✅
- **✅ PPU Mode 0 Completo**
  - **4 background layers (BG0-BG3)** con tile 8x8
  - **Palette RAM** (1KB): 16 colori e 256 colori
  - **BG Control** (BGxCNT): priority, char/screen base, palette mode
  - **BG Scrolling** (BGxHOFS/VOFS) per tutti i layer
  - **Layer compositing** con priority e trasparenza
  - **7 test unitari** per tile rendering, scrolling, priority ✅
- **✅ Sprite Rendering (OAM) Completo** ✨ NUOVO
  - **128 sprite** (OAM 1KB)
  - **Tutte le dimensioni**: 8x8, 16x16, 32x32, 64x64, wide, tall
  - **OBJ palette** (512 byte): 16 e 256 colori
  - **H-flip/V-flip**, priority, trasparenza
  - **VRAM OBJ** tile rendering (0x06010000+)
  - **5 test unitari** per sprite rendering ✅
- **✅ Input Controller Completo**
  - KEYINPUT register (0x04000130)
  - D-Pad, A/B, L/R, Start/Select
  - Mappatura SDL2 completa
- **✅ Sistema Memoria Completo** - Memory mapping accurato per tutte le regioni GBA
- **✅ Sistema Interrupt** - Controller interrupt con IE/IF/IME
- **✅ Caricamento ROM** - Supporto completo con parsing header
- **✅ Frontend SDL2** - Interfaccia grafica 60 FPS con conversione RGB555→RGB888
- **✅ Ottimizzazione Massima** - LTO fat, single codegen unit, strip
- **✅ Codice Professionale** - 0 warning Clippy, best practices Rust

### 🚧 In Sviluppo

- **PPU Advanced Features**
  - Mode 1-2 (affine backgrounds)
  - Mode 4-5 (bitmap paletted)
  - **Sprite rendering (OAM)** - Oggetti e personaggi
  - Window effects
  - Blending avanzato (alpha, brightness)
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

1. **CPU ARM7TDMI completa**
   - ✅ Tutte le istruzioni ARM (40+)
   - ✅ Tutte le istruzioni THUMB (100+ varianti)
   - ✅ Pipeline CPU e switch ARM↔THUMB
   - ✅ 10 test unitari passano
2. **PPU Mode 3 funzionante**
   - ✅ Rendering bitmap RGB555 240x160
   - ✅ I/O registers (DISPCNT, DISPSTAT, VCOUNT)
   - ✅ VBlank interrupt
   - ✅ 4 test unitari per rendering
3. **Input controller completo**
   - ✅ KEYINPUT register
   - ✅ D-Pad + A/B/L/R/Start/Select
   - ✅ SDL2 integration
4. Struttura del progetto modulare
5. Sistema memoria e bus completo
6. Caricamento ROM e parsing header
7. Frontend SDL2 con conversione RGB555→RGB888
8. Sistema interrupt completo

### 🚧 In Corso

1. PPU (Picture Processing Unit) avanzata
   - [ ] Background rendering (Mode 0 tile-based)
   - [ ] Sprite rendering (OAM)
   - [ ] Modalità bitmap Mode 1-2
   - [ ] Effects (blending, mosaic)

### 📋 Pianificato

1. APU (Audio Processing Unit)

   - [ ] Channel 1-4 (GB compatibili)
   - [ ] DMA audio channels
   - [ ] Audio mixing

2. Periferiche Hardware

   - [ ] Timer hardware
   - [ ] DMA controller

3. Salvataggi

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
# Run tutti i test (26 test totali)
cargo test

# Test CPU ARM7TDMI (10 test unitari)
cargo test --package gba-arm7tdmi

# Test PPU (12 test unitari: Mode 0 + Sprites)
cargo test --package gba-core ppu

# Test integrazione (4 test unitari)
cargo test --package gba-core --test integration_tests
```

### Test Suite - 26/26 Passano ✅

**CPU ARM7TDMI (10 test):**

- ✅ `test_mov_instruction` - MOV con immediato
- ✅ `test_add_instruction` - ADD tra registri
- ✅ `test_branch_instruction` - Branch (B)
- ✅ `test_ldr_str_instructions` - LDR/STR memoria
- ✅ `test_cpu_creation` e `test_cpu_reset` - Base CPU
- ✅ `test_thumb_mov_immediate` - THUMB MOV immediato
- ✅ `test_thumb_add_subtract` - THUMB ADD/SUB registri
- ✅ `test_thumb_ldr_str` - THUMB LDR/STR con offset
- ✅ `test_thumb_branch` - THUMB Branch incondizionale

**PPU Rendering (12 test):**

_Mode 0 - Tile Backgrounds (7 test):_

- ✅ `test_mode0_simple_tile` - Rendering tile 8x8 base
- ✅ `test_mode0_scrolling` - Scrolling BG layers
- ✅ `test_mode0_priority` - Priority tra layers
- ✅ `test_mode0_transparency` - Trasparenza color 0
- ✅ `test_bg_control_parsing` - Parsing BGxCNT
- ✅ `test_bg_screen_size` - Dimensioni screen base
- ✅ `test_palette_ram_access` - Lettura/scrittura palette

_Sprite Rendering (5 test):_

- ✅ `test_sprite_attribute_parsing` - Parsing OAM bytes
- ✅ `test_sprite_sizes` - Tutte le 12 dimensioni sprite
- ✅ `test_oam_read_write` - Lettura/scrittura OAM
- ✅ `test_sprite_rendering_simple` - Rendering sprite 8x8
- ✅ `test_sprite_transparency` - Trasparenza sprite (color 0)

**Integrazione (4 test):**

- ✅ `test_load_rom` - Caricamento ROM in memoria
- ✅ `test_basic_execution` - Esecuzione istruzioni base
- ✅ `test_interrupt_handling` - Gestione interrupts
- ✅ `test_memory_mirroring` - Mirroring BIOS/WRAM

Tutti i test passano con 0 warning Clippy strict mode.

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
