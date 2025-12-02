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
        let instruction = bus.read_word(pc);
        self.regs.set_pc(pc.wrapping_add(4));

        // Verifica condition code
        let condition = crate::arm::Condition::from_opcode(instruction);
        if !condition.check(self.regs.cpsr) {
            return 1; // Istruzione skippata, 1 ciclo
        }

        // Decodifica istruzione
        use crate::arm::ArmInstruction;
        let decoded = crate::arm::decode_arm(instruction);

        // Esegui in base al tipo
        match decoded {
            ArmInstruction::DataProcessing {
                opcode,
                set_flags,
                rn,
                rd,
                operand2,
                immediate,
            } => {
                let (op2_value, carry) =
                    crate::instructions::alu::decode_operand2(operand2, immediate, &self.regs);
                crate::instructions::alu::execute_data_processing(
                    &mut self.regs,
                    opcode,
                    rd,
                    rn,
                    op2_value,
                    set_flags,
                    carry,
                )
            }

            ArmInstruction::Branch { link, offset } => {
                crate::instructions::branch::execute_branch(&mut self.regs, offset, link)
            }

            ArmInstruction::BranchExchange { rn } => {
                crate::instructions::branch::execute_branch_exchange(&mut self.regs, rn)
            }

            ArmInstruction::SingleDataTransfer {
                load,
                byte,
                pre_index,
                add,
                writeback,
                rn,
                rd,
                offset,
                immediate,
            } => {
                let offset_val = if immediate {
                    offset
                } else {
                    // Offset è un registro con shift
                    let (val, _) =
                        crate::instructions::alu::decode_operand2(offset, false, &self.regs);
                    val
                };
                crate::instructions::load_store::execute_single_data_transfer(
                    &mut self.regs,
                    bus,
                    load,
                    byte,
                    pre_index,
                    add,
                    writeback,
                    rn,
                    rd,
                    offset_val,
                )
            }

            ArmInstruction::BlockDataTransfer {
                load,
                pre_index,
                add,
                writeback,
                rn,
                register_list,
                ..
            } => crate::instructions::load_store::execute_block_data_transfer(
                &mut self.regs,
                bus,
                load,
                pre_index,
                add,
                writeback,
                rn,
                register_list,
            ),

            ArmInstruction::Multiply {
                accumulate,
                set_flags,
                rd,
                rn,
                rs,
                rm,
            } => {
                // Implementazione base multiply
                let rm_val = self.regs.r[rm as usize];
                let rs_val = self.regs.r[rs as usize];
                let mut result = rm_val.wrapping_mul(rs_val);

                if accumulate {
                    let rn_val = self.regs.r[rn as usize];
                    result = result.wrapping_add(rn_val);
                }

                self.regs.r[rd as usize] = result;

                if set_flags {
                    self.regs.set_flag_n((result & 0x8000_0000) != 0);
                    self.regs.set_flag_z(result == 0);
                    // C è undefined, V non modificato
                }

                // MUL/MLA: dipende dai cicli, base 1-4
                2
            }

            ArmInstruction::SWI { comment: _ } => {
                // Software Interrupt (syscall)
                // Salva stato e salta a SWI handler
                let pc = self.regs.pc();
                self.regs.change_mode(crate::registers::Mode::Supervisor);
                self.regs.set_spsr(self.regs.cpsr);
                self.regs.r[14] = pc; // Salva LR
                self.regs.set_pc(0x08); // SWI vector
                3
            }

            ArmInstruction::Undefined => {
                // Istruzione non riconosciuta
                // TODO: Generare undefined instruction exception
                1
            }
        }
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
        let instruction = bus.read_halfword(pc);
        self.regs.set_pc(pc.wrapping_add(2));

        // Decodifica istruzione THUMB
        use crate::thumb::ThumbInstruction;
        let decoded = crate::thumb::decode_thumb(instruction);

        // Esegui in base al tipo
        match decoded {
            ThumbInstruction::MoveShiftedRegister { op, offset, rs, rd } => {
                let value = self.regs.r[rs as usize];
                let result = match op {
                    0 => value << offset, // LSL
                    1 => {
                        if offset == 0 {
                            0
                        } else {
                            value >> offset
                        }
                    } // LSR
                    2 => {
                        // ASR
                        if offset == 0 {
                            if (value & 0x80000000) != 0 {
                                0xFFFFFFFF
                            } else {
                                0
                            }
                        } else {
                            ((value as i32) >> offset) as u32
                        }
                    }
                    _ => value,
                };

                self.regs.r[rd as usize] = result;
                self.regs.set_flag_n((result & 0x80000000) != 0);
                self.regs.set_flag_z(result == 0);
                if offset != 0 && op == 0 {
                    self.regs.set_flag_c((value & (1 << (32 - offset))) != 0);
                } else if offset != 0 {
                    self.regs.set_flag_c((value & (1 << (offset - 1))) != 0);
                }
                1
            }

            ThumbInstruction::AddSubtract {
                sub,
                immediate,
                rn_offset,
                rs,
                rd,
            } => {
                let rs_val = self.regs.r[rs as usize];
                let operand = if immediate {
                    rn_offset as u32
                } else {
                    self.regs.r[rn_offset as usize]
                };

                let (result, carry, overflow) = if sub {
                    let res = rs_val.wrapping_sub(operand);
                    let c = rs_val >= operand;
                    let v = ((rs_val ^ operand) & (rs_val ^ res) & 0x80000000) != 0;
                    (res, c, v)
                } else {
                    let res = rs_val.wrapping_add(operand);
                    let c = (rs_val as u64 + operand as u64) > 0xFFFFFFFF;
                    let v = ((rs_val ^ res) & (operand ^ res) & 0x80000000) != 0;
                    (res, c, v)
                };

                self.regs.r[rd as usize] = result;
                self.regs.set_flag_n((result & 0x80000000) != 0);
                self.regs.set_flag_z(result == 0);
                self.regs.set_flag_c(carry);
                self.regs.set_flag_v(overflow);
                1
            }

            ThumbInstruction::AluImmediate { op, rd, offset } => {
                let imm = offset as u32;
                let rd_val = self.regs.r[rd as usize];

                match op {
                    0 => {
                        // MOV
                        self.regs.r[rd as usize] = imm;
                        self.regs.set_flag_n(false);
                        self.regs.set_flag_z(imm == 0);
                    }
                    1 => {
                        // CMP
                        let result = rd_val.wrapping_sub(imm);
                        self.regs.set_flag_n((result & 0x80000000) != 0);
                        self.regs.set_flag_z(result == 0);
                        self.regs.set_flag_c(rd_val >= imm);
                        self.regs
                            .set_flag_v(((rd_val ^ imm) & (rd_val ^ result) & 0x80000000) != 0);
                    }
                    2 => {
                        // ADD
                        let result = rd_val.wrapping_add(imm);
                        self.regs.r[rd as usize] = result;
                        self.regs.set_flag_n((result & 0x80000000) != 0);
                        self.regs.set_flag_z(result == 0);
                        self.regs
                            .set_flag_c((rd_val as u64 + imm as u64) > 0xFFFFFFFF);
                        self.regs
                            .set_flag_v(((rd_val ^ result) & (imm ^ result) & 0x80000000) != 0);
                    }
                    3 => {
                        // SUB
                        let result = rd_val.wrapping_sub(imm);
                        self.regs.r[rd as usize] = result;
                        self.regs.set_flag_n((result & 0x80000000) != 0);
                        self.regs.set_flag_z(result == 0);
                        self.regs.set_flag_c(rd_val >= imm);
                        self.regs
                            .set_flag_v(((rd_val ^ imm) & (rd_val ^ result) & 0x80000000) != 0);
                    }
                    _ => {}
                }
                1
            }

            ThumbInstruction::AluOperation { op, rs, rd } => {
                let rd_val = self.regs.r[rd as usize];
                let rs_val = self.regs.r[rs as usize];

                use crate::thumb::thumb_alu::*;
                let result = match op {
                    AND => rd_val & rs_val,
                    EOR => rd_val ^ rs_val,
                    LSL => rd_val << (rs_val & 0xFF),
                    LSR => rd_val >> (rs_val & 0xFF),
                    ASR => ((rd_val as i32) >> (rs_val & 0xFF)) as u32,
                    ADC => {
                        let c = if self.regs.flag_c() { 1 } else { 0 };
                        rd_val.wrapping_add(rs_val).wrapping_add(c)
                    }
                    SBC => {
                        let c = if self.regs.flag_c() { 0 } else { 1 };
                        rd_val.wrapping_sub(rs_val).wrapping_sub(c)
                    }
                    ROR => rd_val.rotate_right(rs_val & 0xFF),
                    TST => rd_val & rs_val,
                    NEG => 0u32.wrapping_sub(rs_val),
                    CMP => rd_val.wrapping_sub(rs_val),
                    CMN => rd_val.wrapping_add(rs_val),
                    ORR => rd_val | rs_val,
                    MUL => rd_val.wrapping_mul(rs_val),
                    BIC => rd_val & !rs_val,
                    MVN => !rs_val,
                    _ => rd_val,
                };

                // TST, CMP, CMN non scrivono risultato
                if op != TST && op != CMP && op != CMN {
                    self.regs.r[rd as usize] = result;
                }

                // Aggiorna flag
                self.regs.set_flag_n((result & 0x80000000) != 0);
                self.regs.set_flag_z(result == 0);

                if op == CMP || op == CMN {
                    if op == CMP {
                        self.regs.set_flag_c(rd_val >= rs_val);
                    } else {
                        self.regs
                            .set_flag_c((rd_val as u64 + rs_val as u64) > 0xFFFFFFFF);
                    }
                }

                1
            }

            ThumbInstruction::HiRegisterOps { op, h1, h2, rs, rd } => {
                let rd_idx = (rd as usize) | (if h1 { 8 } else { 0 });
                let rs_idx = (rs as usize) | (if h2 { 8 } else { 0 });

                match op {
                    0 => {
                        // ADD
                        let result = self.regs.r[rd_idx].wrapping_add(self.regs.r[rs_idx]);
                        if rd_idx == 15 {
                            self.regs.set_pc(result & !1);
                        } else {
                            self.regs.r[rd_idx] = result;
                        }
                    }
                    1 => {
                        // CMP
                        let result = self.regs.r[rd_idx].wrapping_sub(self.regs.r[rs_idx]);
                        self.regs.set_flag_n((result & 0x80000000) != 0);
                        self.regs.set_flag_z(result == 0);
                        self.regs
                            .set_flag_c(self.regs.r[rd_idx] >= self.regs.r[rs_idx]);
                        self.regs.set_flag_v(
                            ((self.regs.r[rd_idx] ^ self.regs.r[rs_idx])
                                & (self.regs.r[rd_idx] ^ result)
                                & 0x80000000)
                                != 0,
                        );
                    }
                    2 => {
                        // MOV
                        let value = self.regs.r[rs_idx];
                        if rd_idx == 15 {
                            self.regs.set_pc(value & !1);
                        } else {
                            self.regs.r[rd_idx] = value;
                        }
                    }
                    3 => {
                        // BX
                        let target = self.regs.r[rs_idx];
                        if (target & 1) != 0 {
                            self.regs.set_pc(target & !1);
                            self.regs.set_thumb(true);
                        } else {
                            self.regs.set_pc(target & !3);
                            self.regs.set_thumb(false);
                        }
                        return 3;
                    }
                    _ => {}
                }
                1
            }

            ThumbInstruction::LoadPcRelative { rd, offset } => {
                let pc = self.regs.pc() & !2;
                let address = pc.wrapping_add((offset as u32) << 2);
                let value = bus.read_word(address & !3);
                self.regs.r[rd as usize] = value;
                3
            }

            ThumbInstruction::LoadStoreRegOffset {
                load,
                byte,
                ro,
                rb,
                rd,
            } => {
                let address = self.regs.r[rb as usize].wrapping_add(self.regs.r[ro as usize]);
                if load {
                    let value = if byte {
                        bus.read_byte(address) as u32
                    } else {
                        bus.read_word(address & !3)
                    };
                    self.regs.r[rd as usize] = value;
                } else {
                    let value = self.regs.r[rd as usize];
                    if byte {
                        bus.write_byte(address, value as u8);
                    } else {
                        bus.write_word(address & !3, value);
                    }
                }
                if load {
                    3
                } else {
                    2
                }
            }

            ThumbInstruction::LoadStoreImmOffset {
                load,
                byte,
                offset,
                rb,
                rd,
            } => {
                let off = if byte {
                    offset as u32
                } else {
                    (offset as u32) << 2
                };
                let address = self.regs.r[rb as usize].wrapping_add(off);

                if load {
                    let value = if byte {
                        bus.read_byte(address) as u32
                    } else {
                        bus.read_word(address & !3)
                    };
                    self.regs.r[rd as usize] = value;
                } else {
                    let value = self.regs.r[rd as usize];
                    if byte {
                        bus.write_byte(address, value as u8);
                    } else {
                        bus.write_word(address & !3, value);
                    }
                }
                if load {
                    3
                } else {
                    2
                }
            }

            ThumbInstruction::LoadStoreHalfword {
                load,
                offset,
                rb,
                rd,
            } => {
                let address = self.regs.r[rb as usize].wrapping_add((offset as u32) << 1);
                if load {
                    let value = bus.read_halfword(address & !1) as u32;
                    self.regs.r[rd as usize] = value;
                } else {
                    bus.write_halfword(address & !1, self.regs.r[rd as usize] as u16);
                }
                if load {
                    3
                } else {
                    2
                }
            }

            ThumbInstruction::LoadStoreSpRelative { load, rd, offset } => {
                let sp = self.regs.r[13];
                let address = sp.wrapping_add((offset as u32) << 2);
                if load {
                    self.regs.r[rd as usize] = bus.read_word(address & !3);
                } else {
                    bus.write_word(address & !3, self.regs.r[rd as usize]);
                }
                if load {
                    3
                } else {
                    2
                }
            }

            ThumbInstruction::LoadAddress { sp, rd, offset } => {
                let base = if sp {
                    self.regs.r[13]
                } else {
                    self.regs.pc() & !2
                };
                self.regs.r[rd as usize] = base.wrapping_add((offset as u32) << 2);
                1
            }

            ThumbInstruction::AddOffsetSp { sub, offset } => {
                let off = (offset as u32) << 2;
                if sub {
                    self.regs.r[13] = self.regs.r[13].wrapping_sub(off);
                } else {
                    self.regs.r[13] = self.regs.r[13].wrapping_add(off);
                }
                1
            }

            ThumbInstruction::PushPop { load, r, rlist } => {
                let mut sp = self.regs.r[13];
                let mut cycles = 0;

                if load {
                    // POP
                    for i in 0..8 {
                        if (rlist & (1 << i)) != 0 {
                            self.regs.r[i] = bus.read_word(sp);
                            sp = sp.wrapping_add(4);
                            cycles += 1;
                        }
                    }
                    if r {
                        let pc = bus.read_word(sp);
                        self.regs.set_pc(pc & !1);
                        sp = sp.wrapping_add(4);
                        cycles += 1;
                    }
                } else {
                    // PUSH
                    if r {
                        sp = sp.wrapping_sub(4);
                        bus.write_word(sp, self.regs.r[14]);
                        cycles += 1;
                    }
                    for i in (0..8).rev() {
                        if (rlist & (1 << i)) != 0 {
                            sp = sp.wrapping_sub(4);
                            bus.write_word(sp, self.regs.r[i]);
                            cycles += 1;
                        }
                    }
                }

                self.regs.r[13] = sp;
                cycles.max(1)
            }

            ThumbInstruction::LoadStoreMultiple { load, rb, rlist } => {
                let mut address = self.regs.r[rb as usize];
                let mut cycles = 0;

                for i in 0..8 {
                    if (rlist & (1 << i)) != 0 {
                        if load {
                            self.regs.r[i] = bus.read_word(address);
                        } else {
                            bus.write_word(address, self.regs.r[i]);
                        }
                        address = address.wrapping_add(4);
                        cycles += 1;
                    }
                }

                self.regs.r[rb as usize] = address;
                cycles.max(1)
            }

            ThumbInstruction::ConditionalBranch { cond, offset } => {
                let condition = crate::arm::Condition::from_opcode((cond as u32) << 28);
                if condition.check(self.regs.cpsr) {
                    let pc = self.regs.pc();
                    let offset_ext = ((offset as i32) << 1) as u32;
                    self.regs.set_pc(pc.wrapping_add(offset_ext));
                    return 3;
                }
                1
            }

            ThumbInstruction::UnconditionalBranch { offset } => {
                let pc = self.regs.pc();
                let offset_val = ((offset as i32) << 1) as u32;
                self.regs.set_pc(pc.wrapping_add(offset_val));
                3
            }

            ThumbInstruction::LongBranchLink {
                first_instruction,
                offset,
            } => {
                if first_instruction {
                    // Prima istruzione: LR = PC + (offset << 12)
                    let pc = self.regs.pc();
                    let mut off = offset as i32;
                    if off & 0x400 != 0 {
                        off |= !0x7FF;
                    }
                    self.regs.r[14] = pc.wrapping_add(((off << 12) as u32));
                } else {
                    // Seconda istruzione: PC = LR + (offset << 1), LR = next instruction
                    let lr = self.regs.r[14];
                    let next_pc = self.regs.pc().wrapping_sub(2);
                    self.regs.set_pc(lr.wrapping_add((offset as u32) << 1));
                    self.regs.r[14] = next_pc | 1;
                }
                3
            }

            ThumbInstruction::SoftwareInterrupt { comment: _ } => {
                let pc = self.regs.pc();
                self.regs.change_mode(crate::registers::Mode::Supervisor);
                self.regs.set_spsr(self.regs.cpsr);
                self.regs.r[14] = pc;
                self.regs.set_pc(0x08);
                self.regs.set_thumb(false); // SWI handler è in ARM mode
                3
            }

            ThumbInstruction::LoadStoreSignExtended {
                h,
                sign,
                ro,
                rb,
                rd,
            } => {
                let address = self.regs.r[rb as usize].wrapping_add(self.regs.r[ro as usize]);
                let value = if h {
                    // Halfword
                    let val = bus.read_halfword(address & !1);
                    if sign && (val & 0x8000) != 0 {
                        val as u32 | 0xFFFF0000
                    } else {
                        val as u32
                    }
                } else {
                    // Byte
                    let val = bus.read_byte(address);
                    if sign && (val & 0x80) != 0 {
                        val as u32 | 0xFFFFFF00
                    } else {
                        val as u32
                    }
                };
                self.regs.r[rd as usize] = value;
                3
            }

            ThumbInstruction::Undefined => {
                // Istruzione non riconosciuta
                1
            }
        }
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

    #[test]
    fn test_mov_instruction() {
        // Test MOV R0, #42 con condition AL (sempre)
        // Formato: cond 00 I opcode S rn rd operand2
        // 1110 00 1 1101 0 0000 0000 000000101010
        // E3A0002A in hex

        struct TestBus {
            instructions: Vec<u32>,
            pc: usize,
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
            pc: 0,
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
        assert_eq!(cpu.regs.flag_z(), false);
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
        assert_eq!(cpu.regs.flag_z(), false);
        assert_eq!(cpu.regs.flag_n(), false);
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
