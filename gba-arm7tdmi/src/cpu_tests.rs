#[cfg(test)]
mod tests {
    use crate::cpu::{MemoryBus, ARM7TDMI};

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
        assert!(!cpu.halted);
    }

    #[test]
    fn test_cpu_reset() {
        let mut cpu = ARM7TDMI::new();
        cpu.cycles = 1000;
        cpu.reset();
        assert_eq!(cpu.cycles, 0);
        assert_eq!(cpu.regs.pc(), 0);
    }

    #[test]
    fn test_mov_instruction() {
        // Test MOV R0, #42 con condition AL (sempre)
        // Formato: cond 00 I opcode S rn rd operand2
        // 1110 00 1 1101 0 0000 0000 000000101010
        // E3A0002A in hex

        struct TestBus {
            instructions: Vec<u32>,
        }

        impl MemoryBus for TestBus {
            fn read_word(&mut self, addr: u32) -> u32 {
                let idx = (addr / 4) as usize;
                if idx < self.instructions.len() {
                    self.instructions[idx]
                } else {
                    0
                }
            }
            fn read_byte(&mut self, _: u32) -> u8 {
                0
            }
            fn read_halfword(&mut self, _: u32) -> u16 {
                0
            }
            fn write_byte(&mut self, _: u32, _: u8) {}
            fn write_halfword(&mut self, _: u32, _: u16) {}
            fn write_word(&mut self, _: u32, _: u32) {}
        }

        let mut cpu = ARM7TDMI::new();
        let mut bus = TestBus {
            instructions: vec![0xE3A0_002A], // MOV R0, #42
        };

        cpu.step(&mut bus);

        assert_eq!(cpu.regs.r[0], 42);
        assert_eq!(cpu.regs.pc(), 4);
    }

    #[test]
    fn test_add_instruction() {
        // Test ADD R2, R0, R1
        // E0802001: ADD R2, R0, R1

        struct TestBus {
            instructions: Vec<u32>,
        }

        impl MemoryBus for TestBus {
            fn read_word(&mut self, addr: u32) -> u32 {
                let idx = (addr / 4) as usize;
                if idx < self.instructions.len() {
                    self.instructions[idx]
                } else {
                    0
                }
            }
            fn read_byte(&mut self, _: u32) -> u8 {
                0
            }
            fn read_halfword(&mut self, _: u32) -> u16 {
                0
            }
            fn write_byte(&mut self, _: u32, _: u8) {}
            fn write_halfword(&mut self, _: u32, _: u16) {}
            fn write_word(&mut self, _: u32, _: u32) {}
        }

        let mut cpu = ARM7TDMI::new();
        cpu.regs.r[0] = 10;
        cpu.regs.r[1] = 20;

        let mut bus = TestBus {
            instructions: vec![0xE080_2001], // ADD R2, R0, R1
        };

        cpu.step(&mut bus);

        assert_eq!(cpu.regs.r[2], 30);
    }

    #[test]
    fn test_branch_instruction() {
        // Test B #8 (salta avanti di 8 byte = 2 istruzioni)
        // EA000000: B #0 (offset 0 + 8 per PC)

        struct TestBus;
        impl MemoryBus for TestBus {
            fn read_word(&mut self, _: u32) -> u32 {
                0xEA00_0001 // B #4 (salta 1 istruzione avanti)
            }
            fn read_byte(&mut self, _: u32) -> u8 {
                0
            }
            fn read_halfword(&mut self, _: u32) -> u16 {
                0
            }
            fn write_byte(&mut self, _: u32, _: u8) {}
            fn write_halfword(&mut self, _: u32, _: u16) {}
            fn write_word(&mut self, _: u32, _: u32) {}
        }

        let mut cpu = ARM7TDMI::new();
        let mut bus = TestBus;

        cpu.step(&mut bus);

        // PC iniziale 0, legge istruzione, incrementa a 4
        // Branch con offset 1 word = 4 byte
        // Nuovo PC = 4 + 4 = 8
        assert_eq!(cpu.regs.pc(), 8);
    }

    #[test]
    fn test_ldr_str_instructions() {
        // Test STR e LDR
        use std::collections::HashMap;

        struct MemBus {
            memory: HashMap<u32, u32>,
            instructions: Vec<u32>,
        }

        impl MemoryBus for MemBus {
            fn read_word(&mut self, addr: u32) -> u32 {
                if addr < (self.instructions.len() * 4) as u32 {
                    self.instructions[(addr / 4) as usize]
                } else {
                    *self.memory.get(&(addr & !3)).unwrap_or(&0)
                }
            }
            fn write_word(&mut self, addr: u32, value: u32) {
                self.memory.insert(addr & !3, value);
            }
            fn read_byte(&mut self, _: u32) -> u8 {
                0
            }
            fn read_halfword(&mut self, _: u32) -> u16 {
                0
            }
            fn write_byte(&mut self, _: u32, _: u8) {}
            fn write_halfword(&mut self, _: u32, _: u16) {}
        }

        let mut cpu = ARM7TDMI::new();
        cpu.regs.r[0] = 0x1234_5678;
        cpu.regs.r[1] = 0x0300_0000; // Address in IWRAM

        let mut bus = MemBus {
            memory: HashMap::new(),
            instructions: vec![
                0xE581_0000, // STR R0, [R1]
                0xE591_2000, // LDR R2, [R1]
            ],
        };

        // STR R0, [R1]
        cpu.step(&mut bus);
        assert_eq!(bus.memory.get(&0x0300_0000), Some(&0x1234_5678));

        // LDR R2, [R1]
        cpu.step(&mut bus);
        assert_eq!(cpu.regs.r[2], 0x1234_5678);
    }

    #[test]
    fn test_thumb_mov_immediate() {
        // Test THUMB: MOV R0, #42
        // Format 3: 001 00 rd(3) imm(8)
        // 0010 0000 0010 1010 = 0x202A

        struct TestBus {
            instructions: Vec<u16>,
        }

        impl MemoryBus for TestBus {
            fn read_halfword(&mut self, addr: u32) -> u16 {
                let idx = (addr / 2) as usize;
                if idx < self.instructions.len() {
                    self.instructions[idx]
                } else {
                    0
                }
            }
            fn read_byte(&mut self, _: u32) -> u8 {
                0
            }
            fn read_word(&mut self, _: u32) -> u32 {
                0
            }
            fn write_byte(&mut self, _: u32, _: u8) {}
            fn write_halfword(&mut self, _: u32, _: u16) {}
            fn write_word(&mut self, _: u32, _: u32) {}
        }

        let mut cpu = ARM7TDMI::new();
        cpu.regs.set_thumb(true); // Modalità THUMB

        let mut bus = TestBus {
            instructions: vec![0x202A], // MOV R0, #42
        };

        cpu.step(&mut bus);

        assert_eq!(cpu.regs.r[0], 42);
        assert!(!cpu.regs.flag_z());
        assert_eq!(cpu.regs.pc(), 2); // THUMB incrementa di 2
    }

    #[test]
    fn test_thumb_add_subtract() {
        // Test THUMB: ADD R2, R0, R1
        // Format 2: 00011 0 0 rn(3) rs(3) rd(3)
        // 0001 1000 0100 0010 = 0x1842

        struct TestBus {
            instructions: Vec<u16>,
        }

        impl MemoryBus for TestBus {
            fn read_halfword(&mut self, addr: u32) -> u16 {
                let idx = (addr / 2) as usize;
                if idx < self.instructions.len() {
                    self.instructions[idx]
                } else {
                    0
                }
            }
            fn read_byte(&mut self, _: u32) -> u8 {
                0
            }
            fn read_word(&mut self, _: u32) -> u32 {
                0
            }
            fn write_byte(&mut self, _: u32, _: u8) {}
            fn write_halfword(&mut self, _: u32, _: u16) {}
            fn write_word(&mut self, _: u32, _: u32) {}
        }

        let mut cpu = ARM7TDMI::new();
        cpu.regs.set_thumb(true);
        cpu.regs.r[0] = 10;
        cpu.regs.r[1] = 20;

        let mut bus = TestBus {
            instructions: vec![0x1842], // ADD R2, R0, R1
        };

        cpu.step(&mut bus);

        assert_eq!(cpu.regs.r[2], 30);
        assert!(!cpu.regs.flag_z());
        assert!(!cpu.regs.flag_n());
    }

    #[test]
    fn test_thumb_ldr_str() {
        // Test THUMB: STR R0, [R1, #4] e LDR R2, [R1, #4]
        use std::collections::HashMap;

        struct MemBus {
            memory: HashMap<u32, u32>,
            instructions: Vec<u16>,
        }

        impl MemoryBus for MemBus {
            fn read_halfword(&mut self, addr: u32) -> u16 {
                if addr < (self.instructions.len() * 2) as u32 {
                    self.instructions[(addr / 2) as usize]
                } else {
                    0
                }
            }
            fn read_word(&mut self, addr: u32) -> u32 {
                *self.memory.get(&(addr & !3)).unwrap_or(&0)
            }
            fn write_word(&mut self, addr: u32, value: u32) {
                self.memory.insert(addr & !3, value);
            }
            fn read_byte(&mut self, _: u32) -> u8 {
                0
            }
            fn write_byte(&mut self, _: u32, _: u8) {}
            fn write_halfword(&mut self, _: u32, _: u16) {}
        }

        let mut cpu = ARM7TDMI::new();
        cpu.regs.set_thumb(true);
        cpu.regs.r[0] = 0xABCD_1234;
        cpu.regs.r[1] = 0x0300_0000;

        let mut bus = MemBus {
            memory: HashMap::new(),
            instructions: vec![
                0x6048, // STR R0, [R1, #4]
                0x684A, // LDR R2, [R1, #4]
            ],
        };

        // STR
        cpu.step(&mut bus);
        assert_eq!(bus.memory.get(&0x0300_0004), Some(&0xABCD_1234));

        // LDR
        cpu.step(&mut bus);
        assert_eq!(cpu.regs.r[2], 0xABCD_1234);
    }

    #[test]
    fn test_thumb_branch() {
        // Test THUMB: B #4 (offset 2 = salta 2 halfwords = 4 byte)
        // Format 18: 11100 offset(11)
        // 1110 0000 0000 0010 = 0xE002

        struct TestBus;
        impl MemoryBus for TestBus {
            fn read_halfword(&mut self, _: u32) -> u16 {
                0xE002 // B #+4
            }
            fn read_byte(&mut self, _: u32) -> u8 {
                0
            }
            fn read_word(&mut self, _: u32) -> u32 {
                0
            }
            fn write_byte(&mut self, _: u32, _: u8) {}
            fn write_halfword(&mut self, _: u32, _: u16) {}
            fn write_word(&mut self, _: u32, _: u32) {}
        }

        let mut cpu = ARM7TDMI::new();
        cpu.regs.set_thumb(true);
        let mut bus = TestBus;

        cpu.step(&mut bus);

        // PC dopo step = 2, branch offset 2*2 = 4, quindi PC finale = 2+4 = 6
        assert_eq!(cpu.regs.pc(), 6);
    }
}
