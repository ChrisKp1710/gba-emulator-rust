// Audio Mixer - Combina tutti i canali

use super::channels::{SquareChannel, WaveChannel, NoiseChannel};
use super::direct_sound::DirectSound;
use super::registers::SoundRegisters;

/// Mixa tutti i 6 canali audio (4 GB + 2 Direct Sound)
/// Ritorna sample stereo (left, right) in formato i16
pub fn mix_audio(
    ch1: &mut SquareChannel,
    ch2: &mut SquareChannel,
    ch3: &mut WaveChannel,
    ch4: &mut NoiseChannel,
    dsa: &mut DirectSound,
    dsb: &mut DirectSound,
    regs: &SoundRegisters,
) -> (i16, i16) {
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    
    // === Mix canali GB (1-4) ===
    
    let (gb_vol_left, gb_vol_right) = regs.get_gb_volume();
    
    // Volume GB: 0=25%, 1=50%, 2=100%
    let gb_master_vol = match regs.soundcnt_h & 0x03 {
        0 => 1, // 25%
        1 => 2, // 50%
        _ => 4, // 100%
    };
    
    // Channel 1
    if ch1.is_enabled() {
        let sample = ch1.get_sample() as i32;
        let (en_left, en_right) = regs.is_gb_channel_enabled(0);
        
        if en_left {
            left += sample * gb_vol_left as i32 * gb_master_vol;
        }
        if en_right {
            right += sample * gb_vol_right as i32 * gb_master_vol;
        }
    }
    
    // Channel 2
    if ch2.is_enabled() {
        let sample = ch2.get_sample() as i32;
        let (en_left, en_right) = regs.is_gb_channel_enabled(1);
        
        if en_left {
            left += sample * gb_vol_left as i32 * gb_master_vol;
        }
        if en_right {
            right += sample * gb_vol_right as i32 * gb_master_vol;
        }
    }
    
    // Channel 3
    if ch3.is_enabled() {
        let sample = ch3.get_sample() as i32;
        let (en_left, en_right) = regs.is_gb_channel_enabled(2);
        
        if en_left {
            left += sample * gb_vol_left as i32 * gb_master_vol;
        }
        if en_right {
            right += sample * gb_vol_right as i32 * gb_master_vol;
        }
    }
    
    // Channel 4
    if ch4.is_enabled() {
        let sample = ch4.get_sample() as i32;
        let (en_left, en_right) = regs.is_gb_channel_enabled(3);
        
        if en_left {
            left += sample * gb_vol_left as i32 * gb_master_vol;
        }
        if en_right {
            right += sample * gb_vol_right as i32 * gb_master_vol;
        }
    }
    
    // === Mix Direct Sound A ===
    
    let dsa_sample = dsa.read_sample() as i32;
    let dsa_vol = if (regs.soundcnt_h >> 2) & 1 != 0 { 4 } else { 2 }; // 100% o 50%
    
    if (regs.soundcnt_h >> 9) & 1 != 0 { // Left enable
        left += dsa_sample * dsa_vol * 8; // Boost Direct Sound
    }
    if (regs.soundcnt_h >> 8) & 1 != 0 { // Right enable
        right += dsa_sample * dsa_vol * 8;
    }
    
    // === Mix Direct Sound B ===
    
    let dsb_sample = dsb.read_sample() as i32;
    let dsb_vol = if (regs.soundcnt_h >> 3) & 1 != 0 { 4 } else { 2 };
    
    if (regs.soundcnt_h >> 13) & 1 != 0 { // Left enable
        left += dsb_sample * dsb_vol * 8;
    }
    if (regs.soundcnt_h >> 12) & 1 != 0 { // Right enable
        right += dsb_sample * dsb_vol * 8;
    }
    
    // === Clamp e converti a i16 ===
    
    let left_final = left.clamp(-32768, 32767) as i16;
    let right_final = right.clamp(-32768, 32767) as i16;
    
    (left_final, right_final)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mixer_silence() {
        let mut ch1 = SquareChannel::new(true);
        let mut ch2 = SquareChannel::new(false);
        let mut ch3 = WaveChannel::new();
        let mut ch4 = NoiseChannel::new();
        let mut dsa = DirectSound::new();
        let mut dsb = DirectSound::new();
        let regs = SoundRegisters::new();
        
        // Tutti i canali disabilitati
        let (left, right) = mix_audio(&mut ch1, &mut ch2, &mut ch3, &mut ch4, &mut dsa, &mut dsb, &regs);
        
        assert_eq!(left, 0);
        assert_eq!(right, 0);
    }
}
