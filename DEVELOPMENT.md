# Guida allo Sviluppo - GBA Emulator

## 🏗️ Architettura

### Moduli Principali

1. **gba-arm7tdmi** - CPU ARM7TDMI

   - Registri e gestione modalità
   - Decodifica istruzioni ARM/THUMB
   - Pipeline a 3 stadi
   - Gestione interrupt

2. **gba-core** - Core dell'emulatore

   - Bus di sistema
   - Memory mapper
   - PPU (Picture Processing Unit)
   - APU (Audio Processing Unit)
   - Timer e DMA
   - Interrupt controller

3. **gba-frontend-sdl2** - Frontend desktop
   - Rendering con SDL2
   - Input handling
   - UI e menu

## 📚 Risorse Tecniche

### Documentazione GBA

- **GBATEK** - http://problemkaputt.de/gbatek.htm
- **TONC** - https://www.coranac.com/tonc/text/
- **ARM7TDMI Manual** - http://infocenter.arm.com/help/topic/com.arm.doc.ddi0210c/DDI0210B.pdf

### Repository di Riferimento

- **rustboyadvance-ng** - https://github.com/michelhe/rustboyadvance-ng
- **mGBA** - https://github.com/mgba-emu/mgba

## 🎯 Roadmap Sviluppo

### ✅ Fase 1: CPU (COMPLETATA)

- ✅ Struttura base registri con banking
- ✅ Gestione modalità CPU (7 modalità)
- ✅ **Implementazione completa istruzioni ARM (40+ istruzioni)**
- ✅ **Implementazione completa istruzioni THUMB (100+ varianti)**
- ✅ **Condition codes e barrel shifter**
- ✅ **10 test unitari che verificano correttezza**
- ✅ **Switch ARM↔THUMB funzionante**
- ✅ **Codice professionale: 0 warning Clippy**

**Risultato**: La CPU può eseguire codice GBA reale! Tutti i test passano. Codice pulito e professionale.

### ✅ Fase 2: Grafica Base (COMPLETATA)

- ✅ PPU timing base
- ✅ **Mode 3 rendering** (bitmap RGB555 240x160)
- ✅ **I/O registers** (DISPCNT, DISPSTAT, VCOUNT)
- ✅ **VBlank interrupt** integrato
- ✅ **Conversione RGB555→RGB888** per SDL2
- ✅ **4 test unitari** per rendering

**Risultato**: Il rendering bitmap funziona! Possiamo vedere pixel colorati sullo schermo.

### 🚧 Fase 3: Grafica Avanzata (PROSSIMA)

- [ ] **Mode 0 rendering** (tile-based per Pokémon)
- [ ] Sprite rendering (OAM)
- [ ] Background scrolling
- [ ] Window e effects
- [ ] Mode 1-2 (affine backgrounds)

**Obiettivo**: Vedere i giochi Pokémon completi!

### ✅ Fase 3: Input (COMPLETATA)

- ✅ **Input controller** (keyboard → GBA buttons)
- ✅ **KEYINPUT register** (0x04000130)
- ✅ **D-Pad completo** + A/B/L/R/Start/Select
- ✅ **SDL2 KeyDown/KeyUp** handling
- [ ] Mappatura tasti configurabile
- [ ] Timing input accurato

**Risultato**: I controlli funzionano! Possiamo interagire con i giochi.

### 🔜 Fase 4: Audio e Completezza

- [ ] APU base (4 canali GB + 2 DMA)
- [ ] Audio mixing
- [ ] DMA controller
- [ ] Timer hardware

**Obiettivo**: Sentire la musica dei giochi!

### 🔜 Fase 5: Salvataggi

- [ ] SRAM detection
- [ ] Flash memory
- [ ] EEPROM
- [ ] Save states

**Obiettivo**: Salvare i progressi di gioco!

### 🎯 Fase 6: Ottimizzazione

- [ ] Profiling performance
- [ ] Ottimizzazioni hotspot
- [ ] Cache-friendly memory layout

## 🧪 Testing

### Test ROM Consigliate

1. **AGS Aging Cartridge** - Test hardware
2. **Tonc Demo ROMs** - Test grafica
3. **Pokémon Emerald** - Test completo
4. **Pokémon Ruby/Sapphire** - Compatibilità
5. **Pokémon FireRed/LeafGreen** - Compatibilità

## 🔧 Compilazione

```bash
# Debug build
cargo build

# Release build (ottimizzato)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run --release -- rom.gba

# Benchmarks
cargo bench
```

## 📊 Performance Target

- **60 FPS** costanti
- **Latenza input** < 16ms
- **Consumo CPU** < 50% (single core moderno)
- **Memoria** < 100 MB

## 🐛 Debug

### Logging Levels

```bash
# Error only
RUST_LOG=error cargo run

# Info (default)
RUST_LOG=info cargo run

# Debug (verbose)
RUST_LOG=debug cargo run

# Trace (molto verbose)
RUST_LOG=trace cargo run
```

### Debugger

- Usare `rust-gdb` o `rust-lldb`
- VS Code con extension Rust Analyzer

## 🤝 Contribuire

1. Fork del repository
2. Crea feature branch
3. Implementa feature con test
4. Submetti Pull Request

## 📝 Note Implementative

### Timing CPU

- CPU Clock: 16.78 MHz
- Cicli per frame (60 FPS): 280,896
- Cicli per scanline: 1,232

### Memory Map

```
0x00000000-0x00003FFF   BIOS (16 KB)
0x02000000-0x0203FFFF   EWRAM (256 KB)
0x03000000-0x03007FFF   IWRAM (32 KB)
0x04000000-0x040003FF   I/O Registers
0x05000000-0x050003FF   Palette RAM (1 KB)
0x06000000-0x06017FFF   VRAM (96 KB)
0x07000000-0x070003FF   OAM (1 KB)
0x08000000-0x09FFFFFF   ROM (32 MB max)
0x0E000000-0x0E00FFFF   SRAM (64 KB max)
```

### Ottimizzazioni Implementate

- LTO (Link Time Optimization)
- Single codegen unit
- Release stripping
- Fast hash (ahash)
- Fast locks (parking_lot)
