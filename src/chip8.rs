use crate::bus::Bus;
use crate::cpu;

pub struct Chip8 {
    pub bus: Bus,
    pub cpu: cpu::Cpu,
}

impl Chip8 {

    /// Create a new Chip8 instance
    pub fn new() -> Self {
        Chip8 {
            bus: Bus::new(),
            cpu: cpu::Cpu::new(),
        }
    }

    /// Load a ROM into memory
    /// Method will panic if the data is larger than the space allocated for the ROM
    pub fn load_rom(&mut self, data: &Vec<u8>) {

        assert!(data.len() <= (self.bus.memory_get_size() - (cpu::PROGRAM_START as usize)), "Size of loaded ROM is larger than the allocated RAM of {}", self.bus.memory_get_size());
        assert!(data.len() > 0, "Loaded ROM is empty");

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

#[cfg(test)]
mod test {
    use super::*;

    pub const MAX_ROM_SIZE: usize = 0x1000 - 0x200;

    #[test]
    #[should_panic]
    pub fn overflow_load_rom() {
        let data: Vec<u8> = vec![0; MAX_ROM_SIZE + 1];
        let mut chip8 = Chip8::new();
        chip8.load_rom(&data);
    }

    #[test]
    #[should_panic]
    pub fn empty_load_rom() {
        let data: Vec<u8> = vec![0; 0];
        let mut chip8 = Chip8::new();
        chip8.load_rom(&data);
    }

    #[test]
    pub fn max_load_rom() {
        let data: Vec<u8> = vec![3; MAX_ROM_SIZE];
        let mut chip8 = Chip8::new();
        chip8.load_rom(&data);

        for i in 0..(MAX_ROM_SIZE) {
            assert_eq!(chip8.bus.memory_read_byte(cpu::PROGRAM_START + i as u16), 3);
        }
    }
}