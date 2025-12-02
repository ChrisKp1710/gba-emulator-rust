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
  - **Architettura modulare**: test separati in `cpu_tests.rs`
  - **10 test unitari** che verificano correttezza ✅
  - **Codice professionale**: 0 warning Clippy ✅
- **✅ PPU (Picture Processing Unit) Completa** 🎨

  - **Architettura modulare**: 11 moduli (`ppu_impl/`) + test separati
  - **Mode 0 - Tile Backgrounds**
    - 4 background layers (BG0-BG3) con tile 8x8
    - Palette RAM (1KB): 16 e 256 colori
    - BG Control (BGxCNT): priority, char/screen base, palette mode
    - BG Scrolling (BGxHOFS/VOFS) per tutti i layer
    - Layer compositing con priority e trasparenza
  - **Mode 3 - Bitmap RGB**
    - Rendering RGB555 240x160 pixel
    - I/O registers: DISPCNT, DISPSTAT, VCOUNT
    - VBlank interrupt integrato
  - **Mode 4 - Bitmap Paletted** 🎨
    - 8-bit indexed color (256 colori)
    - Risoluzione 240x160 pixel
    - Page flipping (2 frame buffer per double buffering)
    - Frame 0/1 selezionabili via DISPCNT bit 4
  - **Mode 5 - Bitmap RGB Small** 🎨
    - 16-bit RGB555 direct color
    - Risoluzione 160x128 pixel (centrato su 240x160)
    - Page flipping (2 frame buffer)
    - Bordi neri automatici per centering
  - **Affine Backgrounds** 🔄 **NUOVO**
    - Trasformazioni affini per BG2/BG3 (Mode 1-2)
    - Matrice 2×2: rotazione, scaling, shearing (PA/PB/PC/PD in 8.8 fixed-point)
    - Reference point (X/Y in 20.8 fixed-point)
    - Wraparound mode: tiling automatico
    - Clipping mode: out-of-bounds = transparent
    - Background sizes: 128x128, 256x256, 512x512, 1024x1024
    - Tile-based 8-bit paletted rendering
  - **Windows System** 🪟
    - WIN0/WIN1: Finestre rettangolari configurabili
    - WINOBJ: Window da sprite (OBJ window)
    - WINOUT: Controllo area fuori finestre
    - Priority: WIN0 > WIN1 > WINOBJ > WINOUT
    - Horizontal/vertical wrapping support
    - Per-window layer enable (BG0-3, OBJ, Blending)
  - **Blending & Effects** ✨
    - Alpha blending: Blend tra due layer (EVA/EVB coefficients)
    - Brightness increase: Fade to white (EVY coefficient)
    - Brightness decrease: Fade to black (EVY coefficient)
    - Target selection per layer (BG0-3, OBJ, backdrop)
    - RGB555 color math con clamping
  - **Sprite Rendering (OAM)**
    - 128 sprite con OAM 1KB
    - Tutte le dimensioni: 8x8, 16x16, 32x32, 64x64, wide, tall
    - OBJ palette (512 byte): 16 e 256 colori
    - H-flip/V-flip, priority, trasparenza
    - VRAM OBJ tile rendering (0x06010000+)
  - **65 test unitari** per PPU completo ✅

- **✅ APU (Audio Processing Unit) Completa** 🔊

  - **Architettura modulare**: 7 moduli (`apu_impl/`) + test separati
  - **GB Sound Channels**
    - Square wave 1-2 con sweep e duty cycle
    - Wave RAM channel con forme d'onda custom
    - Noise channel con LFSR
  - **Direct Sound**
    - DMA Audio A/B con FIFO 32 byte
    - Mixing 8-bit signed PCM
  - **Master Control**
    - GB channel mixer, volume, enable/disable
    - Direct Sound control e output
  - **17 test unitari** per APU completo ✅

- **✅ Timer System Completo** ⏱️

  - **Architettura modulare**: 4 moduli (`timer_impl/`) + test separati
  - **4 Hardware Timers (TM0-TM3)**
    - Prescaler: 1, 64, 256, 1024 CPU cycles
    - Counter 16-bit con reload automatico
    - IRQ su overflow configurabile
    - Cascade mode (timer chaining)
  - **Memory-mapped I/O**: `0x04000100-0x0400010E`
  - **13 test unitari** per tutti i timer features ✅

- **✅ DMA Controller Completo** 🚀

  - **Architettura modulare**: 4 moduli (`dma_impl/`) + test separati
  - **4 DMA Channels (DMA0-DMA3)**
    - Source/Destination address control
    - Transfer modes: 16-bit e 32-bit
    - Address modes: increment, decrement, fixed, reload
    - Timing triggers: Immediate, VBlank, HBlank, Special
    - Repeat mode e IRQ su completamento
  - **Priority system**: DMA0 (highest) → DMA3 (lowest)
  - **Memory-mapped I/O**: `0x040000B0-0x040000DE`
  - **19 test unitari** per tutti i DMA features ✅

- **✅ BIOS Calls (SWI) Completo** 🎯 **NUOVO**

  - **Architettura modulare**: 3 moduli (`bios_impl/`) + test separati
  - **Software Interrupt Handler**
    - State management: halt, interrupt wait
    - 30+ SWI function numbers definiti
  - **Math Functions**
    - Div/DivArm: divisione signed con remainder
    - Sqrt: radice quadrata intera
    - ArcTan/ArcTan2: funzioni trigonometriche
  - **Memory Operations**
    - CpuSet/CpuFastSet: copy/fill 16/32-bit
    - BitUnPack: decompressione bit-packed
  - **Decompression**
    - LZ77UnComp: decompressione LZ77 (WRAM/VRAM)
    - RLUnComp: Run-Length decompression (WRAM/VRAM)
  - **21 test unitari** per tutte le BIOS functions ✅

- **✅ Save System Completo** 💾 **NUOVO**

  - **Architettura modulare**: 6 moduli (`save_impl/`) + test separati
  - **Auto-Detection** 🔍
    - Scansione ROM per identificatori (SRAM_V, FLASH_V, EEPROM_V, etc.)
    - Dimensione EEPROM automatica (ROM >16MB → 8KB, else 512B)
  - **SRAM (32-64KB)** 📝
    - Battery-backed read/write semplice
    - Wraparound automatico, clear, load data
  - **Flash (64-128KB)** ⚡
    - State machine con chip ID, sector erase (4KB)
    - Bank switching per Flash 128KB (2 banks × 64KB)
    - Chip IDs: Macronix, Panasonic, Atmel, Sanyo
  - **EEPROM (512B-8KB)** 🔌
    - Protocollo seriale bit-by-bit via DMA
    - Trasferimenti 64-bit (8 byte blocks)
    - Address bits: 6 (512B) o 14 (8KB)
  - **File Persistence** 📂
    - Auto-save ogni frame se modificato
    - File `.sav` nella stessa directory della ROM
    - Load automatico all'avvio
  - **Memory mapping**: `0x0E000000-0x0E00FFFF`
  - **23 test unitari** per tutti i save types ✅

- **✅ Input Controller Completo**
  - KEYINPUT register (0x04000130)
  - D-Pad, A/B, L/R, Start/Select
  - Mappatura SDL2 completa
- **✅ Sistema Memoria Completo** - Memory mapping accurato per tutte le regioni GBA
- **✅ Sistema Interrupt** - Controller interrupt con IE/IF/IME
- **✅ Caricamento ROM** - Supporto completo con parsing header
- **✅ Frontend SDL2** - Interfaccia grafica 60 FPS con conversione RGB555→RGB888
- **✅ Ottimizzazione Massima** - LTO fat, single codegen unit, strip
- **✅ Architettura Professionale**
  - Codice modulare: ogni componente in moduli separati
  - Test sempre separati in `_tests.rs` files
  - 0 warning Clippy strict mode
  - Best practices Rust

### 🚧 In Sviluppo

- **Rendering Avanzato**
  - Integrazione affine rendering in Mode 1-2
  - Ottimizzazioni performance per trasformazioni

### 📋 Pianificato

- **Save States** - Salvataggio/caricamento stato emulatore completo
- **Ottimizzazioni Avanzate** - JIT compilation, SIMD

## 🏗️ Architettura

Il progetto è strutturato in crate separati per modularità e riusabilità:

```
gba-emulator-rust/
├── gba-core/           # Core dell'emulatore
│   ├── src/
│   │   ├── ppu_impl/   # PPU modularizzata (11 moduli)
│   │   ├── apu_impl/   # APU modularizzata (7 moduli)
│   │   ├── timer_impl/ # Timer modularizzato (4 moduli)
│   │   ├── dma_impl/   # DMA modularizzato (4 moduli)
│   │   ├── bios_impl/  # BIOS modularizzato (3 moduli)
│   │   ├── save_impl/  # Save System modularizzato (6 moduli)
│   │   ├── ppu.rs      # Re-export PPU
│   │   ├── apu.rs      # Re-export APU
│   │   ├── timer.rs    # Re-export Timer
│   │   ├── dma.rs      # Re-export DMA
│   │   ├── bios.rs     # Re-export BIOS
│   │   ├── save.rs     # Re-export Save
│   │   ├── bus.rs      # System bus e I/O mapping
│   │   ├── memory.rs   # Memory management
│   │   └── ...
│   └── tests/          # Integration tests
├── gba-arm7tdmi/       # CPU ARM7TDMI
│   ├── src/
│   │   ├── cpu.rs      # Core CPU (781 lines)
│   │   └── cpu_tests.rs # Test separati (426 lines)
├── gba-frontend-sdl2/  # Frontend desktop SDL2
└── Cargo.toml          # Workspace configuration
```

### 🎯 Principi Architetturali

1. **Modularità**: Ogni componente è suddiviso in moduli piccoli e focalizzati (~20-250 righe)
2. **Test Separati**: Tutti i test sono in file `_tests.rs` dedicati
3. **Zero Warnings**: Clippy strict mode, 0 warning policy
4. **Best Practices**: Rust idiomatico, documentazione, type safety

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
   - ✅ Test separati in cpu_tests.rs
   - ✅ 10 test unitari passano
2. **PPU (Picture Processing Unit) completa** 🎨
   - ✅ Architettura modulare (11 moduli in ppu_impl/)
   - ✅ Mode 0 (tile backgrounds) - 4 layers
   - ✅ Mode 3 (bitmap RGB555)
   - ✅ Mode 4 (bitmap paletted 8-bit)
   - ✅ Mode 5 (bitmap RGB small 160x128)
   - ✅ Affine backgrounds (BG2/BG3 rotation/scaling)
   - ✅ Windows (WIN0/WIN1/WINOBJ/WINOUT)
   - ✅ Blending (alpha, brightness +/-)
   - ✅ Sprite rendering (OAM) - 128 sprite
   - ✅ Palette RAM e scrolling
   - ✅ Test separati: 65 test unitari
3. **APU (Audio Processing Unit) completa** 🔊
   - ✅ Architettura modulare (7 moduli in apu_impl/)
   - ✅ 4 GB sound channels (Square, Wave, Noise)
   - ✅ Direct Sound (DMA Audio A/B)
   - ✅ Mixer e master control
   - ✅ Test separati: 17 test unitari
4. **Timer System completo** ⏱️
   - ✅ Architettura modulare (4 moduli in timer_impl/)
   - ✅ 4 hardware timers (TM0-TM3)
   - ✅ Prescaler, cascade mode, IRQ
   - ✅ Test separati: 13 test unitari
5. **DMA Controller completo** 🚀
   - ✅ Architettura modulare (4 moduli in dma_impl/)
   - ✅ 4 DMA channels con priority
   - ✅ Transfer modes, address control, timing
   - ✅ Test separati: 19 test unitari
6. **BIOS Calls (SWI) completo** 🎯
   - ✅ Architettura modulare (3 moduli in bios_impl/)
   - ✅ Software interrupt handler con state management
   - ✅ Math functions (Div, Sqrt, ArcTan)
   - ✅ Memory operations (CpuSet, CpuFastSet)
   - ✅ Decompression (LZ77, RLE)
   - ✅ Test separati: 21 test unitari
7. **Save System completo** 💾
   - ✅ Architettura modulare (6 moduli in save_impl/)
   - ✅ Auto-detection da ROM (SRAM_V, FLASH_V, EEPROM_V)
   - ✅ SRAM (32-64KB), Flash (64-128KB), EEPROM (512B-8KB)
   - ✅ State machine Flash con chip ID e sector erase
   - ✅ Protocollo seriale EEPROM via DMA
   - ✅ File persistence (.sav) con auto-save
   - ✅ Test separati: 23 test unitari
8. **Input controller completo**
   - ✅ KEYINPUT register
   - ✅ D-Pad + A/B/L/R/Start/Select
   - ✅ SDL2 integration
9. **Sistema base completo**
   - ✅ Memoria e bus
   - ✅ Interrupt controller
   - ✅ Caricamento ROM
   - ✅ Frontend SDL2

**Totale: 179 test unitari ✅**

### 🚧 In Corso

- **Rendering Integration**
  - [ ] Affine rendering in Mode 1-2
  - [ ] Performance optimizations

### 📋 Pianificato

1. **Periferiche Avanzate**

   - [ ] Serial communication
   - [ ] RTC (Real Time Clock)
   - [ ] GPIO per accessori

2. **Altri Features**

   - [ ] Save States

3. **Ottimizzazioni**
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
# Run tutti i test (179 test totali)
cargo test --workspace

# Test CPU ARM7TDMI (10 test unitari)
cargo test --package gba-arm7tdmi

# Test PPU (65 test unitari)
cargo test --package gba-core ppu

# Test APU (17 test unitari)
cargo test --package gba-core apu

# Test Timer (13 test unitari)
cargo test --package gba-core timer

# Test DMA (19 test unitari)
cargo test --package gba-core dma

# Test BIOS (21 test unitari)
cargo test --package gba-core bios

# Test Save System (23 test unitari)
cargo test --package gba-core save

# Test integrazione (4 test)
cargo test --package gba-core --test
```

### Test Suite - 151/151 Passano ✅

**CPU ARM7TDMI (10 test)** - `cpu_tests.rs`

- ✅ `test_mov_instruction` - MOV con immediato
- ✅ `test_add_instruction` - ADD tra registri
- ✅ `test_branch_instruction` - Branch (B)
- ✅ `test_ldr_str_instructions` - LDR/STR memoria
- ✅ `test_cpu_creation` e `test_cpu_reset` - Base CPU
- ✅ `test_thumb_mov_immediate` - THUMB MOV immediato
- ✅ `test_thumb_add_subtract` - THUMB ADD/SUB registri
- ✅ `test_thumb_ldr_str` - THUMB LDR/STR con offset
- ✅ `test_thumb_branch` - THUMB Branch incondizionale

**PPU Rendering (22 test)** - `ppu.rs`

_Mode 0 - Tile Backgrounds (7 test):_

- ✅ `test_mode0_simple_tile` - Rendering tile 8x8 base
- ✅ `test_mode0_scrolling` - Scrolling BG layers
- ✅ `test_mode0_priority` - Priority tra layers
- ✅ `test_mode0_transparency` - Trasparenza color 0
- ✅ `test_bg_control_parsing` - Parsing BGxCNT
- ✅ `test_bg_screen_size` - Dimensioni screen base
- ✅ `test_palette_ram_access` - Lettura/scrittura palette

_Mode 4 - Bitmap Paletted (4 test):_ **NUOVO** 🎨

- ✅ `test_mode4_basic_render` - Rendering 8-bit indexed
- ✅ `test_mode4_page_flip` - Frame 0/1 switching
- ✅ `test_mode4_256_colors` - 256-color palette gradient
- ✅ `test_mode4_scanline_offset` - Scanline offsetting

_Mode 5 - Bitmap RGB (6 test):_ **NUOVO** 🎨

- ✅ `test_mode5_basic_render` - 16-bit RGB rendering
- ✅ `test_mode5_page_flip` - Frame buffer flipping
- ✅ `test_mode5_dimensions` - 160x128 resolution
- ✅ `test_mode5_out_of_bounds` - Scanline > 128 handling
- ✅ `test_mode5_centering` - Centering su 240x160
- ✅ `test_mode5_gradient` - RGB gradient test

_Sprite Rendering (5 test):_

- ✅ `test_sprite_attribute_parsing` - Parsing OAM bytes
- ✅ `test_sprite_sizes` - Tutte le 12 dimensioni sprite
- ✅ `test_oam_read_write` - Lettura/scrittura OAM
- ✅ `test_sprite_rendering_simple` - Rendering sprite 8x8
- ✅ `test_sprite_transparency` - Trasparenza sprite (color 0)

**APU Audio (17 test)** - `apu_tests.rs`

_Channels (9 test):_

- ✅ `test_square_channel_creation` - Square channel init
- ✅ `test_duty_cycle` - Duty cycle 12.5%, 25%, 50%, 75%
- ✅ `test_trigger` - Square trigger e reset
- ✅ `test_wave_ram_access` - Wave RAM read/write
- ✅ `test_wave_trigger` - Wave trigger
- ✅ `test_noise_creation` - Noise channel init
- ✅ `test_noise_trigger` - Noise trigger e LFSR

_Direct Sound (3 test):_

- ✅ `test_fifo_write_read` - FIFO buffer operations
- ✅ `test_fifo_reset` - FIFO clear
- ✅ `test_fifo_wraparound` - FIFO circular buffer

_System (5 test):_

- ✅ `test_apu_creation` - APU initialization
- ✅ `test_master_enable` - Master enable/disable
- ✅ `test_register_routing` - Register mapping
- ✅ `test_gb_volume` - GB volume control
- ✅ `test_mixer_silence` - Mixer output

**Timer System (13 test)** - `timer_tests.rs`

_Core Features (7 test):_

- ✅ `test_timer_creation` - Timer initialization
- ✅ `test_timer_control_register` - Control register parsing
- ✅ `test_timer_reload` - Reload value handling
- ✅ `test_timer_counting` - Basic counting
- ✅ `test_timer_overflow` - Overflow e reload
- ✅ `test_timer_overflow_irq` - IRQ su overflow
- ✅ `test_timer_disabled_no_count` - Timer disabilitato

_Prescaler (4 test):_

- ✅ `test_prescaler_64` - Prescaler 64 cycles
- ✅ `test_prescaler_256` - Prescaler 256 cycles
- ✅ `test_prescaler_1024` - Prescaler 1024 cycles

_Advanced (2 test):_

- ✅ `test_cascade_mode` - Timer chaining
- ✅ `test_all_timers` - Tutti e 4 i timer
- ✅ `test_timer_enable_reloads` - Enable behavior

**DMA Controller (19 test)** - `dma_tests.rs`

_Core Features (8 test):_

- ✅ `test_dma_creation` - DMA initialization
- ✅ `test_dma_control_register` - Control register parsing
- ✅ `test_dma_register_write_read` - Register I/O
- ✅ `test_dma_source_mask` - Source address masking
- ✅ `test_dma_dest_mask` - Dest address masking
- ✅ `test_dma_word_count` - Word count handling
- ✅ `test_dma_irq_flag` - IRQ generation
- ✅ `test_dma_no_irq_when_disabled` - IRQ control

_Timing Modes (4 test):_

- ✅ `test_dma_timing_enum` - Timing enum parsing
- ✅ `test_dma_immediate_trigger` - Immediate mode
- ✅ `test_dma_vblank_trigger` - VBlank trigger
- ✅ `test_dma_hblank_trigger` - HBlank trigger

_Transfer Modes (5 test):_

- ✅ `test_dma_32bit_transfer` - 32-bit transfer
- ✅ `test_dma_address_increment` - Address increment
- ✅ `test_dma_address_decrement` - Address decrement
- ✅ `test_dma_address_fixed` - Fixed address
- ✅ `test_dma_repeat_mode` - Repeat mode

_Advanced (2 test):_

- ✅ `test_dma_priority` - Channel priority
- ✅ `test_dma_reset` - DMA reset

**BIOS Calls (21 test)** - `bios_tests.rs`

_State Management (8 test):_

- ✅ `test_bios_creation` - BIOS initialization
- ✅ `test_bios_reset` - State reset
- ✅ `test_bios_halt` - Halt state
- ✅ `test_bios_stop` - Stop state
- ✅ `test_bios_vblank_wait` - VBlank interrupt wait
- ✅ `test_bios_intr_wait` - Generic interrupt wait
- ✅ `test_bios_clear_halt` - Clear halt
- ✅ `test_bios_clear_wait` - Clear wait

_Math Functions (7 test):_

- ✅ `test_div_normal` - Division normale
- ✅ `test_div_negative` - Division con negativi
- ✅ `test_div_by_zero` - Division by zero handling
- ✅ `test_sqrt_perfect` - Square root perfetta
- ✅ `test_sqrt_imperfect` - Square root imperfetta
- ✅ `test_arctan_zero` - ArcTan zero
- ✅ `test_arctan_positive` - ArcTan positive
- ✅ `test_arctan2_quadrants` - ArcTan2 quadrants
- ✅ `test_arctan2_zero` - ArcTan2 zero

_Core Features (6 test):_

- ✅ `test_swi_constants` - SWI constants
- ✅ `test_cpuset_flags` - CpuSet flags
- ✅ `test_soft_reset_no_panic` - SoftReset
- ✅ `test_bios_unknown_swi` - Unknown SWI handling

**Save System (23 test)** - `save_tests.rs`

_SaveType & Detection (9 test):_

- ✅ `test_save_type_size` - SaveType sizes
- ✅ `test_save_type_is_flash` - Flash detection
- ✅ `test_save_type_is_eeprom` - EEPROM detection
- ✅ `test_detect_sram` - SRAM_V detection
- ✅ `test_detect_flash_64k` - FLASH_V 64KB
- ✅ `test_detect_flash_128k` - FLASH1M_V 128KB
- ✅ `test_detect_eeprom_512b` - EEPROM_V 512B
- ✅ `test_detect_eeprom_8k` - EEPROM_V 8KB (ROM >16MB)
- ✅ `test_detect_none` - No save type

_SRAM (2 test):_

- ✅ `test_sram_basic` - Read/write, wraparound
- ✅ `test_sram_default_empty` - Default 0xFF

_Flash (3 test):_

- ✅ `test_flash_chip_id` - Chip ID mode
- ✅ `test_flash_write_byte` - Byte write sequence
- ✅ `test_flash_erase_sector` - 4KB sector erase
- ✅ `test_flash_bank_switch` - 128KB bank switching

_EEPROM (2 test):_

- ✅ `test_eeprom_basic` - Read/write 64-bit blocks
- ✅ `test_eeprom_8k` - 14-bit address mode

_SaveController (4 test):_

- ✅ `test_save_controller_no_save` - SaveType::None
- ✅ `test_save_controller_detection` - Auto-detect from ROM
- ✅ `test_save_controller_read_write` - Read/write operations
- ✅ `test_save_controller_flash` - Flash controller

_File Persistence (3 test):_

- ✅ `test_save_file_persistence` - Save/load file
- ✅ `test_flash_file_persistence` - Flash file persistence
- ✅ `test_eeprom_file_persistence` - EEPROM file persistence

**Integrazione (4 test)** - `tests/`

- ✅ `test_mode3_rendering` - PPU Mode 3 bitmap
- ✅ `test_mode3_full_scanline` - Scanline completa
- ✅ `test_demo_color_gradient` - Demo gradiente
- ✅ `test_demo_color_bars` - Demo barre colorate

**Qualità del codice:**

- ✅ 0 warning Clippy strict mode (`-D warnings`)
- ✅ Tutti i test passano
- ✅ Codice modulare e documentato

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

```
