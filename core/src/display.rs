pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Display {
    // 64px wide and 32px tall
    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Default for Display {
    fn default() -> Self {
        Display {
            screen: [false; 64 * 32],
        }
    }
}

impl Display {
    pub fn clear(&mut self) {
        self.screen = [false; 64 * 32];
    }

    pub fn to_xy(&self, idx: usize) -> (usize, usize) {
        (idx % SCREEN_WIDTH, idx / SCREEN_WIDTH)
    }
}
