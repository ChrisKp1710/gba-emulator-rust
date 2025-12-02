# 📘 GUIDA ARCHITETTURA E SVILUPPO

## Come È Strutturato il Progetto

Questo emulatore GBA è progettato con **architettura modulare**. Ogni componente è separato e indipendente, così puoi:

- ✅ Capire una parte senza dover conoscere tutto
- ✅ Modificare un modulo senza rompere gli altri
- ✅ Testare ogni componente singolarmente
- ✅ Aggiungere features gradualmente

---

## 🗂️ Struttura del Progetto

```
gba-emulator-rust/
│
├── 📦 gba-arm7tdmi/          ← CPU del Game Boy Advance
│   ├── src/
│   │   ├── cpu.rs            ← Logica principale CPU
│   │   ├── registers.rs      ← Registri e modalità
│   │   ├── arm.rs            ← Istruzioni ARM (32-bit)
│   │   ├── thumb.rs          ← Istruzioni THUMB (16-bit)
│   │   └── instructions/     ← Implementazioni istruzioni
│   └── Cargo.toml
│
├── 📦 gba-core/              ← Core dell'emulatore
│   ├── src/
│   │   ├── emulator.rs       ← Orchestratore principale
│   │   ├── bus.rs            ← Collega CPU e memoria
│   │   ├── memory.rs         ← Gestione memoria
│   │   ├── ppu.rs            ← Grafica (rendering)
│   │   ├── interrupt.rs      ← Sistema interrupt
│   │   └── cartridge.rs      ← Caricamento ROM
│   └── Cargo.toml
│
└── 📦 gba-frontend-sdl2/     ← Interfaccia utente
    ├── src/
    │   ├── main.rs           ← Entry point
    │   ├── ui.rs             ← Finestra e rendering
    │   └── input.rs          ← Gestione input
    └── Cargo.toml
```

---

## 🔄 Come Funziona l'Emulatore

### Flusso di Esecuzione

```
1. CARICAMENTO
   ├─ Utente lancia: gba-emulator.exe pokemon.gba
   ├─ main.rs legge la ROM
   ├─ Cartridge parser estrae header (titolo, game code, etc.)
   └─ ROM viene caricata in memoria

2. INIZIALIZZAZIONE
   ├─ Crea CPU (ARM7TDMI)
   ├─ Crea Bus (collega tutto)
   ├─ Inizializza memoria (BIOS, WRAM, VRAM, etc.)
   ├─ Inizializza PPU (grafica)
   └─ Reset CPU → PC = 0x08000000 (inizio ROM)

3. LOOP PRINCIPALE (run_frame)
   │
   ├─ Per ogni frame (60 FPS):
   │   │
   │   ├─ Esegui ~280,896 cicli CPU
   │   │   ├─ cpu.step() → esegui 1 istruzione
   │   │   ├─ Leggi istruzione da memoria
   │   │   ├─ Decodifica (ARM o THUMB?)
   │   │   ├─ Esegui operazione
   │   │   └─ Aggiorna PC
   │   │
   │   ├─ Ogni ciclo CPU:
   │   │   ├─ ppu.step(cycles) → avanza rendering
   │   │   └─ Controlla se scanline completa
   │   │
   │   ├─ Ogni scanline (160 totali):
   │   │   ├─ Renderizza linea grafica
   │   │   └─ Se scanline = 160 → VBlank interrupt
   │   │
   │   └─ Fine frame:
   │       ├─ Framebuffer pronto
   │       └─ Mostra su schermo
   │
   └─ Ripeti per frame successivo
```

---

## 🧩 Componenti Principali

### 1. CPU (gba-arm7tdmi/src/cpu.rs)

**Cosa fa**: Esegue le istruzioni del gioco

**Come funziona**:

```rust
// Ogni istruzione passa per questi step:
1. Fetch  → Leggi istruzione dalla memoria (PC)
2. Decode → Capisce che tipo di istruzione è
3. Execute→ Esegue l'operazione
4. Update → Aggiorna registri e PC
```

**Dove siamo**:

- ✅ Struttura base fatta
- ✅ Registri funzionanti
- 🚧 TODO: Implementare tutte le istruzioni ARM
- 🚧 TODO: Implementare tutte le istruzioni THUMB

**Come continuare**:

1. Apri `gba-arm7tdmi/src/arm.rs`
2. Implementa decoder per istruzioni ARM
3. Per ogni tipo istruzione, crea una funzione
4. Testa con test ROM

---

### 2. Memoria (gba-core/src/memory.rs)

**Cosa fa**: Gestisce accesso a tutta la memoria del GBA

**Mappa memoria**:

```
0x00000000  ┌─────────────┐
            │ BIOS (16KB) │ ← Sistema, solo lettura
0x02000000  ├─────────────┤
            │ EWRAM(256KB)│ ← RAM esterna
0x03000000  ├─────────────┤
            │ IWRAM (32KB)│ ← RAM interna (veloce!)
0x04000000  ├─────────────┤
            │ I/O (1KB)   │ ← Registri hardware
0x05000000  ├─────────────┤
            │ Palette(1KB)│ ← Colori
0x06000000  ├─────────────┤
            │ VRAM (96KB) │ ← Grafica
0x07000000  ├─────────────┤
            │ OAM (1KB)   │ ← Sprite
0x08000000  ├─────────────┤
            │ ROM (32MB)  │ ← Gioco
0x0E000000  ├─────────────┤
            │ SRAM (64KB) │ ← Salvataggi
            └─────────────┘
```

**Dove siamo**:

- ✅ Tutte le regioni implementate
- ✅ Read/Write funzionanti
- 🚧 TODO: Timing accurato (wait states)

---

### 3. PPU (gba-core/src/ppu.rs)

**Cosa fa**: Renderizza la grafica

**Come funziona**:

```
Frame (1/60 sec)
│
├─ Scanline 0   ┐
├─ Scanline 1   │
├─ Scanline 2   │ 160 linee visibili
├─ ...          │ (rendering attivo)
├─ Scanline 159 ┘
│
├─ VBlank start ← interrupt! Gioco aggiorna grafica
├─ Scanline 160 ┐
├─ Scanline 161 │ 68 linee VBlank
├─ ...          │ (schermo nero)
├─ Scanline 227 ┘
│
└─ Ripeti nuovo frame
```

**Dove siamo**:

- ✅ Timing base (scanline counter)
- ✅ VBlank detection
- 🚧 TODO: Background rendering
- 🚧 TODO: Sprite rendering
- 🚧 TODO: Modalità grafiche

**Come continuare**:

1. Implementa rendering Mode 3 (più semplice, bitmap diretto)
2. Poi Mode 0 (tile-based, usato dai Pokémon)
3. Aggiungi sprite (OAM)

---

### 4. Interrupt (gba-core/src/interrupt.rs)

**Cosa fa**: Gestisce eventi hardware

**Tipi di interrupt**:

- **VBlank**: Fine frame (60 volte/sec) → Gioco aggiorna logica
- **HBlank**: Fine scanline → Effetti per-linea
- **Timer**: Timer scaduti
- **DMA**: Trasferimento memoria completato
- **Input**: Tasto premuto

**Come funziona**:

```rust
1. Hardware segnala evento → request(VBLANK)
2. IF flag viene settato
3. Se IE abilitato && IME = true → interrupt!
4. CPU salta a handler → 0x18 (IRQ)
5. Handler gestisce evento
6. Return from interrupt
```

---

## 🎯 Come Procedere - Step by Step

### FASE 1: CPU Funzionante

**Obiettivo**: Far eseguire istruzioni base

```
Step 1.1: Implementa istruzioni ALU (ADD, SUB, MOV, etc.)
  ├─ File: gba-arm7tdmi/src/instructions/alu.rs
  ├─ Test: Crea test unitari per ogni istruzione
  └─ Riferimento: ARM7TDMI Manual, GBATEK

Step 1.2: Implementa istruzioni Branch (B, BL, BX)
  ├─ File: gba-arm7tdmi/src/instructions/branch.rs
  └─ Importante per salti e chiamate funzioni

Step 1.3: Implementa Load/Store (LDR, STR, etc.)
  ├─ File: gba-arm7tdmi/src/instructions/load_store.rs
  └─ Accesso memoria

Step 1.4: Implementa istruzioni THUMB
  ├─ File: gba-arm7tdmi/src/thumb.rs
  └─ Set ridotto, più semplice di ARM

Step 1.5: Test con ARM test suite
  └─ Verifica che CPU funzioni correttamente
```

### FASE 2: Grafica Base

**Obiettivo**: Vedere qualcosa sullo schermo

```
Step 2.1: Implementa Mode 3 (bitmap 240x160)
  ├─ File: gba-core/src/ppu.rs
  ├─ Più semplice: VRAM → diretto a schermo
  └─ Test: ROM che disegna pixel

Step 2.2: Implementa Mode 0 (tile-based)
  ├─ Background layers
  ├─ Tile mapping
  └─ Usato dalla maggior parte dei giochi

Step 2.3: Implementa sprite (OAM)
  ├─ Lettura OAM
  ├─ Rendering sprite
  └─ Priorità e trasparenza
```

### FASE 3: Input e I/O

**Obiettivo**: Controllare il gioco

```
Step 3.1: Implementa lettura input
  ├─ File: gba-core/src/bus.rs (I/O registers)
  ├─ Registro 0x04000130 (KEYINPUT)
  └─ Mappa tasti SDL → GBA buttons

Step 3.2: Implementa timer
  ├─ 4 timer hardware
  └─ Usati per timing e audio

Step 3.3: Implementa DMA
  └─ Trasferimenti memoria veloci
```

### FASE 4: Audio

**Obiettivo**: Sentire musica

```
Step 4.1: Implementa 4 canali GB compatibili
Step 4.2: Implementa DMA sound
Step 4.3: Mixing audio
```

### FASE 5: Salvataggi

**Obiettivo**: Salvare progresso

```
Step 5.1: Rileva tipo save (SRAM/Flash/EEPROM)
Step 5.2: Implementa scrittura
Step 5.3: Salva su file
Step 5.4: Save states (snapshot completo)
```

---

## 🔧 Come Modificare/Aggiungere Features

### Esempio: Aggiungere una Nuova Istruzione ARM

```rust
// 1. In gba-arm7tdmi/src/arm.rs, aggiungi il tipo
pub enum ArmInstruction {
    // ... esistenti
    Add { rd: u8, rn: u8, operand2: u32 },  // ← nuova!
}

// 2. In gba-arm7tdmi/src/cpu.rs, aggiungi il decoder
fn decode_arm(instruction: u32) -> ArmInstruction {
    let opcode = (instruction >> 21) & 0xF;
    match opcode {
        0b0100 => {  // ADD opcode
            let rd = ((instruction >> 12) & 0xF) as u8;
            let rn = ((instruction >> 16) & 0xF) as u8;
            let operand2 = instruction & 0xFFF;
            ArmInstruction::Add { rd, rn, operand2 }
        }
        // ... altri
    }
}

// 3. Implementa l'esecuzione
fn execute_arm_instruction(&mut self, instr: ArmInstruction) {
    match instr {
        ArmInstruction::Add { rd, rn, operand2 } => {
            let result = self.regs.r[rn] + operand2;
            self.regs.r[rd] = result;
            // Aggiorna flags se necessario
        }
        // ... altri
    }
}

// 4. Scrivi test
#[test]
fn test_add() {
    let mut cpu = ARM7TDMI::new();
    cpu.regs.r[1] = 10;
    // Esegui ADD R0, R1, #5
    // Verifica R0 = 15
}
```

---

## 📚 Risorse per Ogni Componente

### CPU

- **ARM7TDMI Manual**: http://infocenter.arm.com/help/topic/com.arm.doc.ddi0210c/DDI0210B.pdf
- **GBATEK - CPU**: http://problemkaputt.de/gbatek.htm#armcpureference

### Memoria

- **GBATEK - Memory**: http://problemkaputt.de/gbatek.htm#gbamemorymap

### Grafica

- **TONC - Video**: https://www.coranac.com/tonc/text/video.htm
- **GBATEK - Video**: http://problemkaputt.de/gbatek.htm#lcdvideocontroller

### Audio

- **GBATEK - Sound**: http://problemkaputt.de/gbatek.htm#gbasoundcontroller

---

## 🐛 Debug e Testing

### Come Testare una Modifica

```powershell
# 1. Fai la modifica nel file appropriato

# 2. Compila e verifica errori
cargo check

# 3. Esegui test unitari
cargo test

# 4. Compila release
cargo build --release

# 5. Testa con ROM
.\target\release\gba-emulator.exe test.gba

# 6. Se non funziona, abilita logging
$env:RUST_LOG = "debug"
.\target\release\gba-emulator.exe test.gba
```

### Dove Aggiungere Logging

```rust
// All'inizio del file
use log::{debug, info, warn, error};

// Durante esecuzione
debug!("Executing instruction: {:08X} at PC: {:08X}", instruction, pc);
info!("Frame rendered in {} ms", elapsed);
warn!("Unknown I/O register access: {:08X}", addr);
error!("Invalid opcode: {:08X}", opcode);
```

---

## ✅ Checklist Implementazione Feature

Quando implementi qualcosa di nuovo:

- [ ] **Commenti**: Aggiungi commenti che spiegano il "perché"
- [ ] **Test**: Scrivi almeno un test unitario
- [ ] **Logging**: Aggiungi log per debug
- [ ] **Documentazione**: Aggiorna questo file se è una feature importante
- [ ] **Verifica**: Testa con ROM reale
- [ ] **Performance**: Controlla che non rallenti troppo

---

## 🎓 Domande Frequenti

**Q: Da dove inizio se voglio aggiungere una feature?**
A: Segui la roadmap in DEVELOPMENT.md. Inizia dalle istruzioni CPU.

**Q: Come faccio a capire se l'emulatore sta funzionando?**
A: Usa ROM di test come Tonc demos o AGS Aging Cartridge.

**Q: Cosa fare se qualcosa non compila?**
A: Leggi l'errore del compilatore Rust - è molto chiaro! Se non capisci, cerca l'errore online.

**Q: Come aggiungo un nuovo tipo di salvataggio?**
A: In cartridge.rs, rileva il tipo dal game code e implementa la logica in memory.rs.

**Q: L'emulatore è lento, come ottimizzare?**
A: 1) Compila sempre con `--release`, 2) Profila con `cargo flamegraph`, 3) Ottimizza hotspot.

---

## 📞 Dove Chiedere Aiuto

1. **Codice CPU**: Leggi ARM7TDMI Manual
2. **Grafica**: Leggi TONC (molto chiaro!)
3. **Memoria/I/O**: GBATEK (completo ma denso)
4. **Rust**: https://doc.rust-lang.org/book/
5. **Emulation**: Guarda rustboyadvance-ng source code

---

**Ricorda**: L'emulazione è complessa! Vai step-by-step, testa spesso, e celebra ogni piccolo successo! 🎉
