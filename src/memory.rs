/// CHIP-8 Memory Layout ([credits](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap))
/// ```text
/// +---------------+ = 0xFFF (4095) End of Chip-8 RAM
/// |               |
/// |               |
/// |               |
/// |               |
/// |               |
/// | 0x200 to 0xFFF|
/// |     Chip-8    |
/// | Program / Data|
/// |     Space     |
/// |               |
/// |               |
/// |               |
/// +- - - - - - - -+ = 0x600 (1536) Start of ETI 660 Chip-8 programs
/// |               |
/// |               |
/// |               |
/// +---------------+ = 0x200 (512) Start of most Chip-8 programs
/// | 0x000 to 0x1FF|
/// | Reserved for  |
/// |  interpreter  |
/// +---------------+ = 0x000 (0) Start of Chip-8 RAM
/// ```

pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {

    /// Creates a new instance of Memory and initializes the values
    pub fn new() -> Self {
        
        // Create a new instance of Self
        let mut memory = Memory {
            ram: [0; 4096],

        };

        // Create the sprites
        let sprites: [[u8; 5]; 16] = [
            [0xf0, 0x90, 0x90, 0x90, 0xf0], // "0"
            [0x20, 0x60, 0x20, 0x20, 0x70], // "1"
            [0xf0, 0x10, 0xf0, 0x80, 0xf0], // "2"
            [0xf0, 0x10, 0xf0, 0x10, 0xf0], // "3"
            [0x90, 0x90, 0xf0, 0x10, 0x10], // "4"
            [0xf0, 0x80, 0xf0, 0x10, 0xf0], // "5"
            [0xf0, 0x80, 0xf0, 0x90, 0xf0], // "6"
            [0xf0, 0x10, 0x20, 0x40, 0x40], // "7"
            [0xf0, 0x90, 0xf0, 0x90, 0xf0], // "8"
            [0xf0, 0x90, 0xf0, 0x10, 0xf0], // "9"
            [0xf0, 0x90, 0xf0, 0x90, 0x90], // "A"
            [0xe0, 0x90, 0xe0, 0x90, 0xe0], // "B"
            [0xf0, 0x80, 0x80, 0x80, 0xf0], // "C"
            [0xe0, 0x90, 0x90, 0x90, 0xe0], // "D"
            [0xf0, 0x80, 0xf0, 0x80, 0xf0], // "E"
            [0xf0, 0x80, 0xf0, 0x80, 0x80]  // "F"
        ];

        // Load the sprites into memory
        let mut i: u16 = 0;
        for sprite in &sprites {
            for element in sprite {
                memory.write_byte(i, *element);
                i += 1;
            }
        }

        memory
    }


    /// Read a byte from memory given the address. While the address is a u16,
    /// we mask it with 0xfff since the address space max is 0xfff
    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[(address & 0xfff) as usize]
    }

    /// Write a byte to memory given the address. While the address is a u16,
    /// we mask it with 0xfff since the address space max is 0xfff
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[(address & 0xfff) as usize] = value;
    }

}