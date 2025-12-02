# Changelog

Tutte le modifiche notevoli a questo progetto saranno documentate in questo file.

Il formato è basato su [Keep a Changelog](https://keepachangelog.com/it/1.0.0/),
e questo progetto aderisce a [Semantic Versioning](https://semver.org/lang/it/).

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
