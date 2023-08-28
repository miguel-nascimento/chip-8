use std::ops::Deref;

const NUM_KEYS: usize = 16;

#[derive(Debug)]
pub struct Keyboard {
    keys: [bool; NUM_KEYS],
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            keys: [false; NUM_KEYS],
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }
}

impl Deref for Keyboard {
    type Target = [bool; NUM_KEYS];

    fn deref(&self) -> &Self::Target {
        &self.keys
    }
}
