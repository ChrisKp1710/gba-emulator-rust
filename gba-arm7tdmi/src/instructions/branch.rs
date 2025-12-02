// Implementazione istruzioni di Branch (salto)
//
// Le istruzioni di branch modificano il Program Counter (PC) per saltare
// a un'altra posizione nel codice.
//
// Tipi:
// - B: Branch semplice (salta)
// - BL: Branch with Link (salta e salva indirizzo ritorno in R14/LR)
// - BX: Branch and Exchange (salta e switch ARM/THUMB)

use crate::registers::Registers;

/// Esegue un Branch (B o BL)
///
/// # Arguments
/// * `regs` - Registri CPU
/// * `offset` - Offset signed (già moltiplicato per 4) da aggiungere al PC
/// * `link` - Se true, salva PC in R14 (LR) prima del salto
///
/// # Returns
/// Numero di cicli usati (2S+1N = 3 cicli)
pub fn execute_branch(regs: &mut Registers, offset: i32, link: bool) -> u32 {
    let pc = regs.pc();
    
    // Se BL, salva indirizzo ritorno in LR (R14)
    if link {
        regs.r[14] = pc.wrapping_sub(4); // PC-4 = istruzione dopo BL
    }
    
    // Calcola nuovo PC: PC corrente è già +8 (prefetch)
    // quindi sommiamo l'offset a PC che è già avanzato
    let new_pc = (pc as i32).wrapping_add(offset) as u32;
    regs.set_pc(new_pc & !3); // Allinea a 4 byte (ARM mode)
    
    3 // Branch costa 2S+1N = 3 cicli
}/// Esegue un Branch and Exchange (BX)
///
/// Salta all'indirizzo in Rn e switch tra ARM/THUMB mode
/// in base al bit 0 di Rn:
/// - Bit 0 = 0: ARM mode
/// - Bit 0 = 1: THUMB mode
///
/// # Arguments
/// * `regs` - Registri CPU
/// * `rn` - Registro contenente l'indirizzo target
///
/// # Returns
/// Numero di cicli usati (2S+1N = 3 cicli)
pub fn execute_branch_exchange(regs: &mut Registers, rn: u8) -> u32 {
    let target = regs.r[rn as usize];

    // Bit 0 determina ARM (0) o THUMB (1) mode
    let thumb_mode = (target & 1) != 0;

    // Imposta nuovo PC (allineato)
    if thumb_mode {
        regs.set_pc(target & !1); // THUMB: allinea a 2 byte
        regs.set_thumb(true);
    } else {
        regs.set_pc(target & !3); // ARM: allinea a 4 byte
        regs.set_thumb(false);
    }

    3 // BX costa 2S+1N = 3 cicli
}
