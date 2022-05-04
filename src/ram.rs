const MEMORY_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Ram {
  pub memory: [u16; MEMORY_SIZE],
}

impl Ram {
  pub fn new() -> Self {
    Ram { memory: [0; 4096] }
  }

  pub fn load_fontset(&mut self, font_set: [u16; 80]) {
    for i in 0..font_set.len() {
      self.memory[i] = font_set[i]
    }
  }

  pub fn write(&mut self, addr: u16, value: u16) -> () { 
    self.memory[addr as usize] = value;
  }

  pub fn read(&self, addr: u16) -> u16 {
    self.memory[addr as usize]
  }
}
