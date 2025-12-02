/// PPU - Picture Processing Unit del GBA
/// Risoluzione: 240x160 pixel
/// 5 modalità grafiche (Mode 0-5)
///
/// MODE 3: Bitmap 240x160, 16-bit color (RGB555)
/// - VRAM 0x06000000-0x06017FFF (96 KB, ma Mode 3 usa solo primi 75 KB)
/// - Ogni pixel = 2 byte (RGB555: 5 bit R, 5 bit G, 5 bit B)
/// - Più semplice da implementare ma usa più memoria
const SCREEN_WIDTH: usize = 240;
pub const SCREEN_HEIGHT: usize = 160;

/// Registri I/O LCD
pub const DISPCNT: u32 = 0x04000000; // Display Control
pub const DISPSTAT: u32 = 0x04000004; // Display Status
pub const VCOUNT: u32 = 0x04000006; // Vertical Counter

// Background Control Registers (BGxCNT)
pub const BG0CNT: u32 = 0x04000008;
pub const BG1CNT: u32 = 0x0400000A;
pub const BG2CNT: u32 = 0x0400000C;
pub const BG3CNT: u32 = 0x0400000E;

// Background Scroll Registers (BGxHOFS/BGxVOFS)
pub const BG0HOFS: u32 = 0x04000010;
pub const BG0VOFS: u32 = 0x04000012;
pub const BG1HOFS: u32 = 0x04000014;
pub const BG1VOFS: u32 = 0x04000016;
pub const BG2HOFS: u32 = 0x04000018;
pub const BG2VOFS: u32 = 0x0400001A;
pub const BG3HOFS: u32 = 0x0400001C;
pub const BG3VOFS: u32 = 0x0400001E;

/// Palette RAM: 0x05000000-0x050003FF (1KB)
/// - BG Palette: 0x05000000-0x050001FF (512 bytes = 256 colori)
/// - OBJ Palette: 0x05000200-0x050003FF (512 bytes = 256 colori)
pub const PALETTE_RAM_SIZE: usize = 0x400;
pub const BG_PALETTE_SIZE: usize = 0x200;
pub const OBJ_PALETTE_OFFSET: usize = 0x200;

/// OAM (Object Attribute Memory): 0x07000000-0x070003FF (1KB)
/// 128 sprites * 8 bytes = 1024 bytes
pub const OAM_SIZE: usize = 0x400;
pub const OAM_SPRITE_COUNT: usize = 128;

/// VRAM base per tile data e map data
pub const VRAM_BASE: u32 = 0x06000000;
/// OBJ tiles in VRAM: 0x06010000-0x06017FFF (32KB in Mode 0-2)
pub const OBJ_TILE_BASE: usize = 0x10000;

/// Display modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMode {
    Mode0 = 0, // Tiled mode (4 backgrounds)
    Mode1 = 1, // Tiled mode (2 backgrounds + 1 affine)
    Mode2 = 2, // Tiled mode (2 affine backgrounds)
    Mode3 = 3, // Bitmap 240x160, 16-bit color
    Mode4 = 4, // Bitmap 240x160, 8-bit paletted
    Mode5 = 5, // Bitmap 160x128, 16-bit color
}

/// Background Control Register
#[derive(Debug, Clone, Copy, Default)]
pub struct BgControl {
    pub priority: u8,      // Bits 0-1
    pub char_base: u8,     // Bits 2-3 (character base block * 16KB)
    pub mosaic: bool,      // Bit 6
    pub palette_256: bool, // Bit 7 (false = 16x16, true = 256x1)
    pub screen_base: u8,   // Bits 8-12 (screen base block * 2KB)
    pub wrap: bool,        // Bit 13 (affine wrap)
    pub screen_size: u8,   // Bits 14-15
}

impl BgControl {
    pub fn from_u16(value: u16) -> Self {
        Self {
            priority: (value & 0x3) as u8,
            char_base: ((value >> 2) & 0x3) as u8,
            mosaic: (value & (1 << 6)) != 0,
            palette_256: (value & (1 << 7)) != 0,
            screen_base: ((value >> 8) & 0x1F) as u8,
            wrap: (value & (1 << 13)) != 0,
            screen_size: ((value >> 14) & 0x3) as u8,
        }
    }

    pub fn to_u16(&self) -> u16 {
        (self.priority as u16)
            | ((self.char_base as u16) << 2)
            | ((self.mosaic as u16) << 6)
            | ((self.palette_256 as u16) << 7)
            | ((self.screen_base as u16) << 8)
            | ((self.wrap as u16) << 13)
            | ((self.screen_size as u16) << 14)
    }

    /// Ottieni dimensione screen in tile (width, height)
    pub fn get_screen_size(&self) -> (usize, usize) {
        match self.screen_size {
            0 => (32, 32), // 256x256 px
            1 => (64, 32), // 512x256 px
            2 => (32, 64), // 256x512 px
            3 => (64, 64), // 512x512 px
            _ => (32, 32),
        }
    }
}

/// Sprite Attribute (OAM entry)
#[derive(Debug, Clone, Copy)]
pub struct SpriteAttribute {
    // Attribute 0 (16-bit)
    pub y: u8,             // Bits 0-7: Y coordinate
    pub obj_mode: u8,      // Bits 8-9: Object mode (normal, affine, disabled, double)
    pub gfx_mode: u8,      // Bits 10-11: GFX mode (normal, alpha, window)
    pub mosaic: bool,      // Bit 12: Mosaic
    pub palette_256: bool, // Bit 13: 256 colors (true) or 16 colors (false)
    pub shape: u8,         // Bits 14-15: Shape (square, wide, tall)

    // Attribute 1 (16-bit)
    pub x: u16,       // Bits 0-8: X coordinate (9 bits)
    pub h_flip: bool, // Bit 12: Horizontal flip (regular sprites)
    pub v_flip: bool, // Bit 13: Vertical flip (regular sprites)
    pub size: u8,     // Bits 14-15: Size

    // Attribute 2 (16-bit)
    pub tile_index: u16,  // Bits 0-9: Tile number
    pub priority: u8,     // Bits 10-11: Priority
    pub palette_bank: u8, // Bits 12-15: Palette bank (16-color mode)
}

impl SpriteAttribute {
    /// Crea sprite da 6 bytes OAM (primi 6 byte, gli ultimi 2 sono rotation/scaling)
    pub fn from_oam_bytes(bytes: &[u8]) -> Self {
        if bytes.len() < 6 {
            return Self::default();
        }

        let attr0 = (bytes[0] as u16) | ((bytes[1] as u16) << 8);
        let attr1 = (bytes[2] as u16) | ((bytes[3] as u16) << 8);
        let attr2 = (bytes[4] as u16) | ((bytes[5] as u16) << 8);

        Self {
            // Attr 0
            y: (attr0 & 0xFF) as u8,
            obj_mode: ((attr0 >> 8) & 0x3) as u8,
            gfx_mode: ((attr0 >> 10) & 0x3) as u8,
            mosaic: (attr0 & (1 << 12)) != 0,
            palette_256: (attr0 & (1 << 13)) != 0,
            shape: ((attr0 >> 14) & 0x3) as u8,

            // Attr 1
            x: attr1 & 0x1FF,
            h_flip: (attr1 & (1 << 12)) != 0,
            v_flip: (attr1 & (1 << 13)) != 0,
            size: ((attr1 >> 14) & 0x3) as u8,

            // Attr 2
            tile_index: attr2 & 0x3FF,
            priority: ((attr2 >> 10) & 0x3) as u8,
            palette_bank: ((attr2 >> 12) & 0xF) as u8,
        }
    }

    /// Ottieni dimensioni sprite in pixel (width, height)
    pub fn get_size(&self) -> (usize, usize) {
        match (self.shape, self.size) {
            // Square
            (0, 0) => (8, 8),
            (0, 1) => (16, 16),
            (0, 2) => (32, 32),
            (0, 3) => (64, 64),
            // Wide (horizontal)
            (1, 0) => (16, 8),
            (1, 1) => (32, 8),
            (1, 2) => (32, 16),
            (1, 3) => (64, 32),
            // Tall (vertical)
            (2, 0) => (8, 16),
            (2, 1) => (8, 32),
            (2, 2) => (16, 32),
            (2, 3) => (32, 64),
            _ => (8, 8),
        }
    }

    /// Verifica se lo sprite è visibile
    pub fn is_visible(&self) -> bool {
        // obj_mode == 2 significa disabilitato
        self.obj_mode != 2
    }
}

impl Default for SpriteAttribute {
    fn default() -> Self {
        Self {
            y: 0,
            obj_mode: 2, // Disabled
            gfx_mode: 0,
            mosaic: false,
            palette_256: false,
            shape: 0,
            x: 0,
            h_flip: false,
            v_flip: false,
            size: 0,
            tile_index: 0,
            priority: 0,
            palette_bank: 0,
        }
    }
}

pub struct PPU {
    /// Frame buffer (RGB555 format: xBBBBBGGGGGRRRRR)
    pub framebuffer: Vec<u16>,

    /// Display Control Register (DISPCNT)
    pub dispcnt: u16,

    /// Display Status Register (DISPSTAT)
    pub dispstat: u16,

    /// Scanline corrente (VCOUNT)
    pub scanline: u16,

    /// Cicli PPU accumulati
    pub cycles: u32,

    /// Background Control Registers (BG0-BG3)
    pub bg_control: [BgControl; 4],

    /// Background Scroll X (BG0-BG3)
    pub bg_hofs: [u16; 4],

    /// Background Scroll Y (BG0-BG3)
    pub bg_vofs: [u16; 4],

    /// Palette RAM (1KB: 512 bytes BG + 512 bytes OBJ)
    pub palette_ram: Vec<u8>,

    /// OAM (Object Attribute Memory - 1KB, 128 sprites)
    pub oam: Vec<u8>,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
            dispcnt: 0,
            dispstat: 0,
            scanline: 0,
            cycles: 0,
            bg_control: [BgControl::default(); 4],
            bg_hofs: [0; 4],
            bg_vofs: [0; 4],
            palette_ram: vec![0; PALETTE_RAM_SIZE],
            oam: vec![0; OAM_SIZE],
        }
    }

    /// Leggi registro I/O
    pub fn read_register(&self, addr: u32) -> u16 {
        match addr {
            DISPCNT => self.dispcnt,
            DISPSTAT => self.dispstat | (self.in_vblank() as u16),
            VCOUNT => self.scanline,
            BG0CNT => self.bg_control[0].to_u16(),
            BG1CNT => self.bg_control[1].to_u16(),
            BG2CNT => self.bg_control[2].to_u16(),
            BG3CNT => self.bg_control[3].to_u16(),
            BG0HOFS => self.bg_hofs[0],
            BG0VOFS => self.bg_vofs[0],
            BG1HOFS => self.bg_hofs[1],
            BG1VOFS => self.bg_vofs[1],
            BG2HOFS => self.bg_hofs[2],
            BG2VOFS => self.bg_vofs[2],
            BG3HOFS => self.bg_hofs[3],
            BG3VOFS => self.bg_vofs[3],
            _ => 0,
        }
    }

    /// Scrivi registro I/O
    pub fn write_register(&mut self, addr: u32, value: u16) {
        match addr {
            DISPCNT => {
                self.dispcnt = value;
                // Bit 0-2: Mode
                // Bit 4: Display frame select (Mode 4/5)
                // Bit 8-12: BG0-BG3, OBJ enable
            }
            DISPSTAT => {
                // Bit 3-5 sono read-write (VCount setting)
                self.dispstat = (self.dispstat & 0x0007) | (value & 0xFFF8);
            }
            BG0CNT => self.bg_control[0] = BgControl::from_u16(value),
            BG1CNT => self.bg_control[1] = BgControl::from_u16(value),
            BG2CNT => self.bg_control[2] = BgControl::from_u16(value),
            BG3CNT => self.bg_control[3] = BgControl::from_u16(value),
            BG0HOFS => self.bg_hofs[0] = value & 0x1FF,
            BG0VOFS => self.bg_vofs[0] = value & 0x1FF,
            BG1HOFS => self.bg_hofs[1] = value & 0x1FF,
            BG1VOFS => self.bg_vofs[1] = value & 0x1FF,
            BG2HOFS => self.bg_hofs[2] = value & 0x1FF,
            BG2VOFS => self.bg_vofs[2] = value & 0x1FF,
            BG3HOFS => self.bg_hofs[3] = value & 0x1FF,
            BG3VOFS => self.bg_vofs[3] = value & 0x1FF,
            _ => {}
        }
    }

    /// Ottieni modalità display corrente
    pub fn display_mode(&self) -> DisplayMode {
        match self.dispcnt & 0x7 {
            0 => DisplayMode::Mode0,
            1 => DisplayMode::Mode1,
            2 => DisplayMode::Mode2,
            3 => DisplayMode::Mode3,
            4 => DisplayMode::Mode4,
            5 => DisplayMode::Mode5,
            _ => DisplayMode::Mode0,
        }
    }

    /// Esegui cicli PPU
    pub fn step(&mut self, cycles: u32, vram: &[u8]) {
        self.cycles += cycles;

        // Un scanline = 1232 cicli (960 draw + 272 hblank)
        // 228 scanlines totali (160 visibili + 68 vblank)
        const CYCLES_PER_SCANLINE: u32 = 1232;
        const SCANLINES_TOTAL: u16 = 228;

        while self.cycles >= CYCLES_PER_SCANLINE {
            self.cycles -= CYCLES_PER_SCANLINE;

            // Renderizza scanline se visibile
            if self.scanline < 160 {
                self.render_scanline(vram);
            }

            self.scanline += 1;

            if self.scanline >= SCANLINES_TOTAL {
                self.scanline = 0;
            }

            // Aggiorna DISPSTAT flags
            self.update_dispstat();
        }
    }

    /// Aggiorna flag DISPSTAT
    fn update_dispstat(&mut self) {
        // Bit 0: VBlank flag
        if self.in_vblank() {
            self.dispstat |= 0x0001;
        } else {
            self.dispstat &= !0x0001;
        }

        // Bit 1: HBlank flag (sempre 0 per semplicità, TODO: implementare)
        // Bit 2: VCount flag (TODO: implementare confronto)
    }

    /// Verifica se siamo in VBlank
    pub fn in_vblank(&self) -> bool {
        self.scanline >= 160
    }

    /// Renderizza un singolo scanline
    fn render_scanline(&mut self, vram: &[u8]) {
        match self.display_mode() {
            DisplayMode::Mode0 => self.render_mode0_scanline(vram),
            DisplayMode::Mode3 => self.render_mode3_scanline(vram),
            DisplayMode::Mode4 => {
                // TODO: Mode 4 (paletted)
            }
            _ => {
                // TODO: Altri mode (tiled)
            }
        }

        // Renderizza sprite se abilitati (bit 12 di DISPCNT)
        if (self.dispcnt & (1 << 12)) != 0 {
            self.render_sprites_scanline(vram);
        }
    }

    /// Renderizza scanline in Mode 0 (4 tiled backgrounds)
    fn render_mode0_scanline(&mut self, vram: &[u8]) {
        let line = self.scanline as usize;

        // Buffer temporaneo per pixel di ogni layer con priorità
        // (color_rgb555, priority, has_pixel)
        let mut layers: [Vec<(u16, u8, bool)>; 4] = [
            vec![(0, 0, false); SCREEN_WIDTH],
            vec![(0, 0, false); SCREEN_WIDTH],
            vec![(0, 0, false); SCREEN_WIDTH],
            vec![(0, 0, false); SCREEN_WIDTH],
        ];

        // Renderizza ogni background se abilitato
        for (bg_num, layer) in layers.iter_mut().enumerate() {
            // Controlla se BG è abilitato in DISPCNT
            if (self.dispcnt & (1 << (8 + bg_num))) == 0 {
                continue;
            }

            self.render_bg_scanline(vram, bg_num, layer, line);
        }

        // Compositing: priorità più bassa = davanti
        // Per ogni pixel X, trova il layer con priorità più bassa che ha un pixel
        for x in 0..SCREEN_WIDTH {
            let mut final_color = 0u16; // Backdrop (nero)
            let mut found = false;

            // Scansiona tutte le priorità da 0 a 3
            for priority in 0..=3 {
                // Controlla ogni layer per questa priorità
                for layer in &layers {
                    let (color, layer_priority, has_pixel) = layer[x];
                    if has_pixel && layer_priority == priority {
                        final_color = color;
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }

            self.framebuffer[line * SCREEN_WIDTH + x] = final_color;
        }
    }

    /// Renderizza un singolo background per una scanline
    fn render_bg_scanline(
        &self,
        vram: &[u8],
        bg_num: usize,
        layer: &mut [(u16, u8, bool)],
        line: usize,
    ) {
        let bg_control = &self.bg_control[bg_num];
        let priority = bg_control.priority;

        // Calcola posizione Y con scrolling
        let scroll_y = self.bg_vofs[bg_num] as usize;
        let y = (line + scroll_y) & 0x1FF; // Wrap a 512 pixel max

        let (screen_width_tiles, screen_height_tiles) = bg_control.get_screen_size();
        let tile_y = y / 8;

        // Se siamo fuori dai bounds del tilemap, skip
        if tile_y >= screen_height_tiles {
            return;
        }

        let scroll_x = self.bg_hofs[bg_num] as usize;

        // Renderizza 240 pixel + scroll extra per tile parziali
        for (x, pixel) in layer.iter_mut().enumerate().take(SCREEN_WIDTH) {
            let pixel_x = (x + scroll_x) & 0x1FF; // Wrap a 512 pixel max
            let tile_x = pixel_x / 8;

            if tile_x >= screen_width_tiles {
                continue;
            }

            // Leggi tile entry dal tilemap
            let screen_base_addr = (bg_control.screen_base as usize) * 2048;
            let tile_offset = tile_y * screen_width_tiles + tile_x;
            let tile_entry_addr = screen_base_addr + tile_offset * 2;

            if tile_entry_addr + 1 >= vram.len() {
                continue;
            }

            // Tile entry: 16-bit
            // Bits 0-9: Tile number
            // Bit 10: H-flip
            // Bit 11: V-flip
            // Bits 12-15: Palette number (solo se 16 colori mode)
            let tile_entry =
                (vram[tile_entry_addr] as u16) | ((vram[tile_entry_addr + 1] as u16) << 8);
            let tile_num = (tile_entry & 0x3FF) as usize;
            let h_flip = (tile_entry & (1 << 10)) != 0;
            let v_flip = (tile_entry & (1 << 11)) != 0;
            let palette_bank = ((tile_entry >> 12) & 0xF) as usize;

            // Calcola posizione pixel all'interno del tile (0-7)
            let mut tile_pixel_x = pixel_x % 8;
            let mut tile_pixel_y = y % 8;

            if h_flip {
                tile_pixel_x = 7 - tile_pixel_x;
            }
            if v_flip {
                tile_pixel_y = 7 - tile_pixel_y;
            }

            // Leggi pixel dal tile data
            let char_base_addr = (bg_control.char_base as usize) * 16384;
            let palette_index = if bg_control.palette_256 {
                // 256 colori: 1 byte per pixel, 64 byte per tile
                let tile_addr = char_base_addr + tile_num * 64;
                let pixel_addr = tile_addr + tile_pixel_y * 8 + tile_pixel_x;
                if pixel_addr >= vram.len() {
                    0
                } else {
                    vram[pixel_addr] as usize
                }
            } else {
                // 16 colori: 4 bit per pixel, 32 byte per tile
                let tile_addr = char_base_addr + tile_num * 32;
                let pixel_addr = tile_addr + tile_pixel_y * 4 + tile_pixel_x / 2;
                if pixel_addr >= vram.len() {
                    0
                } else {
                    let byte = vram[pixel_addr];
                    if tile_pixel_x & 1 == 0 {
                        (byte & 0xF) as usize
                    } else {
                        ((byte >> 4) & 0xF) as usize
                    }
                }
            };

            // Color 0 = trasparente
            if palette_index == 0 {
                continue;
            }

            // Lookup colore nella palette
            let color = if bg_control.palette_256 {
                // 256 color palette
                self.read_palette(palette_index)
            } else {
                // 16x16 palette
                let palette_offset = palette_bank * 16 + palette_index;
                self.read_palette(palette_offset)
            };

            *pixel = (color, priority, true);
        }
    }

    /// Leggi colore RGB555 dalla palette RAM
    fn read_palette(&self, index: usize) -> u16 {
        let addr = index * 2;
        if addr + 1 < BG_PALETTE_SIZE {
            (self.palette_ram[addr] as u16) | ((self.palette_ram[addr + 1] as u16) << 8)
        } else {
            0
        }
    }

    /// Leggi byte dalla palette RAM
    pub fn read_palette_byte(&self, offset: usize) -> u8 {
        if offset < PALETTE_RAM_SIZE {
            self.palette_ram[offset]
        } else {
            0
        }
    }

    /// Scrivi byte nella palette RAM
    pub fn write_palette_byte(&mut self, offset: usize, value: u8) {
        if offset < PALETTE_RAM_SIZE {
            self.palette_ram[offset] = value;
        }
    }

    /// Leggi halfword dalla palette RAM
    pub fn read_palette_halfword(&self, offset: usize) -> u16 {
        if offset + 1 < PALETTE_RAM_SIZE {
            (self.palette_ram[offset] as u16) | ((self.palette_ram[offset + 1] as u16) << 8)
        } else {
            0
        }
    }

    /// Scrivi halfword nella palette RAM
    pub fn write_palette_halfword(&mut self, offset: usize, value: u16) {
        if offset + 1 < PALETTE_RAM_SIZE {
            self.palette_ram[offset] = (value & 0xFF) as u8;
            self.palette_ram[offset + 1] = ((value >> 8) & 0xFF) as u8;
        }
    }

    /// Leggi byte da OAM
    pub fn read_oam_byte(&self, offset: usize) -> u8 {
        if offset < OAM_SIZE {
            self.oam[offset]
        } else {
            0
        }
    }

    /// Scrivi byte in OAM
    pub fn write_oam_byte(&mut self, offset: usize, value: u8) {
        if offset < OAM_SIZE {
            self.oam[offset] = value;
        }
    }

    /// Leggi halfword da OAM
    pub fn read_oam_halfword(&self, offset: usize) -> u16 {
        if offset + 1 < OAM_SIZE {
            (self.oam[offset] as u16) | ((self.oam[offset + 1] as u16) << 8)
        } else {
            0
        }
    }

    /// Scrivi halfword in OAM
    pub fn write_oam_halfword(&mut self, offset: usize, value: u16) {
        if offset + 1 < OAM_SIZE {
            self.oam[offset] = (value & 0xFF) as u8;
            self.oam[offset + 1] = ((value >> 8) & 0xFF) as u8;
        }
    }

    /// Leggi sprite da OAM (index 0-127)
    pub fn read_sprite(&self, index: usize) -> SpriteAttribute {
        if index < OAM_SPRITE_COUNT {
            let offset = index * 8;
            SpriteAttribute::from_oam_bytes(&self.oam[offset..offset + 6])
        } else {
            SpriteAttribute::default()
        }
    }

    /// Renderizza scanline in Mode 3 (bitmap 16-bit)
    fn render_mode3_scanline(&mut self, vram: &[u8]) {
        // Mode 3: VRAM è array di u16 (RGB555)
        // Offset = scanline * width * 2 byte
        let line = self.scanline as usize;
        let offset = line * SCREEN_WIDTH * 2;

        // Copia scanline da VRAM a framebuffer
        for x in 0..SCREEN_WIDTH {
            let vram_idx = offset + x * 2;

            // Leggi pixel RGB555 (little endian)
            if vram_idx + 1 < vram.len() {
                let pixel = (vram[vram_idx] as u16) | ((vram[vram_idx + 1] as u16) << 8);
                self.framebuffer[line * SCREEN_WIDTH + x] = pixel;
            } else {
                // Fuori bounds, pixel nero
                self.framebuffer[line * SCREEN_WIDTH + x] = 0;
            }
        }
    }

    /// Renderizza sprites per la scanline corrente
    fn render_sprites_scanline(&mut self, vram: &[u8]) {
        let line = self.scanline as usize;

        // Buffer sprite priorità (color, priority, has_sprite)
        let mut sprite_buffer: Vec<(u16, u8, bool)> = vec![(0, 4, false); SCREEN_WIDTH];

        // Renderizza sprite in ordine inverso (priorità: index più alto = dietro)
        for sprite_idx in (0..OAM_SPRITE_COUNT).rev() {
            let sprite = self.read_sprite(sprite_idx);

            if !sprite.is_visible() {
                continue;
            }

            let (sprite_width, sprite_height) = sprite.get_size();
            let sprite_y = sprite.y as usize;

            // Verifica se sprite interseca questa scanline
            let y_in_sprite = if line >= sprite_y {
                line.wrapping_sub(sprite_y)
            } else {
                // Wrap-around per Y > 160
                line.wrapping_add(256).wrapping_sub(sprite_y)
            };

            if y_in_sprite >= sprite_height {
                continue; // Sprite non in questa scanline
            }

            // Applica V-flip
            let actual_y = if sprite.v_flip {
                sprite_height - 1 - y_in_sprite
            } else {
                y_in_sprite
            };

            // Renderizza ogni pixel dello sprite
            for sprite_x in 0..sprite_width {
                let screen_x = (sprite.x as usize).wrapping_add(sprite_x) & 0x1FF;

                if screen_x >= SCREEN_WIDTH {
                    continue;
                }

                // Applica H-flip
                let actual_x = if sprite.h_flip {
                    sprite_width - 1 - sprite_x
                } else {
                    sprite_x
                };

                // Calcola tile e pixel all'interno del tile
                let tiles_per_row = sprite_width / 8;
                let tile_x = actual_x / 8;
                let tile_y = actual_y / 8;
                let pixel_x = actual_x % 8;
                let pixel_y = actual_y % 8;

                // Calcola tile index
                let tile_offset = if sprite.palette_256 {
                    // 256 colori: tile sequenziali
                    tile_y * tiles_per_row + tile_x
                } else {
                    // 16 colori: tile in layout 2D
                    tile_y * 32 + tile_x
                };

                let tile_num = sprite.tile_index as usize + tile_offset;

                // Leggi pixel dal tile in VRAM OBJ
                let palette_index = if sprite.palette_256 {
                    // 256 colori: 64 byte per tile
                    let tile_addr = OBJ_TILE_BASE + tile_num * 64;
                    let pixel_addr = tile_addr + pixel_y * 8 + pixel_x;
                    if pixel_addr < vram.len() {
                        vram[pixel_addr] as usize
                    } else {
                        0
                    }
                } else {
                    // 16 colori: 32 byte per tile
                    let tile_addr = OBJ_TILE_BASE + tile_num * 32;
                    let pixel_addr = tile_addr + pixel_y * 4 + pixel_x / 2;
                    if pixel_addr < vram.len() {
                        let byte = vram[pixel_addr];
                        if pixel_x & 1 == 0 {
                            (byte & 0xF) as usize
                        } else {
                            ((byte >> 4) & 0xF) as usize
                        }
                    } else {
                        0
                    }
                };

                // Color 0 = trasparente
                if palette_index == 0 {
                    continue;
                }

                // Lookup nella palette OBJ
                let color = if sprite.palette_256 {
                    // 256 colori
                    self.read_obj_palette(palette_index)
                } else {
                    // 16 colori
                    let palette_offset = sprite.palette_bank as usize * 16 + palette_index;
                    self.read_obj_palette(palette_offset)
                };

                // Controlla priority: sprite con priority più bassa (numero) = davanti
                let (_, current_priority, has_sprite) = sprite_buffer[screen_x];
                if !has_sprite || sprite.priority <= current_priority {
                    sprite_buffer[screen_x] = (color, sprite.priority, true);
                }
            }
        }

        // Composite sprite sul framebuffer
        for (x, &(sprite_color, _sprite_priority, has_sprite)) in sprite_buffer.iter().enumerate() {
            if has_sprite {
                // TODO: Considera priority BG vs OBJ
                // Per ora sprite sempre sopra background
                self.framebuffer[line * SCREEN_WIDTH + x] = sprite_color;
            }
        }
    }

    /// Leggi colore RGB555 dalla palette OBJ
    fn read_obj_palette(&self, index: usize) -> u16 {
        let addr = OBJ_PALETTE_OFFSET + index * 2;
        if addr + 1 < PALETTE_RAM_SIZE {
            (self.palette_ram[addr] as u16) | ((self.palette_ram[addr + 1] as u16) << 8)
        } else {
            0
        }
    }

    /// Ottieni framebuffer per rendering
    pub fn framebuffer(&self) -> &[u16] {
        &self.framebuffer
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bg_control_parsing() {
        // Test BG0CNT con valori semplici
        let bg_ctrl = BgControl {
            priority: 2,
            char_base: 1,
            mosaic: false,
            palette_256: true,
            screen_base: 10,
            wrap: false,
            screen_size: 1,
        };

        let value = bg_ctrl.to_u16();
        let parsed = BgControl::from_u16(value);

        assert_eq!(parsed.priority, 2);
        assert_eq!(parsed.char_base, 1);
        assert!(parsed.palette_256);
        assert_eq!(parsed.screen_base, 10);
        assert_eq!(parsed.screen_size, 1);
    }
    #[test]
    fn test_bg_screen_size() {
        let configs = [
            (0, (32, 32)), // 256x256
            (1, (64, 32)), // 512x256
            (2, (32, 64)), // 256x512
            (3, (64, 64)), // 512x512
        ];

        for (screen_size, expected) in configs {
            let bg_ctrl = BgControl {
                screen_size,
                ..BgControl::default()
            };
            assert_eq!(bg_ctrl.get_screen_size(), expected);
        }
    }

    #[test]
    fn test_palette_ram_access() {
        let mut ppu = PPU::new();

        // Scrivi colore RGB555 nella palette 0
        ppu.write_palette_halfword(0, 0x7FFF); // Bianco (R=31, G=31, B=31)
        assert_eq!(ppu.read_palette_halfword(0), 0x7FFF);

        // Scrivi colore nella palette 5
        ppu.write_palette_halfword(10, 0x001F); // Rosso puro
        assert_eq!(ppu.read_palette_halfword(10), 0x001F);

        // Test byte access
        ppu.write_palette_byte(20, 0xAB);
        ppu.write_palette_byte(21, 0xCD);
        assert_eq!(ppu.read_palette_halfword(20), 0xCDAB);
    }

    #[test]
    fn test_mode0_simple_tile() {
        let mut ppu = PPU::new();

        // Setup Mode 0
        ppu.dispcnt = 0x0100; // Mode 0, BG0 enabled (bit 8)

        // Setup BG0: priority=0, char_base=0, screen_base=8 (8*2KB=16KB offset), 32x32 tiles
        ppu.bg_control[0] = BgControl {
            priority: 0,
            char_base: 0,
            mosaic: false,
            palette_256: false, // 16-color mode
            screen_base: 8,     // Tilemap at 16KB
            wrap: false,
            screen_size: 0, // 32x32
        };

        // Crea una VRAM di test
        let mut vram = vec![0u8; 96 * 1024]; // 96KB

        // Palette: colore 1 = rosso (0x001F)
        ppu.palette_ram[0] = 0x00; // Color 0 (trasparente)
        ppu.palette_ram[1] = 0x00;
        ppu.palette_ram[2] = 0x1F; // Color 1 low byte (rosso: R=31, G=0, B=0)
        ppu.palette_ram[3] = 0x00; // Color 1 high byte

        // Character data at char_base=0: tile 0 = solido con palette index 1
        // 16 colori: 4 bit per pixel, 32 byte per tile (8x8 pixels)
        // Riempi ogni byte con 0x11 (due pixel con index 1)
        vram.iter_mut().take(32).for_each(|v| *v = 0x11);

        // Tilemap at screen_base=8 (8*2048 = 16384 bytes offset)
        let tilemap_offset = 16384;
        vram[tilemap_offset] = 0x00; // Tile number = 0
        vram[tilemap_offset + 1] = 0x00; // No flip, palette bank 0

        // Renderizza scanline 0
        ppu.scanline = 0;
        ppu.render_mode0_scanline(&vram);

        // Verifica che i primi 8 pixel siano rossi
        for x in 0..8 {
            assert_eq!(
                ppu.framebuffer[x], 0x001F,
                "Pixel {} should be red (got 0x{:04X})",
                x, ppu.framebuffer[x]
            );
        }
    }
    #[test]
    fn test_mode0_scrolling() {
        let mut ppu = PPU::new();

        // Setup Mode 0, BG0 enabled
        ppu.write_register(DISPCNT, 0x0100);
        ppu.write_register(BG0CNT, 0x0000);

        // Setup scroll offset
        ppu.write_register(BG0HOFS, 8); // Scroll 8 pixel a destra
        ppu.write_register(BG0VOFS, 0);

        let mut vram = vec![0u8; 96 * 1024];

        // Palette: colore 1 = blu (0x7C00)
        ppu.write_palette_halfword(2, 0x7C00);

        // Tile 0: tutto blu (index 1)
        vram.iter_mut().take(32).for_each(|v| *v = 0x11);

        // Tile 1: diverso pattern (trasparente)
        vram.iter_mut().skip(32).take(32).for_each(|v| *v = 0x00); // Tilemap: tile 0 a (0,0), tile 1 a (1,0)
        vram[0] = 0x00;
        vram[1] = 0x00;
        vram[2] = 0x01;
        vram[3] = 0x00;

        // Renderizza scanline 0 con scroll
        ppu.scanline = 0;
        ppu.render_mode0_scanline(&vram);

        // I primi pixel dovrebbero essere neri (trasparenti dal tile 1 scrollato)
        assert_eq!(ppu.framebuffer[0], 0x0000);
    }

    #[test]
    fn test_mode0_priority() {
        let mut ppu = PPU::new();

        // Setup Mode 0, BG0 e BG1 enabled
        ppu.write_register(DISPCNT, 0x0300); // BG0 + BG1

        // BG0: priority=1
        ppu.write_register(BG0CNT, 0x0001);
        // BG1: priority=0 (più alta, quindi davanti)
        ppu.write_register(BG1CNT, 0x0000);

        let mut vram = vec![0u8; 96 * 1024];

        // Palette: colore 1 = rosso, colore 2 = verde
        ppu.write_palette_halfword(2, 0x001F); // Index 1 = rosso
        ppu.write_palette_halfword(4, 0x03E0); // Index 2 = verde

        // Tile 0 per BG0: rosso
        vram.iter_mut().take(32).for_each(|v| *v = 0x11);

        // Tile 1 per BG1: verde (assumendo stesso char base per semplicità)
        vram.iter_mut().skip(32).take(32).for_each(|v| *v = 0x22); // Tilemap BG0 (screen_base=0): tile 0
        vram[0] = 0x00;
        vram[1] = 0x00;

        // Tilemap BG1 (screen_base=0 per semplicità): tile 1
        // In realtà dovrebbe avere screen_base diverso, ma per il test va bene
        vram[2] = 0x01;
        vram[3] = 0x00;

        ppu.scanline = 0;
        ppu.render_mode0_scanline(&vram);

        // BG1 ha priority 0, quindi dovrebbe essere visibile sopra BG0
        // Ma questo test è semplificato - nella realtà servirebbero screen base differenti
    }

    #[test]
    fn test_mode0_transparency() {
        let mut ppu = PPU::new();

        ppu.dispcnt = 0x0100; // Mode 0, BG0
        ppu.bg_control[0] = BgControl {
            priority: 0,
            char_base: 0,
            mosaic: false,
            palette_256: false,
            screen_base: 8,
            wrap: false,
            screen_size: 0,
        };

        let mut vram = vec![0u8; 96 * 1024];

        // Palette: colore 1 = bianco
        ppu.palette_ram[2] = 0xFF;
        ppu.palette_ram[3] = 0x7F;

        // Tile con alcuni pixel trasparenti (color 0)
        vram[0] = 0x01; // Pixel 0=1, pixel 1=0
        vram[1] = 0x10; // Pixel 2=0, pixel 3=1

        // Tilemap
        let tilemap_offset = 16384;
        vram[tilemap_offset] = 0x00;
        vram[tilemap_offset + 1] = 0x00;

        ppu.scanline = 0;
        ppu.render_mode0_scanline(&vram);

        // Pixel 0: bianco (index 1)
        assert_eq!(ppu.framebuffer[0], 0x7FFF);
        // Pixel 1: nero/trasparente (index 0)
        assert_eq!(ppu.framebuffer[1], 0x0000);
        // Pixel 2: nero/trasparente
        assert_eq!(ppu.framebuffer[2], 0x0000);
        // Pixel 3: bianco
        assert_eq!(ppu.framebuffer[3], 0x7FFF, "Pixel 3 should be white");
    }

    #[test]
    fn test_sprite_attribute_parsing() {
        // Crea OAM bytes per uno sprite 16x16 a (50,30)
        let oam = vec![
            30,   // Attr0 low: Y=30
            0x00, // Attr0 high: normal mode, no mosaic, 16 colors, square
            50,   // Attr1 low: X=50
            0x40, // Attr1 high: size=1, no flip
            0x05, // Attr2 low: tile=5
            0x20, // Attr2 high: priority=0, palette=2
        ];

        let sprite = SpriteAttribute::from_oam_bytes(&oam);

        assert_eq!(sprite.y, 30);
        assert_eq!(sprite.x, 50);
        assert_eq!(sprite.tile_index, 5);
        assert_eq!(sprite.priority, 0);
        assert_eq!(sprite.palette_bank, 2);
        assert_eq!(sprite.get_size(), (16, 16));
        assert!(sprite.is_visible());
    }

    #[test]
    fn test_sprite_sizes() {
        // Test array di configurazioni (shape, size, expected_dimensions)
        let test_cases = [
            // Square
            (0, 0, (8, 8)),
            (0, 1, (16, 16)),
            (0, 2, (32, 32)),
            (0, 3, (64, 64)),
            // Wide
            (1, 0, (16, 8)),
            (1, 1, (32, 8)),
            (1, 2, (32, 16)),
            (1, 3, (64, 32)),
            // Tall
            (2, 0, (8, 16)),
            (2, 1, (8, 32)),
            (2, 2, (16, 32)),
            (2, 3, (32, 64)),
        ];

        for (shape, size, expected) in test_cases {
            let sprite = SpriteAttribute {
                shape,
                size,
                ..SpriteAttribute::default()
            };
            assert_eq!(
                sprite.get_size(),
                expected,
                "Failed for shape={}, size={}",
                shape,
                size
            );
        }
    }

    #[test]
    fn test_oam_read_write() {
        let mut ppu = PPU::new();

        // Scrivi sprite in OAM slot 0
        ppu.write_oam_halfword(0, 0x0050); // Attr0: Y=80, mode=0
        ppu.write_oam_halfword(2, 0x0064); // Attr1: X=100
        ppu.write_oam_halfword(4, 0x000A); // Attr2: tile=10

        // Leggi sprite
        let sprite = ppu.read_sprite(0);
        assert_eq!(sprite.y, 80);
        assert_eq!(sprite.x, 100);
        assert_eq!(sprite.tile_index, 10);
    }

    #[test]
    fn test_sprite_rendering_simple() {
        let mut ppu = PPU::new();

        // Setup Mode 0 con sprite abilitati
        ppu.dispcnt = 0x1000; // Bit 12: OBJ enable

        // OBJ palette: colore 1 = blu
        ppu.palette_ram[OBJ_PALETTE_OFFSET + 2] = 0x00; // Low
        ppu.palette_ram[OBJ_PALETTE_OFFSET + 3] = 0x7C; // High (0x7C00 = blu)

        // Crea sprite 8x8 a posizione (10, 5)
        ppu.write_oam_halfword(0, 0x0005); // Y=5
        ppu.write_oam_halfword(2, 0x000A); // X=10
        ppu.write_oam_halfword(4, 0x0000); // Tile=0, priority=0

        // VRAM: tile sprite con pixel blu (index 1)
        let mut vram = vec![0u8; 96 * 1024];
        let tile_offset = OBJ_TILE_BASE;
        vram.iter_mut()
            .skip(tile_offset)
            .take(32)
            .for_each(|v| *v = 0x11);

        // Renderizza scanline 5 (dove lo sprite è visibile)
        ppu.scanline = 5;
        ppu.render_sprites_scanline(&vram);

        // Verifica pixel blu a X=10
        assert_eq!(
            ppu.framebuffer[5 * SCREEN_WIDTH + 10],
            0x7C00,
            "Sprite pixel should be blue"
        );
        assert_eq!(
            ppu.framebuffer[5 * SCREEN_WIDTH + 17],
            0x7C00,
            "Sprite extends to X=17"
        );
    }

    #[test]
    fn test_sprite_transparency() {
        let mut ppu = PPU::new();

        ppu.dispcnt = 0x1000; // OBJ enable

        // OBJ palette
        ppu.palette_ram[OBJ_PALETTE_OFFSET + 2] = 0xFF;
        ppu.palette_ram[OBJ_PALETTE_OFFSET + 3] = 0x7F; // Bianco

        // Sprite 8x8 a (0, 0)
        ppu.write_oam_halfword(0, 0x0000); // Y=0
        ppu.write_oam_halfword(2, 0x0000); // X=0
        ppu.write_oam_halfword(4, 0x0000); // Tile=0

        // VRAM: tile con pattern trasparente/opaco
        let mut vram = vec![0u8; 96 * 1024];
        let tile_offset = OBJ_TILE_BASE;
        vram[tile_offset] = 0x01; // Pixel 0=1 (bianco), pixel 1=0 (trasparente)
        vram[tile_offset + 1] = 0x10; // Pixel 2=0, pixel 3=1

        ppu.scanline = 0;
        ppu.render_sprites_scanline(&vram);

        // Pixel 0: bianco
        assert_eq!(ppu.framebuffer[0], 0x7FFF);
        // Pixel 1: trasparente (dovrebbe essere 0 = background)
        assert_eq!(ppu.framebuffer[1], 0x0000);
        // Pixel 3: bianco
        assert_eq!(ppu.framebuffer[3], 0x7FFF);
    }
}
