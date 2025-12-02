# Architettura Modulare - GBA Emulator

## 📐 Principi di Design

Il progetto segue questi principi fondamentali:

1. **Modularità**: Ogni componente è suddiviso in moduli piccoli (~20-250 righe)
2. **Test Separati**: Tutti i test sono in file `_tests.rs` dedicati
3. **Zero Warnings**: Clippy strict mode (`-D warnings`)
4. **Best Practices**: Rust idiomatico, documentazione completa

## 🏗️ Struttura del Progetto

```
gba-emulator-rust/
├── gba-arm7tdmi/          # CPU ARM7TDMI
│   ├── src/
│   │   ├── cpu.rs         # Core CPU (781 lines)
│   │   ├── cpu_tests.rs   # Test CPU (426 lines)
│   │   ├── registers.rs   # Register file (351 lines)
│   │   ├── thumb.rs       # THUMB decoder (348 lines)
│   │   └── lib.rs
│   └── Cargo.toml
│
├── gba-core/              # Sistema GBA
│   ├── src/
│   │   ├── ppu_impl/      # PPU modularizzata
│   │   │   ├── constants.rs   # (35 lines) - Memory map, registri
│   │   │   ├── types.rs       # (53 lines) - BgControl, DisplayMode
│   │   │   ├── sprites.rs     # (224 lines) - Sprite rendering
│   │   │   ├── mode0.rs       # (173 lines) - Tile backgrounds
│   │   │   ├── mode3.rs       # (20 lines) - Bitmap mode
│   │   │   └── mod.rs         # (247 lines) - PPU struct principale
│   │   │
│   │   ├── apu_impl/      # APU modularizzata
│   │   │   ├── constants.rs   # (26 lines) - Registri audio
│   │   │   ├── registers.rs   # (105 lines) - Master control
│   │   │   ├── mixer.rs       # (120 lines) - Audio mixing
│   │   │   ├── direct_sound.rs # (100 lines) - DMA audio
│   │   │   ├── channels/
│   │   │   │   ├── square.rs  # (154 lines) - Square wave
│   │   │   │   ├── wave.rs    # (123 lines) - Wave RAM
│   │   │   │   ├── noise.rs   # (110 lines) - Noise channel
│   │   │   │   └── mod.rs     # (14 lines) - Channel exports
│   │   │   └── mod.rs         # (216 lines) - APU struct principale
│   │   │
│   │   ├── timer_impl/    # Timer modularizzato
│   │   │   ├── constants.rs   # (18 lines) - Timer registers
│   │   │   ├── registers.rs   # (34 lines) - TimerControl
│   │   │   ├── counter.rs     # (90 lines) - TimerCounter logic
│   │   │   └── mod.rs         # (89 lines) - Timer struct
│   │   │
│   │   ├── dma_impl/      # DMA modularizzato
│   │   │   ├── constants.rs   # (34 lines) - DMA registers
│   │   │   ├── types.rs       # (59 lines) - DmaControl, DmaTiming
│   │   │   ├── channel.rs     # (171 lines) - DmaChannel logic
│   │   │   └── mod.rs         # (119 lines) - DMA struct
│   │   │
│   │   ├── ppu.rs         # (2 lines) - Re-export PPU
│   │   ├── apu.rs         # (2 lines) - Re-export APU
│   │   ├── timer.rs       # (2 lines) - Re-export Timer
│   │   ├── dma.rs         # (2 lines) - Re-export DMA
│   │   ├── timer_tests.rs # (194 lines) - Timer tests
│   │   ├── dma_tests.rs   # (300 lines) - DMA tests
│   │   ├── bus.rs         # (290 lines) - System bus
│   │   ├── memory.rs      # (310 lines) - Memory system
│   │   ├── interrupt.rs   # (85 lines) - Interrupts
│   │   ├── input.rs       # (120 lines) - Input controller
│   │   └── lib.rs
│   │
│   └── tests/
│       ├── ppu_mode3_test.rs  # PPU integration
│       └── ppu_visual_test.rs # Visual demos
│
├── gba-frontend-sdl2/     # Frontend grafico
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
│
└── Cargo.toml             # Workspace root
```

## 📊 Metriche del Codice

### Per Componente

| Componente | Moduli    | Righe Codice | Righe Test | Test | Status      |
| ---------- | --------- | ------------ | ---------- | ---- | ----------- |
| **CPU**    | 1 + tests | 781          | 426        | 10   | ✅ Completo |
| **PPU**    | 6 + tests | 752          | in ppu.rs  | 12   | ✅ Completo |
| **APU**    | 7 + tests | 952          | separati   | 17   | ✅ Completo |
| **Timer**  | 4 + tests | 231          | 194        | 13   | ✅ Completo |
| **DMA**    | 4 + tests | 383          | 300        | 19   | ✅ Completo |
| **Bus**    | 1         | 290          | -          | 0    | ✅ Stabile  |
| **Memory** | 1         | 310          | -          | 0    | ✅ Stabile  |
| **Input**  | 1         | 120          | -          | 0    | ✅ Completo |

**Totale Test Suite: 75 test** (10 CPU + 12 PPU + 17 APU + 13 Timer + 19 DMA + 4 integration)

### Dimensione File (Policy: max ~250 righe)

**File Grandi (>250 righe):**

- `cpu.rs`: 781 righe ⚠️ (ma critico, non refactorizzabile)
- `thumb.rs`: 348 righe ⚠️ (decoder, complesso ma stabile)
- `registers.rs`: 351 righe ⚠️ (CPU registers, ok)
- `memory.rs`: 310 righe ⚠️ (memory mapping, ok)
- `bus.rs`: 290 righe ✅ (I/O routing, ok)

**File Moderni (modulari):**

- PPU: 6 moduli da 20-247 righe ✅
- APU: 7 moduli da 14-216 righe ✅
- Timer: 4 moduli da 18-90 righe ✅

## 🎯 Pattern Architetturali

### 1. Modularizzazione (PPU, APU, Timer)

Ogni sistema complesso è suddiviso in:

```rust
// Struttura directory
component_impl/
├── constants.rs    // Memory map, register addresses
├── types.rs        // Structs e enums
├── registers.rs    // Register control logic
├── sub_module.rs   // Funzionalità specifiche
└── mod.rs          // Struct principale + pub exports

// File pubblico (re-export + tests)
component.rs        // pub use component_impl::*;

// Test separati
component_tests.rs  // #[cfg(test)] mod in lib.rs
```

**Esempio PPU:**

```rust
// gba-core/src/ppu_impl/mod.rs
pub struct PPU {
    // Campi interni
}

impl PPU {
    pub fn new() -> Self { ... }
    pub fn step(&mut self, cycles: u32) { ... }
}

// gba-core/src/ppu.rs
pub use crate::ppu_impl::*;

// gba-core/src/lib.rs
mod ppu_impl;  // Privato
pub mod ppu;   // Pubblico
```

### 2. Test Separati

**Policy**: SEMPRE separare test dall'implementazione

```rust
// ❌ VECCHIO MODO (evitare)
// file.rs
impl MyStruct { ... }

#[cfg(test)]
mod tests {
    // 200+ righe di test qui
}

// ✅ NUOVO MODO (corretto)
// file.rs
impl MyStruct { ... }

// file_tests.rs
use crate::file::*;

#[test]
fn test_feature_1() { ... }

#[test]
fn test_feature_2() { ... }

// lib.rs
#[cfg(test)]
mod file_tests;
```

**Vantaggi:**

- File più piccoli e leggibili
- Separazione logica codice/test
- Compilazione test più veloce
- Facile trovare e aggiungere test

### 3. Re-export Pattern

Per mantenere API pubblica pulita:

```rust
// internal_impl/mod.rs
pub struct InternalType { ... }
pub const CONSTANT: u32 = 0x123;

// public_api.rs
pub use crate::internal_impl::*;

// lib.rs
mod internal_impl;  // Privato, non visibile fuori crate
pub mod public_api; // Pubblico, API del crate
```

### 4. Costanti Centralizzate

Ogni modulo ha `constants.rs`:

```rust
// timer_impl/constants.rs
pub const TM0CNT_L: u32 = 0x04000100;
pub const TM0CNT_H: u32 = 0x04000102;
pub const PRESCALER_64: u32 = 64;

// Usato da tutti i moduli del timer
```

## 🔧 Come Aggiungere Nuove Feature

### Step-by-Step

1. **Crea directory modulo**

   ```bash
   mkdir gba-core/src/new_feature_impl
   ```

2. **Crea moduli base**

   ```rust
   // new_feature_impl/constants.rs
   pub const REGISTER_ADDR: u32 = 0x04000XYZ;

   // new_feature_impl/types.rs
   pub struct FeatureControl { ... }

   // new_feature_impl/mod.rs
   mod constants;
   mod types;
   pub use constants::*;
   pub use types::*;

   pub struct Feature { ... }
   impl Feature {
       pub fn new() -> Self { ... }
       pub fn step(&mut self) { ... }
   }
   ```

3. **Crea file pubblico**

   ```rust
   // gba-core/src/new_feature.rs
   pub use crate::new_feature_impl::*;
   ```

4. **Crea test separati**

   ```rust
   // gba-core/src/new_feature_tests.rs
   use crate::new_feature::*;

   #[test]
   fn test_creation() {
       let feature = Feature::new();
       assert_eq!(feature.status(), 0);
   }
   ```

5. **Aggiorna lib.rs**

   ```rust
   // gba-core/src/lib.rs
   mod new_feature_impl;
   pub mod new_feature;
   #[cfg(test)]
   mod new_feature_tests;
   ```

6. **Integra nel Bus**

   ```rust
   // bus.rs
   use crate::new_feature::Feature;

   pub struct Bus {
       pub feature: Feature,
       // ...
   }

   fn read_io_halfword(&mut self, addr: u32) -> u16 {
       match addr & !1 {
           0x04000XYZ => self.feature.read_register(addr),
           // ...
       }
   }
   ```

7. **Test e Clippy**
   ```bash
   cargo test --package gba-core new_feature
   cargo clippy --package gba-core -- -D warnings
   ```

## 📝 Regole di Qualità

### Code Quality Checklist

- [ ] Ogni file < 300 righe (ideale < 250)
- [ ] Test separati in `_tests.rs`
- [ ] Documentazione /// per funzioni pubbliche
- [ ] `cargo clippy -- -D warnings` passa
- [ ] `cargo test` passa al 100%
- [ ] Nomi descrittivi (no abbreviazioni criptiche)
- [ ] Moduli logici (una responsabilità per modulo)

### Esempio Documentazione

```rust
/// Rappresenta il Timer Control Register (TMxCNT_H)
///
/// Bit layout:
/// - 0-1: Prescaler (0=1, 1=64, 2=256, 3=1024 cycles)
/// - 2: Count-up timing (cascade mode)
/// - 6: IRQ enable
/// - 7: Timer enable
#[derive(Debug, Clone, Copy)]
pub struct TimerControl {
    pub prescaler: u8,
    pub count_up: bool,
    pub irq_enable: bool,
    pub enabled: bool,
}

impl TimerControl {
    /// Crea TimerControl da valore u16 register
    pub fn from_u16(value: u16) -> Self {
        Self {
            prescaler: (value & 0x3) as u8,
            count_up: (value & (1 << 2)) != 0,
            irq_enable: (value & (1 << 6)) != 0,
            enabled: (value & (1 << 7)) != 0,
        }
    }
}
```

## 🚀 Best Practices

### DO ✅

- Mantieni file piccoli e focalizzati
- Separa test dall'implementazione
- Usa `const` per magic numbers
- Documenta funzioni pubbliche
- Test dopo ogni modifica
- Clippy strict mode sempre

### DON'T ❌

- File > 300 righe senza motivo
- Test mischiati con implementazione
- Magic numbers nel codice
- Funzioni pubbliche non documentate
- Commit senza test
- Ignorare warning Clippy

## 📚 Riferimenti

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [GBATEK](http://problemkaputt.de/gbatek.htm) - Hardware reference
- [ARM7TDMI Manual](http://infocenter.arm.com/help/topic/com.arm.doc.ddi0210c/DDI0210B.pdf)

---

**Aggiornato:** 2 dicembre 2025  
**Versione:** v0.7.0 (DMA completo)
