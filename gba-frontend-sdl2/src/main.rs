mod ui;
mod input;

use gba_core::{Cartridge, GbaEmulator};
use std::env;
use std::path::PathBuf;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    // Inizializza logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    log::info!("🎮 GBA Emulator - Rust Edition");
    log::info!("Version: {}", env!("CARGO_PKG_VERSION"));
    
    // Parse argomenti
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <rom_file> [--bios <bios_file>]", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} pokemon_emerald.gba", args[0]);
        eprintln!("  {} pokemon_emerald.gba --bios gba_bios.bin", args[0]);
        std::process::exit(1);
    }
    
    let rom_path = PathBuf::from(&args[1]);
    let bios_path = args.iter()
        .position(|arg| arg == "--bios")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from);
    
    // Crea emulatore
    let mut emulator = GbaEmulator::new();
    
    // Carica BIOS (opzionale)
    if let Some(bios_path) = bios_path {
        log::info!("Loading BIOS from: {}", bios_path.display());
        let bios = std::fs::read(&bios_path)
            .with_context(|| format!("Failed to load BIOS: {}", bios_path.display()))?;
        emulator.load_bios(bios);
    } else {
        log::warn!("No BIOS provided - using HLE (High Level Emulation)");
        // TODO: Implementa HLE BIOS
    }
    
    // Carica ROM
    log::info!("Loading ROM from: {}", rom_path.display());
    let cartridge = Cartridge::load(&rom_path)
        .with_context(|| format!("Failed to load ROM: {}", rom_path.display()))?;
    
    emulator.load_cartridge(cartridge);
    emulator.reset();
    
    // Avvia UI
    log::info!("Starting emulator...");
    ui::run(emulator)?;
    
    Ok(())
}
