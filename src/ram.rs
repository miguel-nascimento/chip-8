const MEMORY_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Ram {
  pub memory: [u8; MEMORY_SIZE],
}

impl Ram {
  pub fn new() -> Self {
    Ram { memory: [0; 4096] }
  }

  pub fn load_fontset(&mut self, font_set: [u8; 80]) {
    for i in 0..font_set.len() {
      self.memory[i] = font_set[i]
    }
  }

  pub fn write(&mut self, addr: u16, value: u8) -> () { 
    // YES, IN THE ORIGINAL CHIP 8 THIS CAN OCCUR
    if addr > MEMORY_SIZE as u16 {
      panic!("write_memory: address is bigger than the memory size")
    }
    self.memory[addr as usize] = value;

  }

  pub fn read(&self, addr: u16) -> u8 {
    if addr > MEMORY_SIZE as u16 {
      panic!("read_memory: address is bigger than the memory size")
    }

    self.memory[addr as usize]
  }
}
