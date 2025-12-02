// Module per le istruzioni THUMB
// Sarà implementato in dettaglio successivamente

/// Decodifica un'istruzione THUMB
pub fn decode_thumb(instruction: u16) -> ThumbInstruction {
    // Placeholder - implementazione completa dopo
    ThumbInstruction::Unknown(instruction)
}

#[derive(Debug, Clone, Copy)]
pub enum ThumbInstruction {
    Unknown(u16),
    // TODO: Aggiungi tutti i tipi di istruzioni THUMB
}
