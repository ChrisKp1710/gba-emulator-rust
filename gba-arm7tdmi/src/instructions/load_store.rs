// Implementazione istruzioni Load/Store
//
// Queste istruzioni trasferiscono dati tra registri e memoria:
// - LDR: Load Register (memoria → registro)
// - STR: Store Register (registro → memoria)
// - LDM: Load Multiple (memoria → più registri)
// - STM: Store Multiple (più registri → memoria)

use crate::{cpu::MemoryBus, registers::Registers};

/// Parametri per Single Data Transfer (LDR/STR)
pub struct SingleDataTransferParams {
    pub load: bool,
    pub byte: bool,
    pub pre_index: bool,
    pub add: bool,
    pub writeback: bool,
    pub rn: u8,
    pub rd: u8,
    pub offset: u32,
}

/// Esegue Single Data Transfer (LDR/STR)
///
/// # Arguments
/// * `regs` - Registri CPU
/// * `bus` - Bus memoria per accesso
/// * `params` - Parametri dell'istruzione
///
/// # Returns
/// Numero di cicli usati
pub fn execute_single_data_transfer<M: MemoryBus>(
    regs: &mut Registers,
    bus: &mut M,
    params: &SingleDataTransferParams,
) -> u32 {
    let base = regs.r[params.rn as usize];

    // Calcola offset (può essere signed)
    let offset_val = if params.add {
        params.offset as i32
    } else {
        -(params.offset as i32)
    };

    // Calcola indirizzo
    let address = if params.pre_index {
        // Pre-indexed: usa (base + offset)
        (base as i32).wrapping_add(offset_val) as u32
    } else {
        // Post-indexed: usa base, poi applica offset
        base
    };

    // Esegui load o store
    if params.load {
        // LDR: carica da memoria
        let value = if params.byte {
            bus.read_byte(address) as u32
        } else {
            bus.read_word(address & !3) // Word allineato
        };

        if params.rd == 15 {
            // Load in PC
            regs.set_pc(value & !3);
        } else {
            regs.r[params.rd as usize] = value;
        }
    } else {
        // STR: salva in memoria
        let value = if params.rd == 15 {
            regs.pc() + 12 // PC+12 quando STR usa R15
        } else {
            regs.r[params.rd as usize]
        };

        if params.byte {
            bus.write_byte(address, value as u8);
        } else {
            bus.write_word(address & !3, value); // Word allineato
        }
    }

    // Writeback: aggiorna registro base
    if params.writeback || !params.pre_index {
        let final_address = (base as i32).wrapping_add(offset_val) as u32;
        if params.rn != 15 {
            regs.r[params.rn as usize] = final_address;
        }
    }

    // Cicli: 1S + 1N + 1I (load) o 2N (store)
    if params.load {
        3
    } else {
        2
    }
}

/// Parametri per Block Data Transfer (LDM/STM)
pub struct BlockDataTransferParams {
    pub load: bool,
    pub pre_index: bool,
    pub add: bool,
    pub writeback: bool,
    pub rn: u8,
    pub register_list: u16,
}

/// Esegue Block Data Transfer (LDM/STM)
///
/// Carica o salva multipli registri in un'operazione.
///
/// # Arguments
/// * `regs` - Registri CPU
/// * `bus` - Bus memoria
/// * `params` - Parametri dell'istruzione
///
/// # Returns
/// Numero di cicli usati
pub fn execute_block_data_transfer<M: MemoryBus>(
    regs: &mut Registers,
    bus: &mut M,
    params: &BlockDataTransferParams,
) -> u32 {
    let mut address = regs.r[params.rn as usize];
    let count = params.register_list.count_ones();

    // Calcola indirizzo iniziale per decremento
    if !params.add {
        address = address.wrapping_sub(count * 4);
    }

    let mut cycles = 0;

    // Trasferisci ogni registro nella lista
    for i in 0..16 {
        if (params.register_list & (1 << i)) != 0 {
            // Pre-increment se richiesto
            if params.pre_index {
                address = if params.add {
                    address.wrapping_add(4)
                } else {
                    address.wrapping_sub(4)
                };
            }

            // Esegui load/store
            if params.load {
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
            if !params.pre_index {
                address = if params.add {
                    address.wrapping_add(4)
                } else {
                    address.wrapping_sub(4)
                };
            }

            cycles += 1;
        }
    }

    // Writeback
    if params.writeback {
        let final_address = if params.add {
            regs.r[params.rn as usize].wrapping_add(count * 4)
        } else {
            regs.r[params.rn as usize].wrapping_sub(count * 4)
        };
        regs.r[params.rn as usize] = final_address;
    }

    // Cicli: nS + 1N + 1I (LDM) o (n-1)S + 2N (STM)
    if params.load {
        cycles + 2
    } else {
        cycles + 1
    }
}
