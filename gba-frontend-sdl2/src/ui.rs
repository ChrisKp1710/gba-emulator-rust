use gba_core::GbaEmulator;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use anyhow::Result;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: u32 = 240;
const SCREEN_HEIGHT: u32 = 160;
const SCALE: u32 = 3; // Scala x3 per visibilità migliore

pub fn run(mut emulator: GbaEmulator) -> Result<()> {
    // Inizializza SDL2
    let sdl_context = sdl2::init().map_err(|e| anyhow::anyhow!("Failed to initialize SDL2: {}", e))?;
    let video_subsystem = sdl_context.video().map_err(|e| anyhow::anyhow!("Failed to initialize video: {}", e))?;
    
    // Crea finestra
    let window = video_subsystem
        .window(
            "GBA Emulator - Rust",
            SCREEN_WIDTH * SCALE,
            SCREEN_HEIGHT * SCALE,
        )
        .position_centered()
        .build()?;
    
    let mut canvas = window.into_canvas().accelerated().build()?;
    let texture_creator = canvas.texture_creator();
    
    // Crea texture per il framebuffer
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB565,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
    )?;
    
    let mut event_pump = sdl_context.event_pump().map_err(|e| anyhow::anyhow!("Failed to get event pump: {}", e))?;
    
    // Timing (60 FPS target)
    let frame_duration = Duration::from_micros(16666); // ~60 FPS
    let mut last_frame = Instant::now();
    let mut fps_counter = 0;
    let mut fps_timer = Instant::now();
    
    log::info!("✓ Emulator started successfully!");
    log::info!("Controls:");
    log::info!("  Arrow Keys - D-Pad");
    log::info!("  Z - Button A");
    log::info!("  X - Button B");
    log::info!("  A - Button L");
    log::info!("  S - Button R");
    log::info!("  Enter - Start");
    log::info!("  Backspace - Select");
    log::info!("  F5 - Save State");
    log::info!("  F9 - Load State");
    log::info!("  ESC - Exit");
    
    'running: loop {
        // Gestione eventi
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    log::info!("Shutting down...");
                    break 'running;
                }
                
                Event::KeyDown {
                    keycode: Some(Keycode::F5),
                    ..
                } => {
                    log::info!("Save State (not implemented yet)");
                }
                
                Event::KeyDown {
                    keycode: Some(Keycode::F9),
                    ..
                } => {
                    log::info!("Load State (not implemented yet)");
                }
                
                _ => {}
            }
        }
        
        // Esegui frame emulatore
        emulator.run_frame();
        
        // Aggiorna texture con framebuffer
        let framebuffer = emulator.framebuffer();
        texture.update(None, unsafe {
            std::slice::from_raw_parts(
                framebuffer.as_ptr() as *const u8,
                framebuffer.len() * 2,
            )
        }, SCREEN_WIDTH as usize * 2)?;
        
        // Rendering
        canvas.clear();
        canvas.copy(
            &texture,
            None,
            Some(Rect::new(0, 0, SCREEN_WIDTH * SCALE, SCREEN_HEIGHT * SCALE)),
        ).map_err(|e| anyhow::anyhow!("Failed to copy texture: {}", e))?;
        canvas.present();
        
        // FPS counter
        fps_counter += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            log::debug!("FPS: {}", fps_counter);
            fps_counter = 0;
            fps_timer = Instant::now();
        }
        
        // Limita a 60 FPS
        let elapsed = last_frame.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
        last_frame = Instant::now();
    }
    
    Ok(())
}
