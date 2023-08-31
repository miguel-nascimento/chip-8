const MEMORY_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Ram {
    memory: [u8; MEMORY_SIZE],
}

impl Default for Ram {
    fn default() -> Self {
        Ram {
            memory: [0; MEMORY_SIZE],
        }
    }
}

impl Ram {
    pub fn load_fontset(&mut self, font_set: [u8; 80]) {
        for i in 0..font_set.len() {
            self.memory[i] = font_set[i]
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) -> () {
        self.memory[addr as usize] = value;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
}
