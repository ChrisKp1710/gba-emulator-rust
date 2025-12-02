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

## 🎯 Prossimi Passi

### Fase 1: CPU (In Corso)
- [x] Struttura base registri
- [x] Gestione modalità CPU
- [ ] Implementazione istruzioni ARM
- [ ] Implementazione istruzioni THUMB
- [ ] Pipeline CPU
- [ ] Test suite ARM7TDMI

### Fase 2: Memoria e Bus
- [x] Memory mapper base
- [ ] Timing accurato
- [ ] DMA controller
- [ ] Gestione waitstates

### Fase 3: Grafica
- [ ] PPU base
- [ ] Background rendering (Mode 0-2)
- [ ] Sprite rendering
- [ ] Modalità bitmap (Mode 3-5)
- [ ] Effects (alpha blending, mosaic)

### Fase 4: Audio
- [ ] Channel 1-4 (GB compatibili)
- [ ] DMA audio channels
- [ ] Audio mixing

### Fase 5: Ottimizzazione
- [ ] Profiling e hotspot identification
- [ ] JIT compilation (opzionale)
- [ ] SIMD optimizations
- [ ] Multi-threading

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
