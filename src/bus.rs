use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::memory;
use crate::memory::Memory;
use std::time::{Instant};

pub struct Bus {
    display: Display,
    keyboard: Keyboard,
    memory: Memory,
    delay_timer: u8,
    delay_timer_set_time: Instant
}

impl Bus {

    pub fn new() -> Self {
        Bus {
            memory: Memory::new(),
            keyboard: Keyboard::new(),
            display: Display::new(),
            delay_timer: 0,
            delay_timer_set_time: Instant::now()
        }
    }

    pub fn memory_read_byte(&self, address: u16) -> u8 {
        self.memory.read_byte(address)
    }

    pub fn memory_write_byte(&mut self, address: u16, value: u8) {
        self.memory.write_byte(address, value)
    }

    pub fn memory_get_size(&self) -> usize {
        memory::SIZE
    }

    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        self.display.draw_byte(byte, x, y)
    }

    pub fn clear_screen(&mut self) {
        self.display.clear()
    }

    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.keyboard.set_key_pressed(key)
    }

    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        self.keyboard.is_key_pressed(key_code)
    }

    pub fn get_key_pressed(&mut self) -> Option<u8> {
        self.keyboard.get_key_pressed()
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer_set_time = Instant::now();
        self.delay_timer = value;
    }

    pub fn get_delay_timer(&self) -> u8 {
        let diff = Instant::now() - self.delay_timer_set_time;
        let ms = diff.as_millis() as u64;
        let ticks: u64 = ms / 16;
        if ticks >= self.delay_timer as u64 {
            0
        }
        else {
            self.delay_timer - ticks as u8
        }
    }

    pub fn get_display_buffer(&self) -> &[u8] {
        self.display.get_display_buffer()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_memory_read_byte() {
        let address: u16 = 0x400;
        let value: u8 = 0x8;

        let mut bus = Bus::new();
        bus.memory.write_byte(address, value);
        assert_eq!(bus.memory_read_byte(address), value);
    }

    #[test]
    pub fn test_memory_write_byte() {
        let address: u16 = 0x410;
        let value: u8 = 0x9;

        let mut bus = Bus::new();
        bus.memory_write_byte(address, value);
        assert_eq!(bus.memory.read_byte(address), value);
    }
}