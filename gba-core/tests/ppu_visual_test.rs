use gba_arm7tdmi::cpu::MemoryBus;
use gba_core::GbaEmulator;

/// Test demo che crea uno schermo con gradiente colorato
#[test]
fn test_demo_color_gradient() {
    let mut emu = GbaEmulator::new();

    // Abilita Mode 3
    emu.bus.write_halfword(0x04000000, 0x0403);

    // Crea gradiente colorato su tutto lo schermo
    // Formato RGB555: XBBBBBGGGGGRRRRR (X = non usato)
    for y in 0..160u32 {
        for x in 0..240u32 {
            // Gradiente rosso orizzontale
            let red = ((x * 31) / 240) as u16;

            // Gradiente verde verticale
            let green = ((y * 31) / 160) as u16;

            // Blu fisso a metà intensità
            let blue = 15u16;

            // Combina in RGB555
            let color = (blue << 10) | (green << 5) | red;

            // Scrivi pixel
            let offset = (y * 240 + x) * 2;
            emu.bus.write_halfword(0x06000000 + offset, color);
        }
    }

    // Esegui frame per aggiornare rendering
    emu.run_frame();

    let fb = emu.framebuffer();

    // Verifica angoli
    println!("Top-left: 0x{:04X} (rosso 0, verde 0, blu 15)", fb[0]);
    println!("Top-right: 0x{:04X} (rosso 31, verde 0, blu 15)", fb[239]);
    println!(
        "Bottom-left: 0x{:04X} (rosso 0, verde 31, blu 15)",
        fb[160 * 240 - 240]
    );
    println!(
        "Bottom-right: 0x{:04X} (rosso 31, verde 31, blu 15)",
        fb[160 * 240 - 1]
    );

    // Angolo top-left: blu (0x3C00 = blu 15)
    assert_eq!(fb[0], 0x3C00, "Top-left should be blue only");

    // Angolo top-right: blu+rosso (viola)
    assert!(fb[239] > 0x3C00, "Top-right should have blue and red");

    // Angolo bottom-left: blu+verde (ciano)
    assert!(
        fb[160 * 240 - 240] > 0x3C00,
        "Bottom-left should have blue and green"
    );

    // Angolo bottom-right: tutti i colori (bianco-ish)
    assert!(fb[160 * 240 - 1] > 0x3C00, "Bottom-right should be bright");
}

/// Test demo che disegna barre colorate
#[test]
fn test_demo_color_bars() {
    let mut emu = GbaEmulator::new();

    // Abilita Mode 3
    emu.bus.write_halfword(0x04000000, 0x0403);

    // Colori RGB555
    let colors = [
        0x7FFF, // Bianco
        0x7C00, // Rosso
        0x03E0, // Verde
        0x001F, // Blu
        0x7FE0, // Giallo (Rosso + Verde)
        0x7C1F, // Magenta (Rosso + Blu)
        0x03FF, // Ciano (Verde + Blu)
        0x0000, // Nero
    ];

    let bar_width = 240 / 8; // 30 pixel per barra

    // Disegna barre verticali
    for y in 0..160u32 {
        for x in 0..240u32 {
            let bar_index = (x / bar_width).min(7) as usize;
            let color = colors[bar_index];

            let offset = (y * 240 + x) * 2;
            emu.bus.write_halfword(0x06000000 + offset, color);
        }
    }

    // Esegui frame
    emu.run_frame();

    let fb = emu.framebuffer();

    // Verifica che le barre siano corrette
    assert_eq!(fb[0], 0x7FFF, "First bar should be white");
    assert_eq!(fb[30], 0x7C00, "Second bar should be red");
    assert_eq!(fb[60], 0x03E0, "Third bar should be green");
    assert_eq!(fb[90], 0x001F, "Fourth bar should be blue");

    println!("✅ Color bars test passed!");
}
