pub struct Keyboard {
    key_pressed: Option<u8>
}

impl Keyboard {

    /// Creates a new Keyboard instance
    pub fn new() -> Self {
        Keyboard{ key_pressed: None }
    }

    /// Returns whether or not the key_code passed is in being pressed
    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        if let Some(key) = self.key_pressed {
            key == key_code
        }
        else {
            false
        }
    }

    /// Set the key being pressed
    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.key_pressed = key;
    }

    /// Get the key being pressed
    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    }
}