use bitflags::bitflags;

bitflags! {
    /// Registro Interrupt Enable (IE)
    pub struct InterruptFlags: u16 {
        const VBLANK  = 1 << 0;
        const HBLANK  = 1 << 1;
        const VCOUNT  = 1 << 2;
        const TIMER0  = 1 << 3;
        const TIMER1  = 1 << 4;
        const TIMER2  = 1 << 5;
        const TIMER3  = 1 << 6;
        const SERIAL  = 1 << 7;
        const DMA0    = 1 << 8;
        const DMA1    = 1 << 9;
        const DMA2    = 1 << 10;
        const DMA3    = 1 << 11;
        const KEYPAD  = 1 << 12;
        const GAMEPAK = 1 << 13;
    }
}

pub struct InterruptController {
    /// Interrupt Enable
    pub ie: u16,
    
    /// Interrupt Flags
    pub if_: u16,
    
    /// Interrupt Master Enable
    pub ime: bool,
}

impl InterruptController {
    pub fn new() -> Self {
        Self {
            ie: 0,
            if_: 0,
            ime: false,
        }
    }
    
    /// Richiedi un interrupt
    pub fn request(&mut self, flag: InterruptFlags) {
        self.if_ |= flag.bits();
    }
    
    /// Verifica se c'è un interrupt pendente
    pub fn pending(&self) -> bool {
        self.ime && (self.ie & self.if_) != 0
    }
    
    /// Acknowledgeun interrupt
    pub fn acknowledge(&mut self, flag: InterruptFlags) {
        self.if_ &= !flag.bits();
    }
}

impl Default for InterruptController {
    fn default() -> Self {
        Self::new()
    }
}
