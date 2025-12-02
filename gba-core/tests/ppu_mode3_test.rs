use gba_arm7tdmi::cpu::MemoryBus;
use gba_core::GbaEmulator;

#[test]
fn test_mode3_rendering() {
    let mut emu = GbaEmulator::new();

    // Abilita Mode 3 (bitmap 16-bit)
    // DISPCNT = 0x0403 (Mode 3 + BG2 enabled)
    emu.bus.write_halfword(0x04000000, 0x0403);

    // Scrivi alcuni pixel colorati in VRAM (0x06000000)
    // VRAM Mode 3: array di u16 RGB555

    // Pixel rosso (R=31, G=0, B=0) -> 0x7C00
    emu.bus.write_halfword(0x06000000, 0x7C00);

    // Pixel verde (R=0, G=31, B=0) -> 0x03E0
    emu.bus.write_halfword(0x06000002, 0x03E0);

    // Pixel blu (R=0, G=0, B=31) -> 0x001F
    emu.bus.write_halfword(0x06000004, 0x001F);

    // Pixel bianco (R=31, G=31, B=31) -> 0x7FFF
    emu.bus.write_halfword(0x06000006, 0x7FFF);

    // Esegui un frame completo
    emu.run_frame();

    // Verifica che il framebuffer contenga i pixel corretti
    let fb = emu.framebuffer();

    // Primo pixel deve essere rosso
    assert_eq!(fb[0], 0x7C00, "Pixel 0 should be red");

    // Secondo pixel deve essere verde
    assert_eq!(fb[1], 0x03E0, "Pixel 1 should be green");

    // Terzo pixel deve essere blu
    assert_eq!(fb[2], 0x001F, "Pixel 2 should be blue");

    // Quarto pixel deve essere bianco
    assert_eq!(fb[3], 0x7FFF, "Pixel 3 should be white");

    // Verifica VCOUNT (dovrebbe essere 160+ dopo VBlank)
    let vcount = emu.bus.read_halfword(0x04000006);
    println!("VCOUNT after frame: {}", vcount);
    assert!(vcount <= 227, "VCOUNT should be valid (0-227)");
}

#[test]
fn test_mode3_full_scanline() {
    let mut emu = GbaEmulator::new();

    // Abilita Mode 3
    emu.bus.write_halfword(0x04000000, 0x0403);

    // Riempi prima scanline con gradiente rosso
    for x in 0..240 {
        let intensity = ((x * 31) / 240) as u16;
        let color = intensity << 10; // RGB555: R nella posizione 10-14
        emu.bus.write_halfword(0x06000000 + x * 2, color);
    }

    // Esegui frame
    emu.run_frame();

    let fb = emu.framebuffer();

    // Verifica gradiente: primo pixel quasi nero, ultimo pixel rosso massimo
    println!(
        "First pixel: 0x{:04X}, Last pixel: 0x{:04X}",
        fb[0], fb[239]
    );
    println!("Expected last pixel > 0x7800 (bright red)");

    assert!(fb[0] < 0x0400, "First pixel should be dark red");
    assert!(fb[239] > 0x7000, "Last pixel should be bright red");
}
