use crate::ram::Ram;

const STACK_SIZE: usize = 0x10; // 16
const PROGRAM_START_ADDRESS: u16 = 0x200; // 512 

#[derive(Debug)]
pub struct Cpu {
  stack: [u16; STACK_SIZE],
  delay_timer: u8,
  sound_timer: u8,
  pc: u16,
  sp: u8,
  i: u16,
  register: [u8; 16],
}

impl Cpu {
  pub fn new() -> Self {
    Cpu {
      stack: [0; STACK_SIZE],
      delay_timer: 0,
      sound_timer: 0,
      pc: PROGRAM_START_ADDRESS,
      sp: 0,
      i: 0,
      register: [0; 16],
    }
  }

  fn fetch_opcode(&self, ram: &Ram) -> u16 {
    let lo = ram.memory[self.pc as usize];
    let hi = ram.memory[(self.pc + 1) as usize];
    (hi as u16) << 8 as u8 | lo as u16
    
  } 

  pub fn run_instruction(&mut self, ram: &mut Ram) {
    let opcode = self.fetch_opcode(&ram);
    println!("{:?}", opcode);
    // const pc_option = match opcode & 0xFFF { }
  }
}