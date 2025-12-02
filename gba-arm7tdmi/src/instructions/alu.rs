// Implementazione istruzioni ALU (Arithmetic Logic Unit)
//
// Queste sono le istruzioni di base per operazioni matematiche e logiche:
// - ADD, SUB: Addizione e sottrazione
// - AND, OR, EOR: Operazioni logiche
// - MOV, MVN: Spostamento dati
// - CMP, TST: Confronti e test (solo flag, no write)

use crate::arm::data_processing;
use crate::registers::Registers;

/// Esegue un'istruzione Data Processing (ALU)
///
/// # Arguments
/// * `regs` - Registri CPU
/// * `opcode` - Tipo operazione (AND, EOR, SUB, etc.)
/// * `rd` - Registro destinazione
/// * `rn` - Primo operando (registro)
/// * `operand2` - Secondo operando (già calcolato con eventuali shift)
/// * `set_flags` - Se true, aggiorna i flag NZCV
/// * `carry` - Carry da barrel shifter per operazioni logiche
///
/// # Returns
/// Numero di cicli usati (sempre 1 per ALU base)
pub fn execute_data_processing(
    regs: &mut Registers,
    opcode: u8,
    rd: u8,
    rn: u8,
    operand2: u32,
    set_flags: bool,
    carry: bool,
) -> u32 {
    let rn_value = if rn == 15 {
        regs.pc() + 8 // PC è +8 quando usato come operando
    } else {
        regs.r[rn as usize]
    };

    let (result, new_carry, new_overflow) = match opcode {
        // AND: Rd = Rn AND Op2
        data_processing::AND => {
            let res = rn_value & operand2;
            (Some(res), carry, false)
        }

        // EOR: Rd = Rn XOR Op2
        data_processing::EOR => {
            let res = rn_value ^ operand2;
            (Some(res), carry, false)
        }

        // SUB: Rd = Rn - Op2
        data_processing::SUB => {
            let (res, overflow) = sub_with_flags(rn_value, operand2, false);
            (Some(res), rn_value >= operand2, overflow)
        }

        // RSB: Rd = Op2 - Rn
        data_processing::RSB => {
            let (res, overflow) = sub_with_flags(operand2, rn_value, false);
            (Some(res), operand2 >= rn_value, overflow)
        }

        // ADD: Rd = Rn + Op2
        data_processing::ADD => {
            let (res, overflow) = add_with_flags(rn_value, operand2, false);
            (
                Some(res),
                (res as u64) != ((rn_value as u64) + (operand2 as u64)),
                overflow,
            )
        }

        // ADC: Rd = Rn + Op2 + Carry
        data_processing::ADC => {
            let c = if regs.flag_c() { 1 } else { 0 };
            let (res, overflow) = add_with_flags(rn_value, operand2, regs.flag_c());
            let carry_out = ((rn_value as u64) + (operand2 as u64) + c) > 0xFFFF_FFFF;
            (Some(res), carry_out, overflow)
        }

        // SBC: Rd = Rn - Op2 + Carry - 1
        data_processing::SBC => {
            let c = if regs.flag_c() { 0 } else { 1 };
            let (res, overflow) = sub_with_flags(rn_value, operand2, !regs.flag_c());
            let carry_out = (rn_value as u64) >= ((operand2 as u64) + c);
            (Some(res), carry_out, overflow)
        }

        // RSC: Rd = Op2 - Rn + Carry - 1
        data_processing::RSC => {
            let c = if regs.flag_c() { 0 } else { 1 };
            let (res, overflow) = sub_with_flags(operand2, rn_value, !regs.flag_c());
            let carry_out = (operand2 as u64) >= ((rn_value as u64) + c);
            (Some(res), carry_out, overflow)
        }

        // TST: Flags = Rn AND Op2 (no write)
        data_processing::TST => {
            let res = rn_value & operand2;
            if set_flags {
                update_logic_flags(regs, res, carry);
            }
            (None, carry, false)
        }

        // TEQ: Flags = Rn XOR Op2 (no write)
        data_processing::TEQ => {
            let res = rn_value ^ operand2;
            if set_flags {
                update_logic_flags(regs, res, carry);
            }
            (None, carry, false)
        }

        // CMP: Flags = Rn - Op2 (no write)
        data_processing::CMP => {
            let (res, overflow) = sub_with_flags(rn_value, operand2, false);
            if set_flags {
                update_arithmetic_flags(regs, res, rn_value >= operand2, overflow);
            }
            (None, rn_value >= operand2, overflow)
        }

        // CMN: Flags = Rn + Op2 (no write)
        data_processing::CMN => {
            let (res, overflow) = add_with_flags(rn_value, operand2, false);
            if set_flags {
                let carry_out = ((rn_value as u64) + (operand2 as u64)) > 0xFFFF_FFFF;
                update_arithmetic_flags(regs, res, carry_out, overflow);
            }
            (None, false, overflow)
        }

        // ORR: Rd = Rn OR Op2
        data_processing::ORR => {
            let res = rn_value | operand2;
            (Some(res), carry, false)
        }

        // MOV: Rd = Op2
        data_processing::MOV => (Some(operand2), carry, false),

        // BIC: Rd = Rn AND NOT Op2
        data_processing::BIC => {
            let res = rn_value & !operand2;
            (Some(res), carry, false)
        }

        // MVN: Rd = NOT Op2
        data_processing::MVN => {
            let res = !operand2;
            (Some(res), carry, false)
        }

        _ => (None, false, false),
    };

    // Scrivi risultato nel registro destinazione (se presente)
    if let Some(value) = result {
        if rd == 15 {
            // Scrittura in PC
            regs.set_pc(value & !3); // Allinea a 4 byte
        } else {
            regs.r[rd as usize] = value;
        }

        // Aggiorna flag se richiesto
        if set_flags {
            if is_logic_operation(opcode) {
                update_logic_flags(regs, value, new_carry);
            } else {
                update_arithmetic_flags(regs, value, new_carry, new_overflow);
            }
        }
    }

    1 // ALU operations sempre 1 ciclo
}

/// Addizione con rilevamento overflow
fn add_with_flags(a: u32, b: u32, carry: bool) -> (u32, bool) {
    let c = if carry { 1 } else { 0 };
    let result = a.wrapping_add(b).wrapping_add(c);

    // Overflow: segni uguali ma risultato con segno diverso
    let overflow = ((a ^ result) & (b ^ result) & 0x8000_0000) != 0;

    (result, overflow)
}

/// Sottrazione con rilevamento overflow
fn sub_with_flags(a: u32, b: u32, carry: bool) -> (u32, bool) {
    let c = if carry { 0 } else { 1 };
    let result = a.wrapping_sub(b).wrapping_sub(c);

    // Overflow: segni diversi e risultato con segno diverso da 'a'
    let overflow = ((a ^ b) & (a ^ result) & 0x8000_0000) != 0;

    (result, overflow)
}

/// Verifica se l'operazione è logica (usa carry da shifter)
fn is_logic_operation(opcode: u8) -> bool {
    matches!(
        opcode,
        data_processing::AND
            | data_processing::EOR
            | data_processing::ORR
            | data_processing::MOV
            | data_processing::BIC
            | data_processing::MVN
    )
}

/// Aggiorna flag per operazioni logiche (AND, OR, EOR, MOV, etc.)
fn update_logic_flags(regs: &mut Registers, result: u32, carry: bool) {
    regs.set_flag_n((result & 0x8000_0000) != 0);
    regs.set_flag_z(result == 0);
    regs.set_flag_c(carry);
    // V non viene toccato dalle operazioni logiche
}

/// Aggiorna flag per operazioni aritmetiche (ADD, SUB, etc.)
fn update_arithmetic_flags(regs: &mut Registers, result: u32, carry: bool, overflow: bool) {
    regs.set_flag_n((result & 0x8000_0000) != 0);
    regs.set_flag_z(result == 0);
    regs.set_flag_c(carry);
    regs.set_flag_v(overflow);
}

/// Decodifica e calcola Operand2 con barrel shifter
///
/// Operand2 può essere:
/// - Immediate: valore immediato ruotato
/// - Register: registro con shift opzionale
///
/// # Returns
/// (valore, carry_out)
pub fn decode_operand2(operand2: u32, immediate: bool, regs: &Registers) -> (u32, bool) {
    if immediate {
        // Immediate: [11:8]=rotate, [7:0]=imm
        let imm = operand2 & 0xFF;
        let rotate = ((operand2 >> 8) & 0xF) * 2;
        let value = imm.rotate_right(rotate);
        let carry = if rotate == 0 {
            regs.flag_c()
        } else {
            (value & 0x8000_0000) != 0
        };
        (value, carry)
    } else {
        // Register: [11:4]=shift, [3:0]=Rm
        let rm = (operand2 & 0xF) as u8;
        let shift_type = (operand2 >> 5) & 0x3;
        let shift_amount = if (operand2 & (1 << 4)) != 0 {
            // Shift by register
            let rs = ((operand2 >> 8) & 0xF) as u8;
            regs.r[rs as usize] & 0xFF
        } else {
            // Shift by immediate
            (operand2 >> 7) & 0x1F
        };

        let rm_value = regs.r[rm as usize];
        barrel_shift(rm_value, shift_type, shift_amount, regs.flag_c())
    }
}

/// Barrel shifter (shift/rotate con carry out)
fn barrel_shift(value: u32, shift_type: u32, amount: u32, carry_in: bool) -> (u32, bool) {
    if amount == 0 {
        return (value, carry_in);
    }

    match shift_type {
        0 => {
            // LSL (Logical Shift Left)
            let result = value << amount;
            let carry = if amount <= 32 {
                (value & (1 << (32 - amount))) != 0
            } else {
                false
            };
            (result, carry)
        }
        1 => {
            // LSR (Logical Shift Right)
            let result = value >> amount;
            let carry = (value & (1 << (amount - 1))) != 0;
            (result, carry)
        }
        2 => {
            // ASR (Arithmetic Shift Right)
            let result = ((value as i32) >> amount) as u32;
            let carry = (value & (1 << (amount - 1))) != 0;
            (result, carry)
        }
        3 => {
            // ROR (Rotate Right)
            let result = value.rotate_right(amount);
            let carry = (value & (1 << (amount - 1))) != 0;
            (result, carry)
        }
        _ => (value, carry_in),
    }
}
