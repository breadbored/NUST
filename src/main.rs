mod cartridge;
mod cpu;
mod ppu;
mod system;

use rand::distributions::uniform::SampleBorrow;
use sdl2::event::Event;
use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use crate::ppu::PPU;
use crate::{cpu::CPU, ppu::Screen};
use cartridge::{get_rom, Cartridge};
use sdl2::pixels::Color;
use system::System;

fn main() {
    // System
    const SCALE: u8 = 3;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("CARESemu", 256 * SCALE as u32, 240 * SCALE as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Emulator
    let mut rom: Cartridge = get_rom();
    let system = Arc::new(Mutex::new(System::new(rom.clone())));

    let mut last_cpu_cycle: u128 = get_time();
    let mut last_ppu_cycle: u128 = get_time();
    let mut last_apu_cycle: u128 = get_time();

    let mut last_draw_time: u128 = get_time();

    // System Event Loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { .. } => {
                    system
                        .lock()
                        .unwrap()
                        .cpu
                        .lock()
                        .unwrap()
                        .request_irq_interrupt();
                }
                _ => {}
            }
        }

        (last_cpu_cycle, last_ppu_cycle, last_apu_cycle) = run_processor(
            last_cpu_cycle,
            last_ppu_cycle,
            last_apu_cycle,
            &mut system.clone(),
        );

        // Render at 60 FPS
        if get_time() - last_draw_time > (1_000_000_000u128 / 60) {
            canvas.clear();
            let ppu = system.lock().unwrap().ppu.clone();
            let screen: Screen = ppu.lock().unwrap().get_screen(rom.clone());

            // Draw the screen
            for y in 0..240 {
                for x in 0..256 {
                    let color_nes = screen.get_pixel(x, y);
                    let color = Color::RGB(color_nes.r, color_nes.g, color_nes.b);
                    canvas.set_draw_color(color);
                    canvas
                        .fill_rect(sdl2::rect::Rect::new(
                            x as i32 * SCALE as i32,
                            y as i32 * SCALE as i32,
                            SCALE as u32,
                            SCALE as u32,
                        ))
                        .unwrap();
                }
            }

            canvas.present();
            last_draw_time = get_time();
        }

        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // sloppy FPS limit
    }
}

fn get_time() -> u128 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
}

fn run_processor(
    mut last_cpu_cycle: u128,
    mut last_ppu_cycle: u128,
    mut last_apu_cycle: u128,
    system: &mut Arc<Mutex<System>>,
) -> (u128, u128, u128) {
    // println!("Running processor");
    const CPU_CYCLES: u128 = 559; // 1.79 MHz
    const PPU_CYCLES: u128 = 186; // 5.37 MHz
    const APU_CYCLES: u128 = 559; // 1.79 MHz

    // CPU runs at 1.79 MHz
    let check_cpu_time = get_time();
    if check_cpu_time - last_cpu_cycle.borrow() >= CPU_CYCLES {
        // println!("Running CPU");
        let cpu = system.lock().unwrap().cpu.clone();
        let cycles_ran = cpu.lock().unwrap().tick(&mut system.clone());
        last_cpu_cycle = get_time()
            + (CPU_CYCLES * cycles_ran as u128)
            + (if cpu.lock().unwrap().is_jammed() {
                0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
            } else {
                0
            });
    }

    // PPU runs at 5.37 MHz
    let check_ppu_time = get_time();
    if check_ppu_time - last_ppu_cycle >= PPU_CYCLES {
        let ppu = system.lock().unwrap().ppu.clone();
        ppu.lock().unwrap().tick();
        last_ppu_cycle = get_time();
    }

    // APU runs at 1.79 MHz
    let check_apu_time = get_time();
    if check_apu_time - last_apu_cycle >= APU_CYCLES {
        // APU.tick();
        last_apu_cycle = get_time();
    }

    return (last_cpu_cycle, last_ppu_cycle, last_apu_cycle);
}
