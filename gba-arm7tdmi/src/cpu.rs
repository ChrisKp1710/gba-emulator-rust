use crate::registers::Registers;

//==============================================================================
// MEMORIA E BUS
//==============================================================================
// Il MemoryBus è il trait che definisce come la CPU accede alla memoria.
// Qualsiasi componente che implementa questo trait può essere usato dalla CPU
// per leggere/scrivere dati. Questo design modulare permette di:
// 1. Testare la CPU con un bus fittizio (DummyBus)
// 2. Usare un bus reale che gestisce tutta la memoria GBA
// 3. Cambiare implementazione senza modificare la CPU
//==============================================================================

/// Trait per accesso alla memoria dalla CPU
///
/// Questo trait definisce le operazioni base di lettura/scrittura che la CPU
/// deve poter fare sulla memoria. Implementalo per creare un bus personalizzato.
pub trait MemoryBus {
    fn read_byte(&mut self, addr: u32) -> u8;
    fn read_halfword(&mut self, addr: u32) -> u16;
    fn read_word(&mut self, addr: u32) -> u32;

    fn write_byte(&mut self, addr: u32, value: u8);
    fn write_halfword(&mut self, addr: u32, value: u16);
    fn write_word(&mut self, addr: u32, value: u32);
}

//==============================================================================
// CPU ARM7TDMI - STRUTTURA PRINCIPALE
//==============================================================================
// Questa è la CPU del Game Boy Advance. È un processore ARM7TDMI che:
// - Supporta set di istruzioni ARM a 32-bit
// - Supporta set di istruzioni THUMB a 16-bit (più compatto)
// - Ha 37 registri in totale (16 visibili + 21 banked)
// - Esegue istruzioni in pipeline a 3 stadi (Fetch-Decode-Execute)
//
// STATO CORRENTE:
// ✅ Struttura base implementata
// ✅ Registri e modalità CPU funzionanti
// 🚧 TODO: Implementare tutte le istruzioni ARM
// 🚧 TODO: Implementare tutte le istruzioni THUMB
// 🚧 TODO: Pipeline accurata
//==============================================================================

/// CPU ARM7TDMI del Game Boy Advance
///
/// Campi:
/// - `regs`: Registri della CPU (R0-R15, CPSR, SPSR, banked registers)
/// - `cycles`: Contatore cicli totali eseguiti
/// - `halted`: Se true, la CPU è in stato HALT (risparmio energetico)
pub struct ARM7TDMI {
    pub regs: Registers,
    pub cycles: u64,
    pub halted: bool,
}

impl ARM7TDMI {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            cycles: 0,
            halted: false,
        }
    }

    /// Reset della CPU
    pub fn reset(&mut self) {
        self.regs = Registers::new();
        self.regs.set_pc(0x0000_0000);
        self.cycles = 0;
        self.halted = false;
    }

    //==========================================================================
    // STEP - ESECUZIONE ISTRUZIONE
    //==========================================================================
    // Questo è il metodo principale che esegue UNA SINGOLA ISTRUZIONE.
    //
    // COME FUNZIONA:
    // 1. Controlla se la CPU è in HALT (se sì, salta e restituisce 1 ciclo)
    // 2. Legge il bit THUMB del CPSR per capire quale set istruzioni usare
    // 3. Esegue l'istruzione ARM (32-bit) o THUMB (16-bit)
    // 4. Restituisce il numero di cicli usati dall'istruzione
    //
    // IMPORTANTE: Ogni istruzione ha un costo in cicli diverso!
    // - Istruzioni semplici: 1 ciclo
    // - Accesso memoria: 1-3 cicli (dipende dalla regione)
    // - Moltiplicazioni: 1-4 cicli
    // - Branch: 2-3 cicli
    //==========================================================================

    /// Esegui una singola istruzione e restituisci i cicli usati
    ///
    /// # Arguments
    /// * `bus` - Il bus di memoria per leggere istruzioni e dati
    ///
    /// # Returns
    /// Numero di cicli CPU usati dall'istruzione
    pub fn step<M: MemoryBus>(&mut self, bus: &mut M) -> u32 {
        if self.halted {
            return 1;
        }

        let cycles = if self.regs.is_thumb() {
            self.execute_thumb(bus)
        } else {
            self.execute_arm(bus)
        };

        self.cycles += cycles as u64;
        cycles
    }

    //==========================================================================
    // ESECUZIONE ISTRUZIONI ARM (32-bit)
    //==========================================================================
    // Le istruzioni ARM sono a 32-bit e sono il set principale del processore.
    //
    // FORMATO ISTRUZIONE ARM:
    // [31:28] - Condition code (EQ, NE, CS, etc.)
    // [27:25] - Tipo istruzione
    // [24:0]  - Parametri specifici dell'istruzione
    //
    // PASSI PER IMPLEMENTARE:
    // 1. Leggere istruzione a 32-bit dal PC
    // 2. Verificare condition code (se non soddisfatto, skip)
    // 3. Decodificare il tipo di istruzione dai bit [27:25] e altri
    // 4. Eseguire l'operazione specifica
    // 5. Aggiornare PC (normalmente +4, o branch se è un salto)
    // 6. Restituire cicli usati
    //
    // TODO: Implementare decoder completo per tutte le istruzioni ARM
    // Riferimento: ARM7TDMI Technical Manual, GBATEK
    //==========================================================================

    /// Esegui un'istruzione ARM (32-bit)
    fn execute_arm<M: MemoryBus>(&mut self, bus: &mut M) -> u32 {
        let pc = self.regs.pc();
        let _instruction = bus.read_word(pc);
        self.regs.set_pc(pc.wrapping_add(4));

        // TODO: Decode ed esecuzione istruzioni ARM
        // PROSSIMO STEP: Implementare decoder in arm.rs
        // Vedere: gba-arm7tdmi/src/arm.rs per le condizioni
        1
    } //==========================================================================
      // ESECUZIONE ISTRUZIONI THUMB (16-bit)
      //==========================================================================
      // Le istruzioni THUMB sono a 16-bit, più compatte ma meno potenti.
      // Vengono usate per risparmiare spazio ROM e migliorare cache performance.
      //
      // VANTAGGI THUMB:
      // - Codice più compatto (circa 65% della dimensione ARM)
      // - Migliore uso della cache
      // - Usato dalla maggior parte dei giochi GBA
      //
      // FORMATO ISTRUZIONE THUMB:
      // [15:13] o [15:11] - Tipo istruzione (varia)
      // [12:0]  - Parametri specifici
      //
      // DIFFERENZE DA ARM:
      // - NO condition codes (esegue sempre, tranne branch condizionali)
      // - Accesso limitato ai registri (spesso solo R0-R7)
      // - Set istruzioni ridotto
      //
      // TODO: Implementare decoder completo per tutte le istruzioni THUMB
      // Riferimento: ARM7TDMI Manual Section 5, GBATEK
      //==========================================================================

    /// Esegui un'istruzione THUMB (16-bit)
    fn execute_thumb<M: MemoryBus>(&mut self, bus: &mut M) -> u32 {
        let pc = self.regs.pc();
        let _instruction = bus.read_halfword(pc);
        self.regs.set_pc(pc.wrapping_add(2));

        // TODO: Decode ed esecuzione istruzioni THUMB
        // PROSSIMO STEP: Implementare decoder in thumb.rs
        // Vedere: gba-arm7tdmi/src/thumb.rs per i formati
        1
    }
    /// Gestisci interrupt IRQ
    pub fn request_interrupt(&mut self) {
        if self.regs.cpsr & (1 << 7) == 0 {
            // IRQ non disabilitati
            self.handle_irq();
        }
    }

    fn handle_irq(&mut self) {
        use crate::registers::Mode;

        // Salva stato corrente
        let old_cpsr = self.regs.cpsr;
        let pc = self.regs.pc();

        // Passa a modalità IRQ
        self.regs.change_mode(Mode::IRQ);
        self.regs.set_spsr(old_cpsr);
        self.regs.set_lr(pc.wrapping_add(4));

        // Disabilita IRQ e passa ad ARM state
        self.regs.cpsr |= 1 << 7; // Disable IRQ
        self.regs.cpsr &= !(1 << 5); // ARM state

        // Salta al vettore IRQ
        self.regs.set_pc(0x0000_0018);
    }
}

impl Default for ARM7TDMI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    struct DummyBus;

    impl MemoryBus for DummyBus {
        fn read_byte(&mut self, _addr: u32) -> u8 {
            0
        }
        fn read_halfword(&mut self, _addr: u32) -> u16 {
            0
        }
        fn read_word(&mut self, _addr: u32) -> u32 {
            0
        }
        fn write_byte(&mut self, _addr: u32, _value: u8) {}
        fn write_halfword(&mut self, _addr: u32, _value: u16) {}
        fn write_word(&mut self, _addr: u32, _value: u32) {}
    }

    #[test]
    fn test_cpu_creation() {
        let cpu = ARM7TDMI::new();
        assert_eq!(cpu.cycles, 0);
        assert_eq!(cpu.halted, false);
    }

    #[test]
    fn test_cpu_reset() {
        let mut cpu = ARM7TDMI::new();
        cpu.cycles = 1000;
        cpu.reset();
        assert_eq!(cpu.cycles, 0);
        assert_eq!(cpu.regs.pc(), 0);
    }
}
