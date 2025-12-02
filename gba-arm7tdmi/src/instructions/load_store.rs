// Implementazione istruzioni Load/Store
//
// Queste istruzioni trasferiscono dati tra registri e memoria:
// - LDR: Load Register (memoria → registro)
// - STR: Store Register (registro → memoria)
// - LDM: Load Multiple (memoria → più registri)
// - STM: Store Multiple (più registri → memoria)

use crate::{cpu::MemoryBus, registers::Registers};

/// Esegue Single Data Transfer (LDR/STR)
///
/// # Arguments
/// * `regs` - Registri CPU
/// * `bus` - Bus memoria per accesso
/// * `load` - true=LDR, false=STR
/// * `byte` - true=byte, false=word
/// * `pre_index` - Se true, applica offset prima dell'accesso
/// * `add` - Se true, somma offset; se false, sottrai
/// * `writeback` - Se true, scrivi indirizzo finale in Rn
/// * `rn` - Registro base
/// * `rd` - Registro source/dest
/// * `offset` - Offset da applicare
///
/// # Returns
/// Numero di cicli usati
pub fn execute_single_data_transfer<M: MemoryBus>(
    regs: &mut Registers,
    bus: &mut M,
    load: bool,
    byte: bool,
    pre_index: bool,
    add: bool,
    writeback: bool,
    rn: u8,
    rd: u8,
    offset: u32,
) -> u32 {
    let base = regs.r[rn as usize];

    // Calcola offset (può essere signed)
    let offset_val = if add { offset as i32 } else { -(offset as i32) };

    // Calcola indirizzo
    let address = if pre_index {
        // Pre-indexed: usa (base + offset)
        (base as i32).wrapping_add(offset_val) as u32
    } else {
        // Post-indexed: usa base, poi applica offset
        base
    };

    // Esegui load o store
    if load {
        // LDR: carica da memoria
        let value = if byte {
            bus.read_byte(address) as u32
        } else {
            bus.read_word(address & !3) // Word allineato
        };

        if rd == 15 {
            // Load in PC
            regs.set_pc(value & !3);
        } else {
            regs.r[rd as usize] = value;
        }
    } else {
        // STR: salva in memoria
        let value = if rd == 15 {
            regs.pc() + 12 // PC+12 quando STR usa R15
        } else {
            regs.r[rd as usize]
        };

        if byte {
            bus.write_byte(address, value as u8);
        } else {
            bus.write_word(address & !3, value); // Word allineato
        }
    }

    // Writeback: aggiorna registro base
    if writeback || !pre_index {
        let final_address = (base as i32).wrapping_add(offset_val) as u32;
        if rn != 15 {
            regs.r[rn as usize] = final_address;
        }
    }

    // Cicli: 1S + 1N + 1I (load) o 2N (store)
    if load {
        3
    } else {
        2
    }
}

/// Esegue Block Data Transfer (LDM/STM)
///
/// Carica o salva multipli registri in un'operazione.
///
/// # Arguments
/// * `regs` - Registri CPU
/// * `bus` - Bus memoria
/// * `load` - true=LDM, false=STM
/// * `pre_index` - Se true, incrementa prima dell'accesso
/// * `add` - Se true, incrementa; se false, decrementa
/// * `writeback` - Se true, aggiorna Rn con indirizzo finale
/// * `rn` - Registro base
/// * `register_list` - Bitmask registri da trasferire (bit 0=R0, bit 15=R15)
///
/// # Returns
/// Numero di cicli usati
pub fn execute_block_data_transfer<M: MemoryBus>(
    regs: &mut Registers,
    bus: &mut M,
    load: bool,
    pre_index: bool,
    add: bool,
    writeback: bool,
    rn: u8,
    register_list: u16,
) -> u32 {
    let mut address = regs.r[rn as usize];
    let count = register_list.count_ones();

    // Calcola indirizzo iniziale per decremento
    if !add {
        address = address.wrapping_sub(count * 4);
    }

    let mut cycles = 0;

    // Trasferisci ogni registro nella lista
    for i in 0..16 {
        if (register_list & (1 << i)) != 0 {
            // Pre-increment se richiesto
            if pre_index {
                address = if add {
                    address.wrapping_add(4)
                } else {
                    address.wrapping_sub(4)
                };
            }

            // Esegui load/store
            if load {
                let value = bus.read_word(address);
                if i == 15 {
                    regs.set_pc(value & !3);
                } else {
                    regs.r[i] = value;
                }
            } else {
                let value = if i == 15 { regs.pc() + 12 } else { regs.r[i] };
                bus.write_word(address, value);
            }

            // Post-increment se non pre
            if !pre_index {
                address = if add {
                    address.wrapping_add(4)
                } else {
                    address.wrapping_sub(4)
                };
            }

            cycles += 1;
        }
    }

    // Writeback
    if writeback {
        let final_address = if add {
            regs.r[rn as usize].wrapping_add(count * 4)
        } else {
            regs.r[rn as usize].wrapping_sub(count * 4)
        };
        regs.r[rn as usize] = final_address;
    }

    // Cicli: nS + 1N + 1I (LDM) o (n-1)S + 2N (STM)
    if load {
        cycles + 2
    } else {
        cycles + 1
    }
}
