use crate::bus::Bus;
use crate::cpu;
use crate::cpu::Cpu;

pub struct Chip8 {
    pub bus: Bus,
    pub cpu: Cpu,
}

impl Chip8 {

    /// Create a new Chip8 instance
    pub fn new() -> Self {
        Chip8 {
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }


    /// Load a ROM into memory
    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for index in 0..data.len() {
            self.bus.memory_write_byte(cpu::PROGRAM_START + index as u16, data[index]);
        }

    }

    /// Run CPU instruction
    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.bus);
    }

    /// Get the display buffer
    pub fn get_display_buffer(&self) -> &[u8] {
        self.bus.get_display_buffer()
    }

    /// Set the key being pressed
    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.bus.set_key_pressed(key);
    }
}