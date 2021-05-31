extern crate minifb;
extern crate rand;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

mod bus;
mod chip8;
mod cpu;
mod display;
mod keyboard;
mod memory;

fn get_chip8_keycode_for(key: Option<Key>) -> Option<u8> {
    match key {
        // Row 0
        Some(Key::Key1) => Some(0x1),
        Some(Key::Key2) => Some(0x2),
        Some(Key::Key3) => Some(0x3),
        Some(Key::Key4) => Some(0xC),
        // Row 1
        Some(Key::Q) => Some(0x4),
        Some(Key::W) => Some(0x5),
        Some(Key::E) => Some(0x6),
        Some(Key::R) => Some(0xD),
        // Row 2
        Some(Key::A) => Some(0x7),
        Some(Key::S) => Some(0x8),
        Some(Key::D) => Some(0x9),
        Some(Key::F) => Some(0xE),
        // Row 3
        Some(Key::Z) => Some(0xA),
        Some(Key::X) => Some(0x0),
        Some(Key::C) => Some(0xB),
        Some(Key::V) => Some(0xF),
        // Catch all
        _ => None

    }
}

fn main() {

    let file = File::open("./data/PONG2.ch8");
    let mut data = Vec::<u8>::new();

    // Make sure we can read the ROM
    if let Ok(mut v) = file {
        match v.read_to_end(&mut data) {
            Err(e) => panic!(e),
            _ => ()
        };
    }
    else {
        panic!("Unable to read input")
    }

    let width = 640;
    let height = 320;

    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "Chip8 Emulator",
        width,
        height,
        WindowOptions::default()
    ).unwrap_or_else(|e| {
        panic!("Window creation failed: {:?}", e);
    });

    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(&data);

    let mut last_key_update_time = Instant::now();
    let mut last_instruction_run_time = Instant::now();
    let mut last_display_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys_pressed = window.get_keys_pressed(KeyRepeat::Yes);
        let key = match keys_pressed {
            Some(keys) => if !keys.is_empty() {
                Some(keys[0])
            }
            else {
                None
            },
            None => None,
        };

        let chip8_key = get_chip8_keycode_for(key);

        if chip8_key.is_some() || Instant::now() - last_key_update_time >= Duration::from_millis(200) {
            last_key_update_time = Instant::now();
            chip8.set_key_pressed(chip8_key);
        }

        if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
            chip8.run_instruction();
            last_instruction_run_time = Instant::now();
        }

        if Instant::now() - last_display_time > Duration::from_millis(10) {
            let chip8_buffer = chip8.get_display_buffer();

            for y in 0..height {
                let y_pos = y / 10;
                let offset = y * width;
                for x in 0..width {
                    let index = display::Display::get_index_from_coords(x / 10, y_pos);
                    let pixel = chip8_buffer[index];
                    let color_pixel = match pixel {
                        0 => 0x0,
                        1 => 0xffffff,
                        _ => unreachable!(),
                    };
                    buffer[offset + x] = color_pixel;
                }
            }
            match window.update_with_buffer(&buffer, width, height) {
                Ok(_) => {},
                Err(e) => panic!("Error updating window: {:#?}", e)
            }

            last_display_time = Instant::now();
        }
    }


}
