/// Controller input (KEYINPUT register 0x04000130)
/// 
/// Bit 0: A button
/// Bit 1: B button  
/// Bit 2: Select
/// Bit 3: Start
/// Bit 4: Right
/// Bit 5: Left
/// Bit 6: Up
/// Bit 7: Down
/// Bit 8: R button
/// Bit 9: L button
/// 
/// Nota: I bit sono INVERTITI (0 = premuto, 1 = rilasciato)
pub struct InputController {
    /// Stato corrente dei pulsanti (bit invertiti)
    keyinput: u16,
}

impl InputController {
    pub fn new() -> Self {
        Self {
            keyinput: 0x03FF, // Tutti i pulsanti rilasciati (bit a 1)
        }
    }
    
    /// Leggi registro KEYINPUT
    pub fn read_keyinput(&self) -> u16 {
        self.keyinput
    }
    
    /// Imposta stato pulsante A
    pub fn set_button_a(&mut self, pressed: bool) {
        if pressed {
            self.keyinput &= !(1 << 0);
        } else {
            self.keyinput |= 1 << 0;
        }
    }
    
    /// Imposta stato pulsante B
    pub fn set_button_b(&mut self, pressed: bool) {
        if pressed {
            self.keyinput &= !(1 << 1);
        } else {
            self.keyinput |= 1 << 1;
        }
    }
    
    /// Imposta stato pulsante Select
    pub fn set_button_select(&mut self, pressed: bool) {
        if pressed {
            self.keyinput &= !(1 << 2);
        } else {
            self.keyinput |= 1 << 2;
        }
    }
    
    /// Imposta stato pulsante Start
    pub fn set_button_start(&mut self, pressed: bool) {
        if pressed {
            self.keyinput &= !(1 << 3);
        } else {
            self.keyinput |= 1 << 3;
        }
    }
    
    /// Imposta stato D-Pad Right
    pub fn set_dpad_right(&mut self, pressed: bool) {
        if pressed {
            self.keyinput &= !(1 << 4);
        } else {
            self.keyinput |= 1 << 4;
        }
    }
    
    /// Imposta stato D-Pad Left
    pub fn set_dpad_left(&mut self, pressed: bool) {
        if pressed {
            self.keyinput &= !(1 << 5);
        } else {
            self.keyinput |= 1 << 5;
        }
    }
    
    /// Imposta stato D-Pad Up
    pub fn set_dpad_up(&mut self, pressed: bool) {
        if pressed {
            self.keyinput &= !(1 << 6);
        } else {
            self.keyinput |= 1 << 6;
        }
    }
    
    /// Imposta stato D-Pad Down
    pub fn set_dpad_down(&mut self, pressed: bool) {
        if pressed {
            self.keyinput &= !(1 << 7);
        } else {
            self.keyinput |= 1 << 7;
        }
    }
    
    /// Imposta stato pulsante R
    pub fn set_button_r(&mut self, pressed: bool) {
        if pressed {
            self.keyinput &= !(1 << 8);
        } else {
            self.keyinput |= 1 << 8;
        }
    }
    
    /// Imposta stato pulsante L
    pub fn set_button_l(&mut self, pressed: bool) {
        if pressed {
            self.keyinput &= !(1 << 9);
        } else {
            self.keyinput |= 1 << 9;
        }
    }
}

impl Default for InputController {
    fn default() -> Self {
        Self::new()
    }
}
