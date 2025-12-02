use bitflags::bitflags;
use serde::{Deserialize, Serialize};

//==============================================================================
// REGISTRI CPU ARM7TDMI
//==============================================================================
// La CPU ARM7TDMI ha un sistema complesso di registri con "banking".
// Questo significa che alcuni registri cambiano quando si cambia modalità CPU.
//
// REGISTRI VISIBILI (dipendono dalla modalità):
// R0-R12  : Registri generali (alcuni sono banked in FIQ)
// R13 (SP): Stack Pointer (diverso per ogni modalità)
// R14 (LR): Link Register (dove tornare da funzioni/interrupt)
// R15 (PC): Program Counter (indirizzo istruzione corrente)
// CPSR    : Current Program Status Register (flags e modalità)
// SPSR    : Saved Program Status Register (solo modalità privilegiate)
//
// BANKING:
// Quando cambi modalità (es. da User a IRQ), alcuni registri vengono
// "salvati" e sostituiti con versioni specifiche della modalità.
// Questo permette a interrupt/eccezioni di avere il proprio stack senza
// corrompere i dati del programma principale.
//==============================================================================

/// Modalità operative della CPU ARM7TDMI
///
/// Ogni modalità ha privilegi e scopi diversi:
/// - User: Modalità normale applicazioni
/// - FIQ: Fast Interrupt (interrupt ad alta priorità)
/// - IRQ: Interrupt normale
/// - Supervisor: Sistema operativo/BIOS
/// - Abort: Gestione errori memoria
/// - Undefined: Gestione istruzioni invalide
/// - System: Come User ma con privilegi (GBA non lo usa molto)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    User = 0b10000,
    FIQ = 0b10001,
    IRQ = 0b10010,
    Supervisor = 0b10011,
    Abort = 0b10111,
    Undefined = 0b11011,
    System = 0b11111,
}

impl Mode {
    pub fn from_bits(bits: u32) -> Option<Self> {
        match bits & 0x1F {
            0b10000 => Some(Mode::User),
            0b10001 => Some(Mode::FIQ),
            0b10010 => Some(Mode::IRQ),
            0b10011 => Some(Mode::Supervisor),
            0b10111 => Some(Mode::Abort),
            0b11011 => Some(Mode::Undefined),
            0b11111 => Some(Mode::System),
            _ => None,
        }
    }
}

/// Stato della CPU (ARM o THUMB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CpuState {
    ARM,
    THUMB,
}

bitflags! {
    /// Program Status Register (PSR) flags
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct StatusFlags: u32 {
        const NEGATIVE   = 1 << 31; // N flag
        const ZERO       = 1 << 30; // Z flag
        const CARRY      = 1 << 29; // C flag
        const OVERFLOW   = 1 << 28; // V flag
        const IRQ_DISABLE = 1 << 7; // I flag
        const FIQ_DISABLE = 1 << 6; // F flag
        const THUMB_STATE = 1 << 5; // T flag
    }
}

/// Set di registri ARM7TDMI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registers {
    // Registri generali R0-R15
    pub r: [u32; 16],

    // Registri banked per diverse modalità
    pub r8_fiq: u32,
    pub r9_fiq: u32,
    pub r10_fiq: u32,
    pub r11_fiq: u32,
    pub r12_fiq: u32,
    pub r13_fiq: u32, // SP_fiq
    pub r14_fiq: u32, // LR_fiq

    pub r13_svc: u32, // SP_svc
    pub r14_svc: u32, // LR_svc

    pub r13_abt: u32, // SP_abt
    pub r14_abt: u32, // LR_abt

    pub r13_irq: u32, // SP_irq
    pub r14_irq: u32, // LR_irq

    pub r13_und: u32, // SP_und
    pub r14_und: u32, // LR_und

    // Current Program Status Register
    pub cpsr: u32,

    // Saved Program Status Registers
    pub spsr_fiq: u32,
    pub spsr_svc: u32,
    pub spsr_abt: u32,
    pub spsr_irq: u32,
    pub spsr_und: u32,

    // Modalità corrente
    pub mode: Mode,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            r: [0; 16],
            r8_fiq: 0,
            r9_fiq: 0,
            r10_fiq: 0,
            r11_fiq: 0,
            r12_fiq: 0,
            r13_fiq: 0,
            r14_fiq: 0,
            r13_svc: 0,
            r14_svc: 0,
            r13_abt: 0,
            r14_abt: 0,
            r13_irq: 0,
            r14_irq: 0,
            r13_und: 0,
            r14_und: 0,
            cpsr: Mode::System as u32,
            spsr_fiq: 0,
            spsr_svc: 0,
            spsr_abt: 0,
            spsr_irq: 0,
            spsr_und: 0,
            mode: Mode::System,
        }
    }

    /// Program Counter (R15)
    #[inline(always)]
    pub fn pc(&self) -> u32 {
        self.r[15]
    }

    /// Set Program Counter
    #[inline(always)]
    pub fn set_pc(&mut self, value: u32) {
        self.r[15] = value;
    }

    /// Stack Pointer (R13)
    #[inline(always)]
    pub fn sp(&self) -> u32 {
        self.r[13]
    }

    /// Link Register (R14)
    #[inline(always)]
    pub fn lr(&self) -> u32 {
        self.r[14]
    }

    /// Set Link Register
    #[inline(always)]
    pub fn set_lr(&mut self, value: u32) {
        self.r[14] = value;
    }

    /// Verifica se siamo in stato THUMB
    #[inline(always)]
    pub fn is_thumb(&self) -> bool {
        self.cpsr & StatusFlags::THUMB_STATE.bits() != 0
    }

    /// Imposta stato THUMB
    #[inline(always)]
    pub fn set_thumb(&mut self, thumb: bool) {
        if thumb {
            self.cpsr |= StatusFlags::THUMB_STATE.bits();
        } else {
            self.cpsr &= !StatusFlags::THUMB_STATE.bits();
        }
    }

    /// Ottieni lo stato corrente della CPU
    #[inline(always)]
    pub fn cpu_state(&self) -> CpuState {
        if self.is_thumb() {
            CpuState::THUMB
        } else {
            CpuState::ARM
        }
    }

    /// Verifica flag Negative
    #[inline(always)]
    pub fn flag_n(&self) -> bool {
        self.cpsr & StatusFlags::NEGATIVE.bits() != 0
    }

    /// Imposta flag Negative
    #[inline(always)]
    pub fn set_flag_n(&mut self, value: bool) {
        if value {
            self.cpsr |= StatusFlags::NEGATIVE.bits();
        } else {
            self.cpsr &= !StatusFlags::NEGATIVE.bits();
        }
    }

    /// Verifica flag Zero
    #[inline(always)]
    pub fn flag_z(&self) -> bool {
        self.cpsr & StatusFlags::ZERO.bits() != 0
    }

    /// Imposta flag Zero
    #[inline(always)]
    pub fn set_flag_z(&mut self, value: bool) {
        if value {
            self.cpsr |= StatusFlags::ZERO.bits();
        } else {
            self.cpsr &= !StatusFlags::ZERO.bits();
        }
    }

    /// Verifica flag Carry
    #[inline(always)]
    pub fn flag_c(&self) -> bool {
        self.cpsr & StatusFlags::CARRY.bits() != 0
    }

    /// Imposta flag Carry
    #[inline(always)]
    pub fn set_flag_c(&mut self, value: bool) {
        if value {
            self.cpsr |= StatusFlags::CARRY.bits();
        } else {
            self.cpsr &= !StatusFlags::CARRY.bits();
        }
    }

    /// Verifica flag Overflow
    #[inline(always)]
    pub fn flag_v(&self) -> bool {
        self.cpsr & StatusFlags::OVERFLOW.bits() != 0
    }

    /// Imposta flag Overflow
    #[inline(always)]
    pub fn set_flag_v(&mut self, value: bool) {
        if value {
            self.cpsr |= StatusFlags::OVERFLOW.bits();
        } else {
            self.cpsr &= !StatusFlags::OVERFLOW.bits();
        }
    }

    /// Set flag NZCV
    #[inline(always)]
    pub fn set_flags(&mut self, n: bool, z: bool, c: bool, v: bool) {
        let mut flags = self.cpsr & 0x0FFFFFFF;
        if n {
            flags |= StatusFlags::NEGATIVE.bits();
        }
        if z {
            flags |= StatusFlags::ZERO.bits();
        }
        if c {
            flags |= StatusFlags::CARRY.bits();
        }
        if v {
            flags |= StatusFlags::OVERFLOW.bits();
        }
        self.cpsr = flags;
    }

    /// Cambia modalità CPU
    pub fn change_mode(&mut self, new_mode: Mode) {
        if self.mode == new_mode {
            return;
        }

        // Salva registri banked correnti
        match self.mode {
            Mode::FIQ => {
                self.r8_fiq = self.r[8];
                self.r9_fiq = self.r[9];
                self.r10_fiq = self.r[10];
                self.r11_fiq = self.r[11];
                self.r12_fiq = self.r[12];
                self.r13_fiq = self.r[13];
                self.r14_fiq = self.r[14];
            }
            Mode::Supervisor => {
                self.r13_svc = self.r[13];
                self.r14_svc = self.r[14];
            }
            Mode::Abort => {
                self.r13_abt = self.r[13];
                self.r14_abt = self.r[14];
            }
            Mode::IRQ => {
                self.r13_irq = self.r[13];
                self.r14_irq = self.r[14];
            }
            Mode::Undefined => {
                self.r13_und = self.r[13];
                self.r14_und = self.r[14];
            }
            _ => {}
        }

        // Carica registri banked nuovi
        match new_mode {
            Mode::FIQ => {
                self.r[8] = self.r8_fiq;
                self.r[9] = self.r9_fiq;
                self.r[10] = self.r10_fiq;
                self.r[11] = self.r11_fiq;
                self.r[12] = self.r12_fiq;
                self.r[13] = self.r13_fiq;
                self.r[14] = self.r14_fiq;
            }
            Mode::Supervisor => {
                self.r[13] = self.r13_svc;
                self.r[14] = self.r14_svc;
            }
            Mode::Abort => {
                self.r[13] = self.r13_abt;
                self.r[14] = self.r14_abt;
            }
            Mode::IRQ => {
                self.r[13] = self.r13_irq;
                self.r[14] = self.r14_irq;
            }
            Mode::Undefined => {
                self.r[13] = self.r13_und;
                self.r[14] = self.r14_und;
            }
            _ => {}
        }

        self.mode = new_mode;
        self.cpsr = (self.cpsr & 0xFFFFFFE0) | (new_mode as u32);
    }

    /// Ottieni SPSR corrente
    pub fn spsr(&self) -> u32 {
        match self.mode {
            Mode::FIQ => self.spsr_fiq,
            Mode::Supervisor => self.spsr_svc,
            Mode::Abort => self.spsr_abt,
            Mode::IRQ => self.spsr_irq,
            Mode::Undefined => self.spsr_und,
            _ => self.cpsr,
        }
    }

    /// Set SPSR corrente
    pub fn set_spsr(&mut self, value: u32) {
        match self.mode {
            Mode::FIQ => self.spsr_fiq = value,
            Mode::Supervisor => self.spsr_svc = value,
            Mode::Abort => self.spsr_abt = value,
            Mode::IRQ => self.spsr_irq = value,
            Mode::Undefined => self.spsr_und = value,
            _ => {}
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}
