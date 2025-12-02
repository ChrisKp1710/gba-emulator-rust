# 🗺️ MAPPA DEL PROGETTO - Dove Trovare Ogni Cosa

## 📍 Navigazione Rapida

> **Vuoi modificare...? → Vai in questo file!**

| Cosa Vuoi Fare                 | File da Modificare               | Riga Circa |
| ------------------------------ | -------------------------------- | ---------- |
| 🔧 Aggiungere istruzione ARM   | `gba-arm7tdmi/src/arm.rs`        | 20-40      |
| 🔧 Aggiungere istruzione THUMB | `gba-arm7tdmi/src/thumb.rs`      | 5-25       |
| 🔧 Modificare registri CPU     | `gba-arm7tdmi/src/registers.rs`  | 50-150     |
| 🔧 Cambiare logica CPU         | `gba-arm7tdmi/src/cpu.rs`        | 30-100     |
| 💾 Modificare memoria/RAM      | `gba-core/src/memory.rs`         | 30-250     |
| 🎨 Modificare grafica/PPU      | `gba-core/src/ppu.rs`            | 10-60      |
| ⚡ Modificare interrupt        | `gba-core/src/interrupt.rs`      | 15-70      |
| 📂 Modificare caricamento ROM  | `gba-core/src/cartridge.rs`      | 30-80      |
| 🎮 Modificare interfaccia      | `gba-frontend-sdl2/src/ui.rs`    | 15-120     |
| ⌨️ Modificare input/controlli  | `gba-frontend-sdl2/src/input.rs` | 10-50      |

---

## 🏗️ Perché È Modulare? I Vantaggi Spiegati

### ❌ **SENZA** Modularità (Design Monolitico)

```
main.rs  (5000+ righe) 😱
│
├─ Tutto mescolato insieme:
│  ├─ CPU
│  ├─ Memoria
│  ├─ Grafica
│  ├─ Audio
│  ├─ Input
│  └─ UI
│
└─ PROBLEMI:
   ├─ Devi leggere tutto per capire una cosa
   ├─ Modifichi CPU → rompi grafica per errore
   ├─ Impossibile testare singole parti
   ├─ Difficile collaborare
   └─ Codice duplicato ovunque
```

### ✅ **CON** Modularità (Il Nostro Design)

```
📦 gba-arm7tdmi/        ← CPU isolata
📦 gba-core/            ← Logica emulatore
📦 gba-frontend-sdl2/   ← Interfaccia separata
│
└─ VANTAGGI:
   ✅ Ogni modulo fa UNA COSA sola
   ✅ Modifichi CPU → grafica non si tocca
   ✅ Test separati per ogni parte
   ✅ Facile capire dove andare
   ✅ Riutilizzabile in altri progetti
```

---

## 📦 I Tre Moduli Principali

### 1️⃣ gba-arm7tdmi - Il Cervello

```
gba-arm7tdmi/
│
├── Cargo.toml                    ← Dipendenze modulo CPU
│
└── src/
    ├── lib.rs                    ← Entry point, esporta CPU
    │   └─ "pub use cpu::ARM7TDMI;"
    │
    ├── cpu.rs                    ← ⭐ CORE CPU
    │   ├─ struct ARM7TDMI        ← La CPU vera e propria
    │   ├─ step()                 ← Esegue 1 istruzione
    │   ├─ execute_arm()          ← Esegue istruzione ARM
    │   ├─ execute_thumb()        ← Esegue istruzione THUMB
    │   └─ handle_irq()           ← Gestisce interrupt
    │
    ├── registers.rs              ← ⭐ REGISTRI
    │   ├─ struct Registers       ← R0-R15, CPSR, SPSR
    │   ├─ enum Mode              ← Modalità CPU
    │   ├─ change_mode()          ← Cambia modalità
    │   └─ set_flags()            ← Aggiorna flag N,Z,C,V
    │
    ├── arm.rs                    ← ⭐ ISTRUZIONI ARM
    │   ├─ enum Condition         ← Condizioni (EQ, NE, etc.)
    │   └─ TODO: decode_arm()     ← Da implementare!
    │
    ├── thumb.rs                  ← ⭐ ISTRUZIONI THUMB
    │   ├─ enum ThumbInstruction
    │   └─ TODO: decode_thumb()   ← Da implementare!
    │
    └── instructions/             ← Implementazioni dettagliate
        ├── mod.rs
        ├── alu.rs                ← ADD, SUB, AND, OR, etc.
        ├── branch.rs             ← B, BL, BX
        └── load_store.rs         ← LDR, STR, LDM, STM
```

**🎯 Quando lo modifichi?**

- Vuoi aggiungere supporto per nuove istruzioni
- Vuoi fixare bug nella CPU
- Vuoi ottimizzare performance istruzioni

**🔗 Dipende da:**

- Niente! È completamente indipendente

**🔗 Usato da:**

- `gba-core` (tramite trait `MemoryBus`)

---

### 2️⃣ gba-core - Il Sistema

```
gba-core/
│
├── Cargo.toml                    ← Dipende da gba-arm7tdmi
│
└── src/
    ├── lib.rs                    ← Entry point, esporta tutto
    │   └─ "pub use emulator::GbaEmulator;"
    │
    ├── emulator.rs               ← ⭐ ORCHESTRATORE
    │   ├─ struct GbaEmulator     ← Coordina tutto
    │   ├─ load_bios()            ← Carica BIOS
    │   ├─ load_cartridge()       ← Carica ROM
    │   ├─ reset()                ← Reset sistema
    │   └─ run_frame()            ← ⭐ Loop principale!
    │       ├─ Esegue ~280k cicli CPU
    │       ├─ Avanza PPU
    │       ├─ Gestisce interrupt
    │       └─ Produce 1 frame
    │
    ├── bus.rs                    ← ⭐ COLLEGA TUTTO
    │   ├─ struct Bus             ← Connette CPU ↔ Memoria
    │   ├─ impl MemoryBus         ← Implementa trait CPU
    │   ├─ read_byte/word()       ← CPU legge
    │   └─ write_byte/word()      ← CPU scrive
    │
    ├── memory.rs                 ← ⭐ MEMORIA COMPLETA
    │   ├─ struct Memory
    │   ├─ bios: Vec<u8>          ← BIOS (16 KB)
    │   ├─ ewram: Vec<u8>         ← RAM esterna (256 KB)
    │   ├─ iwram: Vec<u8>         ← RAM interna (32 KB)
    │   ├─ vram: Vec<u8>          ← Video RAM (96 KB)
    │   ├─ oam: Vec<u8>           ← Sprite (1 KB)
    │   ├─ rom: Vec<u8>           ← Gioco
    │   └─ sram: Vec<u8>          ← Salvataggi
    │
    ├── ppu.rs                    ← ⭐ GRAFICA
    │   ├─ struct PPU
    │   ├─ framebuffer            ← Buffer 240x160
    │   ├─ step()                 ← Avanza rendering
    │   ├─ render_scanline()      ← Disegna 1 riga
    │   └─ in_vblank()            ← Controlla VBlank
    │
    ├── interrupt.rs              ← ⭐ INTERRUPT
    │   ├─ struct InterruptController
    │   ├─ ie, if_, ime           ← Registri interrupt
    │   ├─ request()              ← Richiedi interrupt
    │   └─ pending()              ← C'è interrupt?
    │
    ├── cartridge.rs              ← ⭐ ROM LOADING
    │   ├─ struct Cartridge
    │   ├─ load()                 ← Leggi file .gba
    │   └─ parse_header()         ← Estrai info ROM
    │
    ├── apu.rs                    ← Audio (TODO)
    ├── timer.rs                  ← Timer (TODO)
    └── dma.rs                    ← DMA (TODO)
```

**🎯 Quando lo modifichi?**

- Vuoi aggiungere features hardware (audio, timer, DMA)
- Vuoi modificare come funziona la memoria
- Vuoi cambiare il rendering grafico

**🔗 Dipende da:**

- `gba-arm7tdmi` (usa la CPU)

**🔗 Usato da:**

- `gba-frontend-sdl2` (l'interfaccia)

---

### 3️⃣ gba-frontend-sdl2 - L'Interfaccia

```
gba-frontend-sdl2/
│
├── Cargo.toml                    ← Dipende da gba-core e SDL2
│
└── src/
    ├── main.rs                   ← ⭐ ENTRY POINT
    │   ├─ fn main()              ← Parte da qui!
    │   ├─ Parse argomenti        ← ROM path, BIOS, etc.
    │   ├─ Carica ROM             ← Leggi file
    │   ├─ Crea emulatore         ← GbaEmulator::new()
    │   └─ Avvia UI               ← ui::run()
    │
    ├── ui.rs                     ← ⭐ FINESTRA E RENDERING
    │   ├─ fn run()               ← Loop principale UI
    │   ├─ Crea finestra SDL2     ← 720x480 (240x160 x3)
    │   ├─ Loop eventi            ← Input, ESC per uscire
    │   ├─ emulator.run_frame()   ← Esegue 1 frame
    │   ├─ Copia framebuffer      ← PPU → SDL texture
    │   ├─ Renderizza             ← Mostra su schermo
    │   └─ Limita FPS             ← 60 FPS target
    │
    └── input.rs                  ← ⭐ CONTROLLI
        ├─ enum GbaButton         ← A, B, L, R, etc.
        ├─ struct InputMapper
        └─ map_key()              ← SDL key → GBA button
```

**🎯 Quando lo modifichi?**

- Vuoi cambiare l'interfaccia utente
- Vuoi aggiungere menu, opzioni
- Vuoi modificare controlli
- Vuoi cambiare risoluzione/scaling

**🔗 Dipende da:**

- `gba-core` (usa l'emulatore)
- `SDL2` (libreria grafica)

**🔗 Usato da:**

- Utente finale!

---

## 🔄 Come Comunicano i Moduli

```
┌─────────────────────────────────────────────────┐
│  gba-frontend-sdl2 (Interfaccia)                │
│  ┌─────────────────────────────────────────┐   │
│  │ main.rs                                  │   │
│  │  ├─ Carica ROM                          │   │
│  │  └─ Crea GbaEmulator                    │   │
│  └──────────────┬──────────────────────────┘   │
│                 │                                │
│  ┌──────────────▼──────────────────────────┐   │
│  │ ui.rs                                    │   │
│  │  └─ emulator.run_frame() ◄─ Chiama      │   │
│  └──────────────┬──────────────────────────┘   │
└─────────────────┼──────────────────────────────┘
                  │
                  │ Usa gba-core
                  │
┌─────────────────▼──────────────────────────────┐
│  gba-core (Sistema Emulatore)                  │
│  ┌─────────────────────────────────────────┐   │
│  │ emulator.rs                              │   │
│  │  └─ run_frame()                          │   │
│  │      ├─ cpu.step() ◄───────────┐        │   │
│  │      └─ ppu.step()              │        │   │
│  └──────────────┬──────────────────┼────────┘   │
│                 │                  │             │
│  ┌──────────────▼─────────┐  ┌────▼──────────┐ │
│  │ bus.rs                 │  │ ppu.rs        │ │
│  │  └─ read/write()       │  │  └─ render()  │ │
│  └──────────┬─────────────┘  └───────────────┘ │
│             │                                    │
│  ┌──────────▼─────────────┐                    │
│  │ memory.rs              │                    │
│  │  ├─ BIOS, WRAM, VRAM   │                    │
│  │  └─ ROM, SRAM          │                    │
│  └────────────────────────┘                    │
└─────────────────┬──────────────────────────────┘
                  │
                  │ Usa gba-arm7tdmi
                  │
┌─────────────────▼──────────────────────────────┐
│  gba-arm7tdmi (CPU)                            │
│  ┌─────────────────────────────────────────┐   │
│  │ cpu.rs                                   │   │
│  │  ├─ step()                              │   │
│  │  ├─ execute_arm()                       │   │
│  │  └─ execute_thumb()                     │   │
│  └──────────────┬──────────────────────────┘   │
│                 │                                │
│  ┌──────────────▼──────────────────────────┐   │
│  │ registers.rs                             │   │
│  │  └─ R0-R15, CPSR, SPSR                  │   │
│  └──────────────────────────────────────────┘   │
└──────────────────────────────────────────────────┘
```

### 🔗 Dipendenze (Dal Basso verso l'Alto)

```
gba-arm7tdmi         ← Livello 1: Nessuna dipendenza
      ▲
      │ dipende
      │
gba-core             ← Livello 2: Usa CPU
      ▲
      │ dipende
      │
gba-frontend-sdl2    ← Livello 3: Usa tutto
```

**Perché questo è importante?**

- ✅ **Puoi testare CPU da sola** senza bisogno del resto
- ✅ **Puoi cambiare UI** senza toccare CPU o memoria
- ✅ **Puoi riutilizzare CPU** in altri progetti
- ✅ **Modifiche isolate** - cambi un modulo, gli altri non si rompono

---

## 🎯 Esempi Pratici: "Voglio fare X, dove vado?"

### 📝 Scenario 1: "Voglio aggiungere istruzione ADD"

```
1. 📂 Vai in: gba-arm7tdmi/src/arm.rs
   └─ Aggiungi variante enum per ADD

2. 📂 Vai in: gba-arm7tdmi/src/cpu.rs
   └─ In execute_arm(), aggiungi decoder per ADD

3. 📂 Vai in: gba-arm7tdmi/src/instructions/alu.rs
   └─ Implementa logica ADD

4. ✅ Test:
   └─ cargo test --package gba-arm7tdmi
```

### 📝 Scenario 2: "Voglio cambiare risoluzione schermo"

```
1. 📂 Vai in: gba-frontend-sdl2/src/ui.rs
   └─ Cambia SCALE da 3 a 4 (più grande)

2. ✅ Compila:
   └─ cargo build --release

3. ✅ Nessun'altra modifica necessaria!
   └─ CPU e memoria non sanno niente dello schermo
```

### 📝 Scenario 3: "Voglio aggiungere save EEPROM"

```
1. 📂 Vai in: gba-core/src/cartridge.rs
   └─ Rileva tipo save da ROM

2. 📂 Vai in: gba-core/src/memory.rs
   └─ Implementa read/write EEPROM in regione 0x0D000000

3. 📂 Opzionale: gba-core/src/emulator.rs
   └─ Aggiungi save_to_file() / load_from_file()

4. ✅ Test con ROM che usa EEPROM
```

### 📝 Scenario 4: "Voglio aggiungere audio"

```
1. 📂 Vai in: gba-core/src/apu.rs
   └─ Implementa struct APU completa

2. 📂 Vai in: gba-core/src/bus.rs
   └─ Aggiungi APU e collega I/O registers audio

3. 📂 Vai in: gba-core/src/emulator.rs
   └─ In run_frame(), aggiungi apu.step()

4. 📂 Vai in: gba-frontend-sdl2/src/ui.rs
   └─ Aggiungi SDL2 audio output

5. ✅ Test con ROM che usa audio
```

---

## 🧪 Testing Modulare

Ogni modulo può essere testato **separatamente**:

```powershell
# Testa solo CPU
cargo test --package gba-arm7tdmi

# Testa solo core
cargo test --package gba-core

# Testa tutto
cargo test --workspace
```

**Esempio test CPU isolato:**

```rust
// In gba-arm7tdmi/src/cpu.rs

#[cfg(test)]
mod tests {
    use super::*;

    // Bus fittizio per test
    struct DummyBus;
    impl MemoryBus for DummyBus {
        fn read_byte(&mut self, _: u32) -> u8 { 0 }
        // ...
    }

    #[test]
    fn test_cpu_add() {
        let mut cpu = ARM7TDMI::new();
        let mut bus = DummyBus;

        // Test istruzione ADD
        cpu.regs.r[1] = 10;
        // Esegui ADD R0, R1, #5

        assert_eq!(cpu.regs.r[0], 15);
    }
}
```

---

## 📊 Vantaggi della Modularità - Tabella Comparativa

| Aspetto                   | Monolitico ❌                          | Modulare ✅ (Nostro)               |
| ------------------------- | -------------------------------------- | ---------------------------------- |
| **Capire il codice**      | Devi leggere tutto                     | Leggi solo modulo interessato      |
| **Tempo per trovare bug** | Ore                                    | Minuti (sai dove cercare)          |
| **Rischio di rompere**    | Alto (modifica una cosa → rompe altre) | Basso (moduli isolati)             |
| **Testing**               | Difficile (tutto insieme)              | Facile (ogni modulo separato)      |
| **Riutilizzo**            | Impossibile                            | Facile (es: CPU in altro progetto) |
| **Collaborazione**        | Conflitti continui                     | Ognuno su modulo diverso           |
| **Manutenzione**          | Nightmare 😱                           | Gestibile 😊                       |
| **Performance**           | Non ottimizzabile                      | Compili solo ciò che serve         |

---

## 🚀 Come Iniziare a Modificare

### Step 1: Identifica Cosa Vuoi Fare

```
Esempio: "Voglio far funzionare Pokémon"
│
├─ Cosa serve?
│  ├─ CPU che esegue istruzioni ✓
│  ├─ Grafica per vedere ✗ (da fare)
│  ├─ Input per controllare ✗ (da fare)
│  └─ Salvataggi ✗ (da fare)
│
└─ Priorità:
   1. Completa CPU (ARM + THUMB)
   2. Implementa rendering (PPU)
   3. Aggiungi input
   4. Implementa save SRAM
```

### Step 2: Vai nel Modulo Giusto

Usa la tabella all'inizio di questo file!

### Step 3: Leggi i Commenti

Ogni file ha spiegazioni su:

- Cosa fa
- Come funziona
- Cosa manca (TODO)
- Come procedere

### Step 4: Modifica e Testa

```powershell
# 1. Fai modifica
# 2. Compila
cargo check

# 3. Testa modulo specifico
cargo test --package nome-modulo

# 4. Compila tutto
cargo build --release

# 5. Prova
.\target\release\gba-emulator.exe test.gba
```

---

## 🎓 Regole d'Oro dell'Architettura Modulare

1. **Un Modulo = Una Responsabilità**

   - CPU fa solo CPU
   - Memoria fa solo memoria
   - UI fa solo UI

2. **Comunicazione tramite Interfacce**

   - CPU non sa cos'è SDL2
   - UI non sa come funziona CPU
   - Usano trait/interfacce per parlare

3. **Dipendenze Unidirezionali**

   - Frontend → Core → CPU
   - MAI CPU → Frontend!

4. **Test Isolati**

   - Ogni modulo ha i suoi test
   - Non serve tutto il sistema per testare

5. **Documentazione in Modulo**
   - Commenti dove serve
   - README per overview
   - Guida per architettura

---

## 📞 Cheat Sheet Rapido

```
╔════════════════════════════════════════════════════════════╗
║  VOGLIO...                    │  VADO IN...               ║
╠════════════════════════════════════════════════════════════╣
║  Modificare CPU               │  gba-arm7tdmi/src/        ║
║  Aggiungere istruzione        │  gba-arm7tdmi/src/arm.rs  ║
║  Cambiare memoria             │  gba-core/src/memory.rs   ║
║  Modificare grafica           │  gba-core/src/ppu.rs      ║
║  Aggiungere audio             │  gba-core/src/apu.rs      ║
║  Cambiare UI                  │  gba-frontend-sdl2/       ║
║  Modificare controlli         │  gba-frontend-sdl2/input  ║
║  Caricare ROM diverso         │  gba-core/src/cartridge   ║
║  Gestire interrupt            │  gba-core/src/interrupt   ║
║  Ottimizzare performance      │  Profila prima, poi il    ║
║                               │  modulo specifico         ║
╚════════════════════════════════════════════════════════════╝
```

---

## ✅ Conclusione

L'architettura modulare significa:

🎯 **Sai sempre dove andare** - Non ti perdi in migliaia di righe
🔧 **Modifiche sicure** - Cambi un pezzo, il resto funziona
🧪 **Test facili** - Provi ogni parte separatamente
👥 **Collaborazione semplice** - Ognuno lavora su modulo diverso
♻️ **Codice riutilizzabile** - CPU in altri progetti
📚 **Facile da capire** - Leggi solo ciò che serve

**Ricorda**: Quando modifichi qualcosa, pensa sempre "In che modulo va questa cosa?" e mettila lì. Non mescolare responsabilità!
