#[derive(Debug)]
pub struct Display {
  // 64px wide and 32px tall
  screen: [u8; 64 * 32]
}

impl Display {
  pub fn new() -> Self {
    Display {
      screen: [0; 64 * 32]
    }
  }
}