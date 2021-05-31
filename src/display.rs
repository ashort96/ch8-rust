pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    screen: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Display { screen: [0; WIDTH * HEIGHT]}
    }

    pub fn get_index_from_coords(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut erased = false;
        let mut pos_x = x as usize;
        let mut pos_y = y as usize;
        let mut b = byte;

        for _ in 0..8 {
            pos_x %= WIDTH;
            pos_y %= HEIGHT;
            let index = Display::get_index_from_coords(pos_x, pos_y);
            let bit = (b & 0x80) >> 7;
            let prev_value = self.screen[index];
            self.screen[index] ^= bit;

            if prev_value == 1 && self.screen[index] == 0 {
                erased = true;
            }

            pos_x += 1;
            b <<= 1;
        }

        erased
    }

    pub fn clear(&mut self) {
        for pixel in self.screen.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn get_display_buffer(&self) -> &[u8] {
        &self.screen
    }
}