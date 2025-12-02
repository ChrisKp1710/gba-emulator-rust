// Implementazione istruzioni THUMB (16-bit)
//
// Le istruzioni THUMB sono più compatte (16-bit vs 32-bit ARM)
// e vengono usate dalla maggior parte dei giochi GBA per:
// - Ridurre dimensione ROM
// - Migliorare performance cache
// - Codice più denso
//
// LIMITAZIONI THUMB:
// - Nessun condition code (tranne branch condizionali)
// - Accesso limitato ai registri (spesso solo R0-R7)
// - Set istruzioni ridotto
//
// VANTAGGI:
// - Codice 30-40% più piccolo
// - Migliore uso della cache a 16-bit del GBA
// - Switch facile con ARM tramite BX

/// Tipi di istruzioni THUMB (16-bit)
#[derive(Debug, Clone, Copy)]
pub enum ThumbInstruction {
    /// Format 1: Move shifted register
    /// LSL/LSR/ASR Rd, Rs, #Offset5
    MoveShiftedRegister {
        op: u8,     // 0=LSL, 1=LSR, 2=ASR
        offset: u8, // Bits 6-10 (shift amount)
        rs: u8,     // Bits 3-5 (source)
        rd: u8,     // Bits 0-2 (dest)
    },

    /// Format 2: Add/subtract
    /// ADD/SUB Rd, Rs, Rn/#Offset3
    AddSubtract {
        sub: bool,       // Bit 9 (0=ADD, 1=SUB)
        immediate: bool, // Bit 10 (0=register, 1=immediate)
        rn_offset: u8,   // Bits 6-8 (Rn o immediate)
        rs: u8,          // Bits 3-5
        rd: u8,          // Bits 0-2
    },

    /// Format 3: Move/compare/add/subtract immediate
    /// MOV/CMP/ADD/SUB Rd, #Offset8
    AluImmediate {
        op: u8,     // Bits 11-12 (0=MOV, 1=CMP, 2=ADD, 3=SUB)
        rd: u8,     // Bits 8-10
        offset: u8, // Bits 0-7 (immediate value)
    },

    /// Format 4: ALU operations
    /// AND/EOR/LSL/LSR/ASR/ADC/SBC/ROR/TST/NEG/CMP/CMN/ORR/MUL/BIC/MVN
    AluOperation {
        op: u8, // Bits 6-9 (operazione)
        rs: u8, // Bits 3-5
        rd: u8, // Bits 0-2
    },

    /// Format 5: Hi register operations/branch exchange
    /// ADD/CMP/MOV/BX con accesso a R8-R15
    HiRegisterOps {
        op: u8,   // Bits 8-9 (0=ADD, 1=CMP, 2=MOV, 3=BX)
        h1: bool, // Bit 7 (Rd hi)
        h2: bool, // Bit 6 (Rs hi)
        rs: u8,   // Bits 3-5
        rd: u8,   // Bits 0-2
    },

    /// Format 6: PC-relative load
    /// LDR Rd, [PC, #Imm8*4]
    LoadPcRelative {
        rd: u8,     // Bits 8-10
        offset: u8, // Bits 0-7 (*4)
    },

    /// Format 7: Load/store with register offset
    /// LDR/STR/LDRB/STRB Rd, [Rb, Ro]
    LoadStoreRegOffset {
        load: bool, // Bit 11 (0=STR, 1=LDR)
        byte: bool, // Bit 10 (0=word, 1=byte)
        ro: u8,     // Bits 6-8 (offset register)
        rb: u8,     // Bits 3-5 (base)
        rd: u8,     // Bits 0-2
    },

    /// Format 8: Load/store sign-extended byte/halfword
    /// LDRH/STRH/LDSB/LDSH Rd, [Rb, Ro]
    LoadStoreSignExtended {
        h: bool,    // Bit 11 (0=byte, 1=halfword)
        sign: bool, // Bit 10 (0=unsigned, 1=signed)
        ro: u8,     // Bits 6-8
        rb: u8,     // Bits 3-5
        rd: u8,     // Bits 0-2
    },

    /// Format 9: Load/store with immediate offset
    /// LDR/STR/LDRB/STRB Rd, [Rb, #Imm5]
    LoadStoreImmOffset {
        load: bool, // Bit 11 (0=STR, 1=LDR)
        byte: bool, // Bit 12 (0=word, 1=byte)
        offset: u8, // Bits 6-10
        rb: u8,     // Bits 3-5
        rd: u8,     // Bits 0-2
    },

    /// Format 10: Load/store halfword
    /// LDRH/STRH Rd, [Rb, #Imm5*2]
    LoadStoreHalfword {
        load: bool, // Bit 11 (0=STRH, 1=LDRH)
        offset: u8, // Bits 6-10 (*2)
        rb: u8,     // Bits 3-5
        rd: u8,     // Bits 0-2
    },

    /// Format 11: SP-relative load/store
    /// LDR/STR Rd, [SP, #Imm8*4]
    LoadStoreSpRelative {
        load: bool, // Bit 11 (0=STR, 1=LDR)
        rd: u8,     // Bits 8-10
        offset: u8, // Bits 0-7 (*4)
    },

    /// Format 12: Load address
    /// ADD Rd, PC/SP, #Imm8*4
    LoadAddress {
        sp: bool,   // Bit 11 (0=PC, 1=SP)
        rd: u8,     // Bits 8-10
        offset: u8, // Bits 0-7 (*4)
    },

    /// Format 13: Add offset to stack pointer
    /// ADD SP, #Imm7*4 o SUB SP, #Imm7*4
    AddOffsetSp {
        sub: bool,  // Bit 7 (0=ADD, 1=SUB)
        offset: u8, // Bits 0-6 (*4)
    },

    /// Format 14: Push/pop registers
    /// PUSH {Rlist, LR} / POP {Rlist, PC}
    PushPop {
        load: bool, // Bit 11 (0=PUSH, 1=POP)
        r: bool,    // Bit 8 (include LR/PC)
        rlist: u8,  // Bits 0-7
    },

    /// Format 15: Multiple load/store
    /// LDMIA/STMIA Rb!, {Rlist}
    LoadStoreMultiple {
        load: bool, // Bit 11 (0=STMIA, 1=LDMIA)
        rb: u8,     // Bits 8-10
        rlist: u8,  // Bits 0-7
    },

    /// Format 16: Conditional branch
    /// B<cond> label
    ConditionalBranch {
        cond: u8,   // Bits 8-11 (condition)
        offset: i8, // Bits 0-7 (signed, *2)
    },

    /// Format 17: Software interrupt
    /// SWI #Imm8
    SoftwareInterrupt {
        comment: u8, // Bits 0-7
    },

    /// Format 18: Unconditional branch
    /// B label
    UnconditionalBranch {
        offset: i16, // Bits 0-10 (signed, *2)
    },

    /// Format 19: Long branch with link
    /// BL label (first or second instruction)
    LongBranchLink {
        first_instruction: bool, // Bit 11 (1=first H, 0=second L)
        offset: u16,             // Bits 0-10
    },

    /// Istruzione non riconosciuta
    Undefined,
}

/// Decodifica un'istruzione THUMB a 16-bit
pub fn decode_thumb(instruction: u16) -> ThumbInstruction {
    // Format 1: Move shifted register (000xx)
    if (instruction & 0xE000) == 0x0000 {
        let op = ((instruction >> 11) & 0x3) as u8;
        if op == 3 {
            // Format 2: Add/subtract (00011)
            return ThumbInstruction::AddSubtract {
                sub: (instruction & (1 << 9)) != 0,
                immediate: (instruction & (1 << 10)) != 0,
                rn_offset: ((instruction >> 6) & 0x7) as u8,
                rs: ((instruction >> 3) & 0x7) as u8,
                rd: (instruction & 0x7) as u8,
            };
        }
        return ThumbInstruction::MoveShiftedRegister {
            op,
            offset: ((instruction >> 6) & 0x1F) as u8,
            rs: ((instruction >> 3) & 0x7) as u8,
            rd: (instruction & 0x7) as u8,
        };
    }

    // Format 3: Move/compare/add/subtract immediate (001xx)
    if (instruction & 0xE000) == 0x2000 {
        return ThumbInstruction::AluImmediate {
            op: ((instruction >> 11) & 0x3) as u8,
            rd: ((instruction >> 8) & 0x7) as u8,
            offset: (instruction & 0xFF) as u8,
        };
    }

    // Format 4: ALU operations (010000)
    if (instruction & 0xFC00) == 0x4000 {
        return ThumbInstruction::AluOperation {
            op: ((instruction >> 6) & 0xF) as u8,
            rs: ((instruction >> 3) & 0x7) as u8,
            rd: (instruction & 0x7) as u8,
        };
    }

    // Format 5: Hi register operations/branch exchange (010001)
    if (instruction & 0xFC00) == 0x4400 {
        return ThumbInstruction::HiRegisterOps {
            op: ((instruction >> 8) & 0x3) as u8,
            h1: (instruction & (1 << 7)) != 0,
            h2: (instruction & (1 << 6)) != 0,
            rs: ((instruction >> 3) & 0x7) as u8,
            rd: (instruction & 0x7) as u8,
        };
    }

    // Format 6: PC-relative load (01001)
    if (instruction & 0xF800) == 0x4800 {
        return ThumbInstruction::LoadPcRelative {
            rd: ((instruction >> 8) & 0x7) as u8,
            offset: (instruction & 0xFF) as u8,
        };
    }

    // Format 7: Load/store with register offset (0101xx0)
    if (instruction & 0xF200) == 0x5000 {
        return ThumbInstruction::LoadStoreRegOffset {
            load: (instruction & (1 << 11)) != 0,
            byte: (instruction & (1 << 10)) != 0,
            ro: ((instruction >> 6) & 0x7) as u8,
            rb: ((instruction >> 3) & 0x7) as u8,
            rd: (instruction & 0x7) as u8,
        };
    }

    // Format 8: Load/store sign-extended byte/halfword (0101xx1)
    if (instruction & 0xF200) == 0x5200 {
        return ThumbInstruction::LoadStoreSignExtended {
            h: (instruction & (1 << 11)) != 0,
            sign: (instruction & (1 << 10)) != 0,
            ro: ((instruction >> 6) & 0x7) as u8,
            rb: ((instruction >> 3) & 0x7) as u8,
            rd: (instruction & 0x7) as u8,
        };
    }

    // Format 9: Load/store with immediate offset (011xx)
    if (instruction & 0xE000) == 0x6000 {
        return ThumbInstruction::LoadStoreImmOffset {
            byte: (instruction & (1 << 12)) != 0,
            load: (instruction & (1 << 11)) != 0,
            offset: ((instruction >> 6) & 0x1F) as u8,
            rb: ((instruction >> 3) & 0x7) as u8,
            rd: (instruction & 0x7) as u8,
        };
    }

    // Format 10: Load/store halfword (1000x)
    if (instruction & 0xF000) == 0x8000 {
        return ThumbInstruction::LoadStoreHalfword {
            load: (instruction & (1 << 11)) != 0,
            offset: ((instruction >> 6) & 0x1F) as u8,
            rb: ((instruction >> 3) & 0x7) as u8,
            rd: (instruction & 0x7) as u8,
        };
    }

    // Format 11: SP-relative load/store (1001x)
    if (instruction & 0xF000) == 0x9000 {
        return ThumbInstruction::LoadStoreSpRelative {
            load: (instruction & (1 << 11)) != 0,
            rd: ((instruction >> 8) & 0x7) as u8,
            offset: (instruction & 0xFF) as u8,
        };
    }

    // Format 12: Load address (1010x)
    if (instruction & 0xF000) == 0xA000 {
        return ThumbInstruction::LoadAddress {
            sp: (instruction & (1 << 11)) != 0,
            rd: ((instruction >> 8) & 0x7) as u8,
            offset: (instruction & 0xFF) as u8,
        };
    }

    // Format 13: Add offset to stack pointer (10110000)
    if (instruction & 0xFF00) == 0xB000 {
        return ThumbInstruction::AddOffsetSp {
            sub: (instruction & (1 << 7)) != 0,
            offset: (instruction & 0x7F) as u8,
        };
    }

    // Format 14: Push/pop registers (1011x10x)
    if (instruction & 0xF600) == 0xB400 {
        return ThumbInstruction::PushPop {
            load: (instruction & (1 << 11)) != 0,
            r: (instruction & (1 << 8)) != 0,
            rlist: (instruction & 0xFF) as u8,
        };
    }

    // Format 15: Multiple load/store (1100x)
    if (instruction & 0xF000) == 0xC000 {
        return ThumbInstruction::LoadStoreMultiple {
            load: (instruction & (1 << 11)) != 0,
            rb: ((instruction >> 8) & 0x7) as u8,
            rlist: (instruction & 0xFF) as u8,
        };
    }

    // Format 17: Software interrupt (11011111)
    if (instruction & 0xFF00) == 0xDF00 {
        return ThumbInstruction::SoftwareInterrupt {
            comment: (instruction & 0xFF) as u8,
        };
    }

    // Format 16: Conditional branch (1101xxxx, but not 1111)
    if (instruction & 0xF000) == 0xD000 {
        let cond = ((instruction >> 8) & 0xF) as u8;
        if cond != 0xF {
            return ThumbInstruction::ConditionalBranch {
                cond,
                offset: (instruction & 0xFF) as i8,
            };
        }
    }

    // Format 18: Unconditional branch (11100)
    if (instruction & 0xF800) == 0xE000 {
        let mut offset = (instruction & 0x7FF) as i16;
        // Sign extend da 11-bit
        if offset & 0x0400 != 0 {
            offset |= 0xF800u16 as i16;
        }
        return ThumbInstruction::UnconditionalBranch { offset };
    }

    // Format 19: Long branch with link (1111x)
    if (instruction & 0xF000) == 0xF000 {
        return ThumbInstruction::LongBranchLink {
            first_instruction: (instruction & (1 << 11)) != 0,
            offset: (instruction & 0x7FF) as u16,
        };
    }

    // Istruzione non riconosciuta
    ThumbInstruction::Undefined
}

/// Opcodes per Format 4 (ALU operations)
#[allow(dead_code)]
pub mod thumb_alu {
    pub const AND: u8 = 0x0;
    pub const EOR: u8 = 0x1;
    pub const LSL: u8 = 0x2;
    pub const LSR: u8 = 0x3;
    pub const ASR: u8 = 0x4;
    pub const ADC: u8 = 0x5;
    pub const SBC: u8 = 0x6;
    pub const ROR: u8 = 0x7;
    pub const TST: u8 = 0x8;
    pub const NEG: u8 = 0x9;
    pub const CMP: u8 = 0xA;
    pub const CMN: u8 = 0xB;
    pub const ORR: u8 = 0xC;
    pub const MUL: u8 = 0xD;
    pub const BIC: u8 = 0xE;
    pub const MVN: u8 = 0xF;
}
