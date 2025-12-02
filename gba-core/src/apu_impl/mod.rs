// APU - Audio Processing Unit
//
// Struttura modulare per gestire l'audio del GBA:
// - channels/: I 4 canali GB (square1, square2, wave, noise)
// - direct_sound.rs: Direct Sound A/B (DMA audio)
// - mixer.rs: Mixing dei 6 canali
// - registers.rs: Registri audio (SOUNDCNT_L/H/X, SOUNDBIAS)

mod channels;
mod direct_sound;
mod mixer;
mod registers;

pub use registers::SoundRegisters;
use channels::{SquareChannel, WaveChannel, NoiseChannel};
use direct_sound::DirectSound;

/// GBA Audio Processing Unit
#[derive(Debug)]
pub struct APU {
    /// Registri audio condivisi
    registers: SoundRegisters,
    
    /// Channel 1: Square Wave con Sweep
    channel1: SquareChannel,
    
    /// Channel 2: Square Wave
    channel2: SquareChannel,
    
    /// Channel 3: Wave Output
    channel3: WaveChannel,
    
    /// Channel 4: Noise
    channel4: NoiseChannel,
    
    /// Direct Sound A
    direct_sound_a: DirectSound,
    
    /// Direct Sound B
    direct_sound_b: DirectSound,
    
    /// Frame counter per timing
    frame_counter: u64,
}

impl APU {
    /// Crea una nuova APU
    pub fn new() -> Self {
        Self {
            registers: SoundRegisters::new(),
            channel1: SquareChannel::new(true), // Con sweep
            channel2: SquareChannel::new(false), // Senza sweep
            channel3: WaveChannel::new(),
            channel4: NoiseChannel::new(),
            direct_sound_a: DirectSound::new(),
            direct_sound_b: DirectSound::new(),
            frame_counter: 0,
        }
    }
    
    /// Legge un byte da un registro audio
    pub fn read_byte(&self, addr: u32) -> u8 {
        match addr {
            // Channel 1
            0x04000060..=0x04000065 => self.channel1.read_byte(addr),
            
            // Channel 2
            0x04000068..=0x0400006D => self.channel2.read_byte(addr),
            
            // Channel 3
            0x04000070..=0x04000075 => self.channel3.read_byte(addr),
            
            // Wave RAM
            0x04000090..=0x0400009F => self.channel3.read_wave_ram(addr),
            
            // Channel 4
            0x04000078..=0x0400007D => self.channel4.read_byte(addr),
            
            // Control registers
            0x04000080..=0x04000089 => self.registers.read_byte(addr),
            
            _ => 0,
        }
    }
    
    /// Scrive un byte in un registro audio
    pub fn write_byte(&mut self, addr: u32, value: u8) {
        match addr {
            // Channel 1
            0x04000060..=0x04000065 => self.channel1.write_byte(addr, value),
            
            // Channel 2
            0x04000068..=0x0400006D => self.channel2.write_byte(addr, value),
            
            // Channel 3
            0x04000070..=0x04000075 => self.channel3.write_byte(addr, value),
            
            // Wave RAM
            0x04000090..=0x0400009F => self.channel3.write_wave_ram(addr, value),
            
            // Channel 4
            0x04000078..=0x0400007D => self.channel4.write_byte(addr, value),
            
            // Control registers
            0x04000080..=0x04000089 => {
                self.registers.write_byte(addr, value);
                
                // Reset FIFO se richiesto
                if addr == 0x04000083 {
                    if value & 0x08 != 0 {
                        self.direct_sound_a.reset_fifo();
                    }
                    if value & 0x80 != 0 {
                        self.direct_sound_b.reset_fifo();
                    }
                }
            }
            
            _ => {}
        }
    }
    
    /// Legge una halfword
    pub fn read_halfword(&self, addr: u32) -> u16 {
        let low = self.read_byte(addr) as u16;
        let high = self.read_byte(addr + 1) as u16;
        (high << 8) | low
    }
    
    /// Scrive una halfword
    pub fn write_halfword(&mut self, addr: u32, value: u16) {
        self.write_byte(addr, value as u8);
        self.write_byte(addr + 1, (value >> 8) as u8);
    }
    
    /// Scrive nel FIFO A (Direct Sound A)
    pub fn write_fifo_a(&mut self, value: i8) {
        self.direct_sound_a.write_sample(value);
    }
    
    /// Scrive nel FIFO B (Direct Sound B)
    pub fn write_fifo_b(&mut self, value: i8) {
        self.direct_sound_b.write_sample(value);
    }
    
    /// Genera un sample audio stereo (left, right)
    /// Chiamato a 32768 Hz (sample rate default)
    pub fn generate_sample(&mut self) -> (i16, i16) {
        if !self.registers.is_master_enabled() {
            return (0, 0);
        }
        
        // Mix tutti i canali
        mixer::mix_audio(
            &mut self.channel1,
            &mut self.channel2,
            &mut self.channel3,
            &mut self.channel4,
            &mut self.direct_sound_a,
            &mut self.direct_sound_b,
            &self.registers,
        )
    }
    
    /// Avanza l'APU di un ciclo
    pub fn step(&mut self) {
        self.frame_counter += 1;
        
        // Step sui canali se abilitati
        if self.registers.is_master_enabled() {
            self.channel1.step();
            self.channel2.step();
            self.channel3.step();
            self.channel4.step();
        }
    }
}

impl Default for APU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_apu_creation() {
        let apu = APU::new();
        assert!(!apu.registers.is_master_enabled());
    }
    
    #[test]
    fn test_master_enable() {
        let mut apu = APU::new();
        
        // Abilita master
        apu.write_byte(0x04000084, 0x80);
        assert!(apu.registers.is_master_enabled());
        
        // Sample dovrebbe essere 0 se master disabilitato
        apu.write_byte(0x04000084, 0x00);
        let (left, right) = apu.generate_sample();
        assert_eq!(left, 0);
        assert_eq!(right, 0);
    }
    
    #[test]
    fn test_register_routing() {
        let mut apu = APU::new();
        
        // Test routing a channel 1
        apu.write_halfword(0x04000062, 0xF800);
        assert_eq!(apu.read_halfword(0x04000062), 0xF800);
        
        // Test routing a control registers
        apu.write_halfword(0x04000080, 0x1234);
        assert_eq!(apu.read_halfword(0x04000080), 0x1234);
    }
}
