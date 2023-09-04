use std::ops::Deref;

const NUM_KEYS: usize = 16;

#[derive(Debug, Clone)]
pub struct Keyboard {
    keys: [bool; NUM_KEYS],
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard {
            keys: [false; NUM_KEYS],
        }
    }
}

impl Keyboard {
    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn press(&mut self, key: u8, pressed: bool) {
        self.keys[key as usize] = pressed;
    }

    pub fn clean(&mut self) {
        self.keys = [false; NUM_KEYS];
    }
}

impl Deref for Keyboard {
    type Target = [bool; NUM_KEYS];

    fn deref(&self) -> &Self::Target {
        &self.keys
    }
}
