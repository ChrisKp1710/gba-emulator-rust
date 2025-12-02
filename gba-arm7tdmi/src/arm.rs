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
}
