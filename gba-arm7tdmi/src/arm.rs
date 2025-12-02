// Module per le istruzioni ARM
// Sarà implementato in dettaglio successivamente

/// Condizioni per l'esecuzione delle istruzioni ARM
#[derive(Debug, Clone, Copy)]
pub enum Condition {
    EQ = 0b0000, // Equal
    NE = 0b0001, // Not Equal
    CS = 0b0010, // Carry Set
    CC = 0b0011, // Carry Clear
    MI = 0b0100, // Minus
    PL = 0b0101, // Plus
    VS = 0b0110, // Overflow Set
    VC = 0b0111, // Overflow Clear
    HI = 0b1000, // Unsigned Higher
    LS = 0b1001, // Unsigned Lower or Same
    GE = 0b1010, // Signed Greater or Equal
    LT = 0b1011, // Signed Less Than
    GT = 0b1100, // Signed Greater Than
    LE = 0b1101, // Signed Less or Equal
    AL = 0b1110, // Always
}

impl Condition {
    pub fn from_opcode(opcode: u32) -> Self {
        match (opcode >> 28) & 0xF {
            0b0000 => Condition::EQ,
            0b0001 => Condition::NE,
            0b0010 => Condition::CS,
            0b0011 => Condition::CC,
            0b0100 => Condition::MI,
            0b0101 => Condition::PL,
            0b0110 => Condition::VS,
            0b0111 => Condition::VC,
            0b1000 => Condition::HI,
            0b1001 => Condition::LS,
            0b1010 => Condition::GE,
            0b1011 => Condition::LT,
            0b1100 => Condition::GT,
            0b1101 => Condition::LE,
            _ => Condition::AL,
        }
    }

    /// Verifica se la condizione è soddisfatta dato il CPSR corrente
    pub fn check(&self, cpsr: u32) -> bool {
        let n = (cpsr >> 31) & 1 == 1; // Negative
        let z = (cpsr >> 30) & 1 == 1; // Zero
        let c = (cpsr >> 29) & 1 == 1; // Carry
        let v = (cpsr >> 28) & 1 == 1; // Overflow

        match self {
            Condition::EQ => z,              // Z set
            Condition::NE => !z,             // Z clear
            Condition::CS => c,              // C set
            Condition::CC => !c,             // C clear
            Condition::MI => n,              // N set
            Condition::PL => !n,             // N clear
            Condition::VS => v,              // V set
            Condition::VC => !v,             // V clear
            Condition::HI => c && !z,        // C set AND Z clear
            Condition::LS => !c || z,        // C clear OR Z set
            Condition::GE => n == v,         // N == V
            Condition::LT => n != v,         // N != V
            Condition::GT => !z && (n == v), // Z clear AND (N == V)
            Condition::LE => z || (n != v),  // Z set OR (N != V)
            Condition::AL => true,           // Always
        }
    }
}

/// Tipi di istruzioni ARM
#[derive(Debug, Clone, Copy)]
pub enum ArmInstruction {
    /// Data Processing (ALU operations)
    DataProcessing {
        opcode: u8,      // Bits 21-24
        set_flags: bool, // Bit 20
        rn: u8,          // Bits 16-19 (primo operando)
        rd: u8,          // Bits 12-15 (destinazione)
        operand2: u32,   // Bits 0-11 (secondo operando, può essere registro o immediato)
        immediate: bool, // Bit 25
    },

    /// Branch and Branch with Link
    Branch {
        link: bool,  // Bit 24 (BL vs B)
        offset: i32, // Bits 0-23 (signed offset)
    },

    /// Branch and Exchange (switch ARM/THUMB)
    BranchExchange {
        rn: u8, // Bits 0-3 (registro con indirizzo)
    },

    /// Single Data Transfer (LDR/STR)
    SingleDataTransfer {
        load: bool,      // Bit 20 (LDR vs STR)
        byte: bool,      // Bit 22 (byte vs word)
        pre_index: bool, // Bit 24
        add: bool,       // Bit 23 (add vs subtract offset)
        writeback: bool, // Bit 21
        rn: u8,          // Bits 16-19 (base register)
        rd: u8,          // Bits 12-15 (source/dest)
        offset: u32,     // Bits 0-11
        immediate: bool, // Bit 25 (offset type)
    },

    /// Block Data Transfer (LDM/STM)
    BlockDataTransfer {
        load: bool,         // Bit 20 (LDM vs STM)
        pre_index: bool,    // Bit 24
        add: bool,          // Bit 23
        user_mode: bool,    // Bit 22
        writeback: bool,    // Bit 21
        rn: u8,             // Bits 16-19 (base)
        register_list: u16, // Bits 0-15
    },

    /// Multiply
    Multiply {
        accumulate: bool, // Bit 21 (MLA vs MUL)
        set_flags: bool,  // Bit 20
        rd: u8,           // Bits 16-19 (dest)
        rn: u8,           // Bits 12-15 (accumulator per MLA)
        rs: u8,           // Bits 8-11
        rm: u8,           // Bits 0-3
    },

    /// Software Interrupt
    SWI {
        comment: u32, // Bits 0-23
    },

    /// Istruzione non riconosciuta
    Undefined,
}

/// Decodifica un'istruzione ARM a 32-bit
pub fn decode_arm(instruction: u32) -> ArmInstruction {
    // Controllo per istruzioni speciali

    // Branch and Exchange: xxxx 0001 0010 1111 1111 1111 0001 rrrr
    if (instruction & 0x0FFF_FFF0) == 0x012F_FF10 {
        return ArmInstruction::BranchExchange {
            rn: (instruction & 0xF) as u8,
        };
    }

    // Software Interrupt: xxxx 1111 xxxx xxxx xxxx xxxx xxxx xxxx
    if (instruction & 0x0F00_0000) == 0x0F00_0000 {
        return ArmInstruction::SWI {
            comment: instruction & 0x00FF_FFFF,
        };
    }

    // Multiply: xxxx 0000 00as dddd nnnn ssss 1001 mmmm
    if (instruction & 0x0FC0_00F0) == 0x0000_0090 {
        return ArmInstruction::Multiply {
            accumulate: (instruction & (1 << 21)) != 0,
            set_flags: (instruction & (1 << 20)) != 0,
            rd: ((instruction >> 16) & 0xF) as u8,
            rn: ((instruction >> 12) & 0xF) as u8,
            rs: ((instruction >> 8) & 0xF) as u8,
            rm: (instruction & 0xF) as u8,
        };
    }

    // Block Data Transfer: xxxx 100p uswl nnnn llll llll llll llll
    if (instruction & 0x0E00_0000) == 0x0800_0000 {
        return ArmInstruction::BlockDataTransfer {
            load: (instruction & (1 << 20)) != 0,
            pre_index: (instruction & (1 << 24)) != 0,
            add: (instruction & (1 << 23)) != 0,
            user_mode: (instruction & (1 << 22)) != 0,
            writeback: (instruction & (1 << 21)) != 0,
            rn: ((instruction >> 16) & 0xF) as u8,
            register_list: (instruction & 0xFFFF) as u16,
        };
    }

    // Single Data Transfer: xxxx 01ip ubwl nnnn dddd oooo oooo oooo
    if (instruction & 0x0C00_0000) == 0x0400_0000 {
        return ArmInstruction::SingleDataTransfer {
            load: (instruction & (1 << 20)) != 0,
            byte: (instruction & (1 << 22)) != 0,
            pre_index: (instruction & (1 << 24)) != 0,
            add: (instruction & (1 << 23)) != 0,
            writeback: (instruction & (1 << 21)) != 0,
            rn: ((instruction >> 16) & 0xF) as u8,
            rd: ((instruction >> 12) & 0xF) as u8,
            offset: instruction & 0xFFF,
            immediate: (instruction & (1 << 25)) == 0, // Nota: invertito rispetto al bit I
        };
    }

    // Branch: xxxx 101l oooo oooo oooo oooo oooo oooo
    if (instruction & 0x0E00_0000) == 0x0A00_0000 {
        let mut offset = (instruction & 0x00FF_FFFF) as i32;
        // Sign extend da 24-bit a 32-bit
        if offset & 0x0080_0000 != 0 {
            offset |= 0xFF00_0000u32 as i32;
        }
        // Moltiplica per 4 (istruzioni sono word-aligned)
        offset = offset << 2;

        return ArmInstruction::Branch {
            link: (instruction & (1 << 24)) != 0,
            offset,
        };
    }

    // Data Processing: xxxx 00ip ppps nnnn dddd oooo oooo oooo
    if (instruction & 0x0C00_0000) == 0x0000_0000 {
        return ArmInstruction::DataProcessing {
            opcode: ((instruction >> 21) & 0xF) as u8,
            set_flags: (instruction & (1 << 20)) != 0,
            rn: ((instruction >> 16) & 0xF) as u8,
            rd: ((instruction >> 12) & 0xF) as u8,
            operand2: instruction & 0xFFF,
            immediate: (instruction & (1 << 25)) != 0,
        };
    }

    // Istruzione non riconosciuta
    ArmInstruction::Undefined
}

/// Opcodes per istruzioni Data Processing
#[allow(dead_code)]
pub mod data_processing {
    pub const AND: u8 = 0b0000; // Rd = Rn AND Op2
    pub const EOR: u8 = 0b0001; // Rd = Rn EOR Op2
    pub const SUB: u8 = 0b0010; // Rd = Rn - Op2
    pub const RSB: u8 = 0b0011; // Rd = Op2 - Rn
    pub const ADD: u8 = 0b0100; // Rd = Rn + Op2
    pub const ADC: u8 = 0b0101; // Rd = Rn + Op2 + Carry
    pub const SBC: u8 = 0b0110; // Rd = Rn - Op2 + Carry - 1
    pub const RSC: u8 = 0b0111; // Rd = Op2 - Rn + Carry - 1
    pub const TST: u8 = 0b1000; // Set flags da Rn AND Op2 (no write)
    pub const TEQ: u8 = 0b1001; // Set flags da Rn EOR Op2 (no write)
    pub const CMP: u8 = 0b1010; // Set flags da Rn - Op2 (no write)
    pub const CMN: u8 = 0b1011; // Set flags da Rn + Op2 (no write)
    pub const ORR: u8 = 0b1100; // Rd = Rn OR Op2
    pub const MOV: u8 = 0b1101; // Rd = Op2
    pub const BIC: u8 = 0b1110; // Rd = Rn AND NOT Op2
    pub const MVN: u8 = 0b1111; // Rd = NOT Op2
}
