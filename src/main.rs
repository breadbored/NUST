mod cartridge;
mod cpu;

use cartridge::{get_rom, Cartridge};
use rand::distributions::uniform::SampleBorrow;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use crate::cpu::CPU;

fn main() {
    // System
    const SCALE: u8 = 3;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo", 256 * SCALE as u32, 240 * SCALE as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // // Emulator
    let mut cpu: CPU = CPU::new();
    let mut rom: Cartridge = get_rom();
    let ram: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0; 0x800]));
    let vram: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0; 0x800]));

    let mut last_cpu_cycle: u128 = get_time();
    let mut last_ppu_cycle: u128 = get_time();
    let mut last_apu_cycle: u128 = get_time();

    let mut last_keyboard_poll: u128 = get_time();

    // System Event Loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { .. } => {
                    println!("Key down");
                }
                _ => {}
            }
        }

        println!("Running");

        (last_cpu_cycle, last_ppu_cycle, last_apu_cycle) = run_processor(
            last_cpu_cycle,
            last_ppu_cycle,
            last_apu_cycle,
            &mut cpu,
            &mut rom,
            ram.clone(),
            vram.clone(),
        );

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .draw_line(
                Point::new(0, 0),
                Point::new(256 * (SCALE as i32) - 1, 240 * (SCALE as i32) - 1),
            )
            .unwrap();

        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // let (w, h) = canvas.output_size().unwrap();
        // let mut points = [Point::new(0, 0); 256];
        // points.fill_with(|| Point::new(rng.gen_range(0..w as i32), rng.gen_range(0..h as i32)));
        // // For performance, it's probably better to draw a whole bunch of points at once
        // canvas.draw_points(points.as_slice()).unwrap();

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // sloppy FPS limit
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
    cpu: &mut CPU,
    rom: &mut Cartridge,
    ram: Arc<Mutex<Vec<u8>>>,
    vram: Arc<Mutex<Vec<u8>>>,
) -> (u128, u128, u128) {
    println!("Running processor");
    const CPU_CYCLES: u128 = 559; // 1.79 MHz
    const PPU_CYCLES: u128 = 186; // 5.37 MHz
    const APU_CYCLES: u128 = 559; // 1.79 MHz

    // CPU runs at 1.79 MHz
    let check_cpu_time = get_time();
    if check_cpu_time - last_cpu_cycle.borrow() >= CPU_CYCLES {
        println!("Running CPU");
        let cycles_ran = cpu.tick(rom.clone(), &ram, &vram);
        last_cpu_cycle = get_time() + cycles_ran as u128;
    }

    // PPU runs at 5.37 MHz
    let check_ppu_time = get_time();
    if check_ppu_time - last_ppu_cycle >= PPU_CYCLES {
        // PPU.tick(&vram);
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
