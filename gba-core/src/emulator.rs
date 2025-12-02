use crate::bus::Bus;
use crate::cartridge::Cartridge;
use gba_arm7tdmi::ARM7TDMI;

//==============================================================================
// EMULATORE GBA - COMPONENTE PRINCIPALE
//==============================================================================
// Questo è il "cervello" che coordina tutti i componenti:
// - CPU (ARM7TDMI)
// - Bus di sistema (memoria, I/O)
// - PPU (grafica)
// - APU (audio)
// - Timer, DMA, Interrupt
//
// COME FUNZIONA UN FRAME:
// 1. CPU esegue istruzioni fino a raggiungere ~280,896 cicli (1/60 sec)
// 2. Ogni ciclo CPU, il PPU avanza il rendering
// 3. Alla fine di ogni scanline (linea orizzontale), possibile HBlank interrupt
// 4. Alla fine del frame (dopo 160 scanline), VBlank interrupt
// 5. Durante VBlank, il gioco aggiorna grafica e logica
// 6. Il framebuffer viene copiato sullo schermo
// 7. Ripeti per il prossimo frame
//
// ARCHITETTURA MODULARE:
// Ogni componente (CPU, PPU, etc.) è separato in moduli.
// Questo permette di:
// - Testare ogni parte singolarmente
// - Modificare un componente senza toccare gli altri
// - Aggiungere features gradualmente
// - Riutilizzare codice in altri progetti
//==============================================================================

/// Emulatore GBA principale
///
/// Coordina CPU, memoria, grafica e tutti i componenti del sistema
pub struct GbaEmulator {
    pub cpu: ARM7TDMI,
    pub bus: Bus,
}

impl GbaEmulator {
    pub fn new() -> Self {
        Self {
            cpu: ARM7TDMI::new(),
            bus: Bus::new(),
        }
    }

    /// Carica un BIOS
    pub fn load_bios(&mut self, bios: Vec<u8>) {
        self.bus.load_bios(bios);
    }

    /// Carica una cartridge
    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        log::info!("Loading ROM: {}", cartridge.header.title);
        log::info!("Game Code: {}", cartridge.header.game_code);
        log::info!("Maker Code: {}", cartridge.header.maker_code);
        log::info!("Version: {}", cartridge.header.version);

        // Initialize save system with ROM data
        let rom_path = cartridge.rom_path.clone();
        self.bus.save.init_from_rom(&cartridge.rom, rom_path);

        // Log save type
        let save_type = self.bus.save.save_type();
        log::info!("Save Type: {:?}", save_type);
        if let Some(save_path) = self.bus.save.save_path() {
            log::info!("Save Path: {}", save_path.display());
        }

        self.bus.load_rom(cartridge.rom);
    }

    /// Reset dell'emulatore
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.cpu.regs.set_pc(0x0800_0000); // Salta alla ROM
    }

    /// Esegui un singolo frame
    pub fn run_frame(&mut self) {
        // GBA: 16.78 MHz CPU, ~280896 cicli per frame (60 FPS)
        const CYCLES_PER_FRAME: u32 = 280896;

        let mut frame_cycles = 0;

        while frame_cycles < CYCLES_PER_FRAME {
            let cycles = self.cpu.step(&mut self.bus);
            frame_cycles += cycles;

            // Step PPU con accesso alla VRAM
            let vram_ptr = self.bus.memory.vram.as_ptr();
            let vram_len = self.bus.memory.vram.len();
            unsafe {
                let vram_slice = std::slice::from_raw_parts(vram_ptr, vram_len);
                self.bus.ppu.step(cycles, vram_slice);
            }

            // Gestione interrupt VBlank
            if self.bus.ppu.in_vblank() && self.bus.ppu.scanline == 160 {
                self.bus
                    .interrupt
                    .request(crate::interrupt::InterruptFlags::VBLANK);
            }

            // Gestione interrupt CPU
            if self.bus.interrupt.pending() {
                self.cpu.request_interrupt();
            }
        }

        // Auto-save at end of frame if save is modified
        let _ = self.bus.save.auto_save();
    }

    /// Ottieni il framebuffer corrente
    pub fn framebuffer(&self) -> &[u16] {
        &self.bus.ppu.framebuffer
    }

    /// Ottieni riferimento mutabile all'input controller
    pub fn input_mut(&mut self) -> &mut crate::input::InputController {
        &mut self.bus.input
    }
}

impl Default for GbaEmulator {
    fn default() -> Self {
        Self::new()
    }
}
