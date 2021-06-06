use crate::bus::Bus;

use rand::Rng;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    rng: rand::rngs::ThreadRng,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            v: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            stack: [0; 16],
            sp: 0,
            rng: rand::thread_rng()
        }
    }

    pub fn run_instruction(&mut self, bus: &mut Bus) {

        let lo = bus.memory_read_byte(self.pc) as u16;
        let hi = bus.memory_read_byte(self.pc + 1) as u16;
        let instruction: u16 = (lo << 8) | hi;

        let nnn: u16 = instruction & 0xfff;
        let n: u8 = (instruction & 0xf) as u8;
        let x: u8 = ((instruction & 0x0f00) >> 8) as u8;
        let y: u8 = ((instruction & 0x00f0) >> 4) as u8;
        let kk: u8 = (instruction & 0xff) as u8;

        match (instruction >> 12) & 0xf {
            0x0 => {
                match instruction & 0xfff {
                    // Clear the display
                    0x0e0 => {
                        bus.clear_screen();
                        self.pc += 2;
                    }
                    // Returns from a subroutine
                    0x0ee => {
                        self.pc = self.stack[self.sp as usize];
                        self.sp -= 1;
                    }
                    _ => {
                        self.pc = nnn;
                    }
                }

            // Jump to location nnn
            }
            0x1 => {
                self.pc = nnn;
            }
            // Call subroutine at nnn
            0x2 => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc + 2;
                self.pc = nnn;
            }
            // Skip next instruction if Vx = kk
            0x3 => {
                if self.read_reg(x) == kk {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            }
            // Skip next instruction if Vx != kk
            0x4 => {
                if self.read_reg(x) != kk {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            }
            // Skip next instruction if Vx == Vx
            0x5 => {
                assert_eq!(0, n);
                if self.read_reg(x) == self.read_reg(y) {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            }
            // Set Vx = kk
            0x6 => {
                self.write_reg(x, kk);
                self.pc += 2;
            }
            // Set Vx = Vx + kk
            0x7 => {
                let (value, _) = self.read_reg(x).overflowing_add(kk);
                self.write_reg(x, value);
                self.pc += 2;
            }
            0x8 => {
                match instruction & 0xf {
                    // Set Vx = Vy
                    0x0 => {
                        self.write_reg(x, self.read_reg(y));
                        self.pc += 2;
                    }
                    // Set Vx = Vx | Vy
                    0x1 => {
                        self.write_reg(x, self.read_reg(x) | self.read_reg(y));
                        self.pc += 2;
                    }
                    // Set Vx = Vx & Vy
                    0x2 => {
                        self.write_reg(x, self.read_reg(x) & self.read_reg(y));
                        self.pc += 2;
                    }
                    // Set Vx ^ Vy
                    0x3 => {
                        self.write_reg(x, self.read_reg(x) ^ self.read_reg(y));
                        self.pc += 2;
                    }
                    // Set Vx = Vx + Vy, set VF = carry
                    0x4 => {
                        let (value, flag) = self.read_reg(x).overflowing_add(self.read_reg(y));
                        self.write_reg(x, value);
                        self.write_flag_reg(if flag {1} else {0});
                        self.pc += 2;
                    }
                    // Set Vx = Vx - Vy, set VF = NOT borrow
                    0x5 => {
                        let (value, flag) = self.read_reg(x).overflowing_sub(self.read_reg(y));
                        self.write_reg(x, value);
                        self.write_flag_reg(if flag {0} else {1});
                        self.pc += 2;
                    }
                    // Set Vx = Vx SHR 1
                    0x6 => {
                        self.write_flag_reg(self.read_reg(x) & 0x1);
                        self.write_reg(x, self.read_reg(x) >> 1);
                        self.pc += 2;
                    }
                    // Set Vx = Vy - Vx, set VF = NOT borrow
                    0x7 => {
                        let (value, _) = self.read_reg(y).overflowing_sub(self.read_reg(x));
                        self.write_flag_reg(if self.read_reg(y) > self.read_reg(x) {1} else {0});
                        self.write_reg(x, value);
                        self.pc += 2;
                    }
                    // Set Vx = Vx SHL 1
                    0xE => {
                        self.write_flag_reg(if (self.read_reg(x) & 0x80) >> 7 == 1 {1} else {0});
                        self.write_reg(x, self.read_reg(x) << 1);
                        self.pc += 2;
                    }
                    _ => unreachable!()
                }
            }
            // Skip next instruction if Vx != Vy
            0x9 => {
                assert_eq!(0, n);
                if self.read_reg(x) != self.read_reg(y) {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            }
            // Set I = nnn
            0xA => {
                self.i = nnn;
                self.pc += 2;
            }
            // Jump to location nnn + V0
            0xB => {
                self.pc = nnn + self.read_reg(0) as u16;
            }
            // Set Vx = random byte AND kk
            0xC => {
                let random_value: u8 = self.rng.gen_range(0..0xff);
                self.write_reg(x, random_value & kk);
                self.pc += 2;
            }
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
            0xD => {
                self.draw_sprite(bus, self.read_reg(x), self.read_reg(y), n);
                self.pc += 2;
            }
            0xE => {
                match hi {
                    // Skip next instruction if key with value of Vx is pressed
                    0x9e => {
                        let key = self.read_reg(x);
                        if bus.is_key_pressed(key) {
                            self.pc += 4;
                        }
                        else {
                            self.pc += 2;
                        }
                    }
                    // Skip the next instruction if the key with the value of Vx is not pressed
                    0xA1 => {
                        let key = self.read_reg(x);
                        if !bus.is_key_pressed(key) {
                            self.pc += 4;
                        }
                        else {
                            self.pc += 2;
                        }
                    }
                    _ => unreachable!()
                }
            }
            0xF => {
                match hi {
                    // Set Vx = delay timer value
                    0x07 => {
                        self.write_reg(x, bus.get_delay_timer());
                        self.pc += 2;
                    }
                    // Wait for a key press, store the value of the key in Vx
                    0x0A => {
                        if let Some(val) = bus.get_key_pressed() {
                            self.write_reg(x, val);
                        }
                        self.pc += 2;
                    }
                    // Set delay timer = Vx
                    0x15 => {
                        bus.set_delay_timer(self.read_reg(x));
                        self.pc += 2;
                    }
                    // Set sound timer = Vx
                    0x18 => {
                        // TODO Sound Timer
                        self.pc += 2;
                    }
                    // Set I = I + Vx
                    0x1E => {
                        self.i = self.i + self.read_reg(x) as u16;
                        self.pc += 2;
                    }
                    // Set I = location of sprite for digit Vx
                    0x29 => {
                        self.i = self.read_reg(x) as u16 * 5;
                        self.pc += 2;
                    }
                    // Store BCD representation of Vx in memory locations I, I+1, I+2
                    0x33 => {
                        let val = self.read_reg(x);
                        bus.memory_write_byte(self.i, val / 100);
                        bus.memory_write_byte(self.i + 1, (val % 100) / 10);
                        bus.memory_write_byte(self.i + 2, val % 10);
                        self.pc += 2;
                    }
                    // Store registers V0 through Vx in memory starting at location I
                    0x55 => {
                        for index in 0..=x {
                            bus.memory_write_byte(self.i + index as u16, self.read_reg(index));
                        }
                        self.i += x as u16 + 1;
                        self.pc += 2;
                    }
                    // Read registers V0 through Vx from memory starting at location I
                    0x65 => {
                        for index in 0..=x {
                            self.write_reg(index, bus.memory_read_byte(self.i + index as u16))
                        }
                        self.pc += 2;
                    }
                    _ => unreachable!("Instruction: {:#x}", instruction)
                }
            }

            _ => panic!("Unhandled or unknown instruction ({:#x})", instruction)
        }


    }

    pub fn write_reg(&mut self, index: u8, value: u8) {
        self.v[index as usize] = value;
    }

    pub fn write_flag_reg(&mut self, value: u8) {
        self.write_reg(15, value);
    }

    pub fn read_reg(&self, index: u8) -> u8 {
        self.v[index as usize]
    }

    fn draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8) {
        let mut should_set_vf = false;
        for sprite_y in 0..height {
            let b = bus.memory_read_byte(self.i + sprite_y as u16);
            if bus.draw_byte(b, x, y + sprite_y) {
                should_set_vf = true;
            }
        }
        if should_set_vf {
            self.write_flag_reg(1);
        } else {
            self.write_flag_reg(0);
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    pub fn put_first_instruction(bus: &mut Bus, instruction: u16) {
        bus.memory_write_byte(0x200, ((instruction & 0xff00) >> 8) as u8);
        bus.memory_write_byte(0x201, (instruction & 0xff) as u8);
    }

    #[test]
    pub fn test_2nnn() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        put_first_instruction(&mut bus, 0x2345);

        cpu.run_instruction(&mut bus);

        assert_eq!(0x345, cpu.pc);
        assert_eq!(1, cpu.sp);
        assert_eq!(previous_pc + 2, cpu.stack[1]);
    }

    #[test]
    pub fn test_3xkk_not_equal() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x44;
        put_first_instruction(&mut bus, 0x3145);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
    }

    #[test]
    pub fn test_3xkk_equal() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x45;
        put_first_instruction(&mut bus, 0x3145);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 4, cpu.pc);
    }

    #[test]
    pub fn test_4xkk_not_equal() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x54;
        put_first_instruction(&mut bus, 0x4155);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 4, cpu.pc);
    }

    #[test]
    pub fn test_4xkk_equal() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x55;
        put_first_instruction(&mut bus, 0x4155);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc); 
    }

    #[test]
    pub fn test_5xy0_not_equal() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x45;
        cpu.v[2] = 0x46;
        put_first_instruction(&mut bus, 0x5120);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
    }

    #[test]
    pub fn test_5xy0_equal() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x46;
        cpu.v[2] = 0x46;
        put_first_instruction(&mut bus, 0x5120);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 4, cpu.pc);   
    }

    #[test]
    #[should_panic]
    pub fn test_5xy0_last_0() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x46;
        cpu.v[2] = 0x46;
        put_first_instruction(&mut bus, 0x5121);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 4, cpu.pc); 
    }

    #[test]
    pub fn test_6xkk() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        put_first_instruction(&mut bus, 0x6145);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x45);
    }

    #[test]
    pub fn test_7xkk() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x30;
        put_first_instruction(&mut bus, 0x7145);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x30 + 0x45);
    }

    #[test]
    pub fn test_8xy0() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[2] = 0x30;
        put_first_instruction(&mut bus, 0x8120);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x30);
    }

    #[test]
    pub fn test_8xy1() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x05;
        cpu.v[2] = 0x30;
        put_first_instruction(&mut bus, 0x8121);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x30 | 0x05);
    }

    #[test]
    pub fn test_8xy2() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x05;
        cpu.v[2] = 0x30;
        put_first_instruction(&mut bus, 0x8122);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x30 & 0x05);
    }

    #[test]
    pub fn test_8xy3() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x05;
        cpu.v[2] = 0x30;
        put_first_instruction(&mut bus, 0x8123);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x30 ^ 0x05);
    }

    #[test]
    pub fn test_8xy4_no_carry() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x06;
        cpu.v[2] = 0x30;
        put_first_instruction(&mut bus, 0x8124);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x30 + 0x06);
        assert_eq!(cpu.v[15], 0);
    }

    #[test]
    pub fn test_8xy4_set_carry() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0xFE;
        cpu.v[2] = 0x03;
        put_first_instruction(&mut bus, 0x8124);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x1);
        assert_eq!(cpu.v[15], 1);
    }

    #[test]
    pub fn test_8xy5_no_borrow() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x31;
        cpu.v[2] = 0x30;
        put_first_instruction(&mut bus, 0x8125);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x31 - 0x30);
        assert_eq!(cpu.v[15], 1);
    }

    #[test]
    pub fn test_8xy5_borrow() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x31;
        cpu.v[2] = 0x32;
        put_first_instruction(&mut bus, 0x8125);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0xFF);
        assert_eq!(cpu.v[15], 0);
    }

    #[test]
    pub fn test_8xy6_lsb_0() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x40;
        put_first_instruction(&mut bus, 0x8126);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x40 >> 1);
        assert_eq!(cpu.v[15], 0);
    }

    #[test]
    pub fn test_8xy6_lsb_1() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x41;
        put_first_instruction(&mut bus, 0x8126);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x41 >> 1);
        assert_eq!(cpu.v[15], 1);
    }

    #[test]
    pub fn test_8xy7_no_borrow() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x40;
        cpu.v[2] = 0x41;
        put_first_instruction(&mut bus, 0x8127);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x1);
        assert_eq!(cpu.v[15], 1);
    }

    #[test]
    pub fn test_8xy7_borrow() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x41;
        cpu.v[2] = 0x40;
        put_first_instruction(&mut bus, 0x8127);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0xFF);
        assert_eq!(cpu.v[15], 0);
    }


    #[test]
    #[allow(non_snake_case)]
    pub fn test_8xyE_msb_0() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x41;
        put_first_instruction(&mut bus, 0x812E);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x41 << 1);
        assert_eq!(cpu.v[15], 0);
    }

    #[test]
    #[allow(non_snake_case)]
    pub fn test_8xyE_msb_1() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x81;
        put_first_instruction(&mut bus, 0x812E);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.v[1], 0x81 << 1);
        assert_eq!(cpu.v[15], 1);
    }

    #[test]
    pub fn test_9xy0_not_equal() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x81;
        cpu.v[2] = 0x82;
        put_first_instruction(&mut bus, 0x9120);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 4, cpu.pc);
    }

    #[test]
    pub fn test_9xy0_equal() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x81;
        cpu.v[2] = 0x81;
        put_first_instruction(&mut bus, 0x9120);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
    }

    #[test]
    #[should_panic]
    pub fn test_9xy0_last_0() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        cpu.v[1] = 0x81;
        cpu.v[2] = 0x81;
        put_first_instruction(&mut bus, 0x9121);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
    }

    #[test]
    #[allow(non_snake_case)]
    pub fn test_Annn() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        let previous_pc = cpu.pc;
        put_first_instruction(&mut bus, 0xA123);

        cpu.run_instruction(&mut bus);

        assert_eq!(previous_pc + 2, cpu.pc);
        assert_eq!(cpu.i, 0x123);
    }

    #[test]
    #[allow(non_snake_case)]
    pub fn test_Bnnn() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        cpu.v[0] = 0x45;
        put_first_instruction(&mut bus, 0xB123);

        cpu.run_instruction(&mut bus);

        assert_eq!(cpu.pc, 0x123 + 0x45);
    }



}