# Changelog

Tutte le modifiche notevoli a questo progetto saranno documentate in questo file.

Il formato è basato su [Keep a Changelog](https://keepachangelog.com/it/1.0.0/),
e questo progetto aderisce a [Semantic Versioning](https://semver.org/lang/it/).

## [0.3.0] - 2024-12-02

### ✅ Completato - Graphics & Input

#### PPU (Picture Processing Unit)

- ✅ **Mode 3 Rendering** in `ppu.rs` (~200 righe):
  - Rendering bitmap RGB555 240x160
  - Metodo `render_mode3_scanline()` per copia VRAM→framebuffer
  - DisplayMode enum (Mode 0-5 supportati)
  - I/O registers: DISPCNT, DISPSTAT, VCOUNT
  - Timing: 960 cicli/scanline, 227 scanline/frame
  - VBlank interrupt integrato

#### Bus I/O

- ✅ **I/O Register Routing** in `bus.rs`:
  - `read_io_halfword()` e `write_io_halfword()` (~100 righe)
  - PPU registers: 0x04000000-0x04000006
  - Interrupt registers: 0x04000200-0x04000208
  - Input register: 0x04000130 (KEYINPUT)

#### Input Controller

- ✅ **InputController** in `input.rs` (~130 righe):
  - KEYINPUT register (bit invertiti: 0=premuto, 1=rilasciato)
  - D-Pad completo (Up, Down, Left, Right)
  - Pulsanti: A, B, L, R, Start, Select
  - Integrazione SDL2 con KeyDown/KeyUp events

#### SDL2 Frontend

- ✅ **Conversione Framebuffer** in `ui.rs`:
  - RGB555→RGB888 per SDL2 texture
  - Espansione bit corretta: `r8 = (r5 << 3) | (r5 >> 2)`
  - Mapping tastiera completo (frecce, Z/X, A/S, Enter/Backspace)

#### Testing

- ✅ **PPU Tests** in `ppu_mode3_test.rs` (2 test):
  - `test_mode3_rendering`: Pixel colorati (rosso, verde, blu, bianco)
  - `test_mode3_full_scanline`: Gradiente rosso
  - Totale: 12 test (10 CPU + 2 PPU) - Tutti passano ✅

## [0.2.0] - 2024-12-02

### ✅ Completato - CPU ARM7TDMI Funzionante

#### Istruzioni ARM (32-bit)

- ✅ **Decoder ARM completo** in `arm.rs`:
  - Data Processing (16 operazioni: AND, EOR, SUB, RSB, ADD, ADC, SBC, RSC, TST, TEQ, CMP, CMN, ORR, MOV, BIC, MVN)
  - Branch and Branch with Link (B, BL)
  - Branch and Exchange (BX) - switch ARM↔THUMB
  - Single Data Transfer (LDR, STR, LDRB, STRB)
  - Block Data Transfer (LDM, STM)
  - Multiply (MUL, MLA)
  - Software Interrupt (SWI)
- ✅ **Implementazioni complete**:
  - `instructions/alu.rs` (314 righe) - Tutte le operazioni ALU con barrel shifter
  - `instructions/branch.rs` (66 righe) - B, BL, BX
  - `instructions/load_store.rs` (173 righe) - LDR/STR singolo e multiplo
- ✅ **Condition codes**: Tutte le 15 condizioni (EQ, NE, CS, CC, MI, PL, VS, VC, HI, LS, GE, LT, GT, LE, AL)
- ✅ **Barrel shifter**: LSL, LSR, ASR, ROR con carry out corretto
- ✅ **Flag NZCV**: Gestione completa flag Negative, Zero, Carry, Overflow

#### Istruzioni THUMB (16-bit)

- ✅ **Decoder THUMB completo** in `thumb.rs` (350+ righe):
  - Tutti i 19 formati THUMB decodificati
  - 100+ varianti di istruzioni supportate
- ✅ **Esecuzione THUMB** in `cpu.rs` (450+ righe):
  - Format 1: Move shifted register (LSL, LSR, ASR)
  - Format 2: Add/subtract (ADD, SUB con registro o immediato)
  - Format 3: Move/compare/add/subtract immediate (MOV, CMP, ADD, SUB)
  - Format 4: ALU operations (16 operazioni complete)
  - Format 5: Hi register operations/BX (R8-R15)
  - Format 6: PC-relative load
  - Format 7-8: Load/store register offset e sign-extended
  - Format 9-10: Load/store immediate offset e halfword
  - Format 11: SP-relative load/store
  - Format 12-13: Load address e add offset to SP
  - Format 14-15: Push/pop registers e multiple load/store
  - Format 16: Conditional branch
  - Format 17: Software interrupt
  - Format 18: Unconditional branch
  - Format 19: Long branch with link (BL)

#### Testing

- ✅ **10 test unitari** che verificano correttezza:
  - **ARM**: MOV, ADD, Branch, LDR/STR
  - **THUMB**: MOV immediato, ADD registri, LDR/STR, Branch
- ✅ Tutti i test passano con successo
- ✅ Compilazione release senza errori

#### Miglioramenti Registri

- ✅ Metodi setter per flag: `set_flag_n()`, `set_flag_z()`, `set_flag_c()`, `set_flag_v()`
- ✅ Metodo `set_thumb()` per switch ARM/THUMB mode
- ✅ Accesso diretto CPSR per condition checks

### Aggiunte Documentazione

- 📚 **MAPPA_PROGETTO.md** - Guida visuale navigazione codice
- 📚 **GUIDA_ARCHITETTURA.md** - Architettura step-by-step dettagliata
- 📚 Commenti inline estesi in tutti i file principali

### Note Tecniche v0.2.0

- **Righe codice CPU**: ~1.600 righe funzionanti
- **Istruzioni supportate**: 60+ ARM + 100+ varianti THUMB
- **Performance test**: Tutti passano in <1 secondo
- **Compatibilità**: Il codice GBA reale può ora essere eseguito dalla CPU!

### ⚠️ Limitazioni Correnti

- PPU rendering non implementato (solo timing)
- Input controller da collegare
- APU non implementato
- Save states non implementati
- Nessun gioco ancora completamente giocabile (manca grafica)

---

## [0.1.0] - 2024-12-02

### Aggiunto

- ✨ Struttura base del progetto con workspace Rust
- ✨ Modulo CPU ARM7TDMI con:
  - Registri completi (R0-R15, banked registers)
  - Gestione modalità CPU (User, FIQ, IRQ, SVC, ABT, UND, SYS)
  - Status flags (NZCV)
  - Supporto CPU state (ARM/THUMB)
  - Gestione interrupt base
- ✨ Sistema memoria completo con memory mapping GBA:
  - BIOS (16 KB)
  - EWRAM (256 KB)
  - IWRAM (32 KB)
  - I/O Registers
  - Palette RAM (1 KB)
  - VRAM (96 KB)
  - OAM (1 KB)
  - ROM (32 MB max)
  - SRAM (64 KB max)
- ✨ PPU (Picture Processing Unit) base:
  - Framebuffer 240x160
  - Timing scanline accurato
  - VBlank detection
- ✨ Sistema interrupt controller:
  - Registro IE (Interrupt Enable)
  - Registro IF (Interrupt Flags)
  - IME (Interrupt Master Enable)
  - Supporto per tutti i tipi di interrupt GBA
- ✨ Caricamento cartridge:
  - Parsing ROM GBA
  - Lettura header (titolo, game code, maker code, version)
- ✨ Frontend SDL2:
  - Rendering 240x160 con scaling x3
  - Loop principale a 60 FPS
  - Gestione eventi input
  - Controllo framerate
- ✨ Build system ottimizzato:
  - LTO (Link Time Optimization)
  - Single codegen unit
  - Strip binaries
  - Panic abort
- ✨ Script PowerShell per:
  - Download automatico SDL2
  - Build automatizzato
  - Run con ROM
- 📚 Documentazione completa:
  - README con guida uso
  - DEVELOPMENT.md con dettagli architettura
  - QUICKSTART.md per inizio rapido
  - SDL2_SETUP.md per installazione
  - Commenti inline nel codice

### In Sviluppo

- 🚧 Implementazione istruzioni ARM complete
- 🚧 Implementazione istruzioni THUMB complete
- 🚧 Pipeline CPU a 3 stadi
- 🚧 Rendering grafico (background, sprites)
- 🚧 APU (Audio Processing Unit)
- 🚧 Input controller funzionante
- 🚧 DMA controller
- 🚧 Timer hardware
- 🚧 Save states
- 🚧 Supporto salvataggi (SRAM, Flash, EEPROM)

### Note Tecniche

- Linguaggio: Rust 2021 Edition
- Architettura: Modulare (workspace con 3 crate)
- Performance: Compilazione ottimizzata con LTO fat
- Dipendenze principali: SDL2, serde, bitflags, anyhow
- Piattaforme: Windows, Linux, macOS

### Compatibilità

- ✅ Compila su Windows con MSVC
- ✅ Compila su Linux (testato su Debian/Ubuntu)
- ✅ Compila su macOS
- ⚠️ Emulazione CPU non ancora completa
- ⚠️ Nessun gioco ancora giocabile

---

## [Unreleased]

### Pianificato per v0.2.0

- Implementazione completa istruzioni ARM
- Implementazione completa istruzioni THUMB
- Pipeline CPU accurata
- Test suite per CPU

### Pianificato per v0.3.0

- Background rendering (Mode 0-2)
- Sprite rendering base
- Test con ROM demo

### Pianificato per v0.4.0

- Audio base (4 channels GB)
- Input controller completo
- Primi giochi giocabili

### Pianificato per v1.0.0

- Emulazione accurata e completa
- Compatibilità Pokémon Gen III
- Save states
- Salvataggi SRAM/Flash/EEPROM
- Performance ottimali (60 FPS costanti)

---

## Convenzioni

Tipi di cambiamenti:

- `Aggiunto` per nuove funzionalità
- `Modificato` per cambiamenti in funzionalità esistenti
- `Deprecato` per funzionalità presto rimosse
- `Rimosso` per funzionalità rimosse
- `Corretto` per bug fix
- `Sicurezza` per vulnerabilità

[0.1.0]: https://github.com/yourrepo/releases/tag/v0.1.0
