use crate::ram::Ram;

const STACK_SIZE: usize = 0x10; // 16
const PROGRAM_START_ADDRESS: u16 = 0x200; // 512 
const OPCODE_SIZE: u16 = 2;

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

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

impl ProgramCounter {
  fn skip_if(condition: bool) -> Self {
    if condition {
      ProgramCounter::Skip
    } else {
      ProgramCounter::Next
    }
  }
}

enum Opcode {
  OP_00E0,          // Clears the screen.
  OP_8XY4(u8, u8),  // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
  OP_8XY5(u8, u8),  // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
  OP_8XY6(u8, u8),  // Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift.[2]
  OP_8XY7(u8, u8),  // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
  OP_8XYE(u8, u8),  // Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.[2]
  OP_9XY0(u8, u8),  // Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block);
  OP_CXNN(u8),      // Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
  OP_DXYN(u8, u8),  // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn't change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn't happen.
  OP_EX9E(u8),      // Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block);
  OP_EXA1(u8),      // Skips the next instruction if the key stored in VX isn't pressed. (Usually the next instruction is a jump to skip a code block);
  OP_FX0A(u8),      // A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event);
  OP_FX33(u8),      // Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.);
  OP_FX55(u8),      // Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
  OP_FX65(u8)       // Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
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

  fn read_register(&self, register: u8) -> u8 {
    self.register[register as usize]
  }

  fn write_register(&mut self, register: u8, value: u8) {
    self.register[register as usize] = value;
  }

  pub fn run_instruction(&mut self, ram: &mut Ram) {
    let opcode = self.fetch_opcode(&ram);
    // Explanation of the AND operation:
    let hex_nibbles = (
      ((opcode & 0xF000) >> 12) as u8,
      ((opcode & 0x0F00) >> 8) as u8,
      ((opcode & 0x00F0) >> 4) as u8,
      ((opcode & 0x000F) >> 2) as u8,
    );

    let nnn = opcode & 0x0FFF;
    let nn = (opcode & 0x00FF) as u8;
    let x = hex_nibbles.1;
    let y = hex_nibbles.2;
    let n = hex_nibbles.3;

    let pc_operation: ProgramCounter = match hex_nibbles {
      (0x00, 0x00, 0x0E, 0x00) => todo!(),
      (0x00, 0x00, 0x0E, 0x0E) => {
        // FIXME: maybe this will not work
        // Returns from a subroutine.
        // Pop the last address from the stack and setting the PC to it.
        ProgramCounter::Jump(self.stack[self.sp as usize - 1])
      }
      (0x01, _, _,_)           => ProgramCounter::Jump(nnn), // Jumps to address NNN.
      (0x02, _, _, _)          => {
        // FIXME: maybe this will not work
        // Calls subroutine at NNN.
        // Push the current PC to the stack, so the subroutine can return later.
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        ProgramCounter::Jump(nnn)
        
      }
      (0x03,_, _, _)           => {
        // Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block);
        let vx = self.read_register(x);
        ProgramCounter::skip_if(vx == nn)
      }
      (0x04,_, _, _)           => {
        // Skips the next instruction if VX does not equal NN. (Usually the next instruction is a jump to skip a code block);
        let vx = self.read_register(x);
        ProgramCounter::skip_if(vx != nn)
      }
      (0x05, _, _, _)          => {
        // Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block);
        let vx = self.read_register(x);
        let vy = self.read_register(y);
        ProgramCounter::skip_if(vx == vy)
      }
      (0x06, _, _, _)          => {
        // Sets VX to NN.
        self.register[x as usize] = nn;
        println!("Register[{:#X}] = {:#X}", x, nn);
        ProgramCounter::Next
      }
      (0x07, _, _,_)           => {
        // Adds NN to VX. (Carry flag is not changed)
        self.write_register(x, self.read_register(x) + nn);
        ProgramCounter::Next
      }
      (0x08, _,_, 0x00)        => {
        // Sets VX to the value of VY.
        self.write_register(x, self.read_register(y)); 
        ProgramCounter::Next
      }
      (0x08, _,_, 0x01)        => {
        // Sets VX to VX or VY. (Bitwise OR operation)
        let or = self.read_register(x) | self.read_register(y);
        self.write_register(x, or); 
        ProgramCounter::Next
      }
      (0x08, _,_, 0x02)        => {
        // Sets VX to VX and VY. (Bitwise AND operation) 
        let and = self.read_register(x) & self.read_register(y);
        self.write_register(x, and);
        ProgramCounter::Next
      }
      (0x08, _,_, 0x03)        => {
        // Sets VX to VX xor VY. (Bitwise XOR operation)
        let xor = self.read_register(x) ^ self.read_register(y);
        self.write_register(x, xor);
        ProgramCounter::Next
      }
      (0x08, _,_, 0x04)        => todo!(),
      (0x08, _,_, 0x05)        => todo!(),
      (0x08, _,_, 0x06)        => todo!(),
      (0x08, _,_, 0x07)        => todo!(),
      (0x08, _,_, 0x0E)        => todo!(),
      (0x09, _,_, 0x00)        => todo!(),
      (0x0A, _, _,_)           => {
        // Sets I to the address NNN.
        self.i = nnn;
        ProgramCounter::Next
      }
      (0x0B, _, _,_)           => {
        // Jumps to the address NNN plus V0.
        self.pc = nnn + self.read_register(0x0) as u16;
        ProgramCounter::Next
      }
      (0x0C, _, _,_)           => todo!(),
      (0x0D, _, _,_)           => todo!(),
      (0x0E, _, 0x09, 0x0E)    => todo!(),
      (0x0E, _, 0x0A, 0x01)    => todo!(),
      (0x0F, _, 0x00, 0x07) => {
        // Sets VX to the value of the delay timer.
        self.write_register(x, self.delay_timer);
        ProgramCounter::Next
      }
      (0x0F, _, 0x00, 0x0A)    => todo!(),
      (0x0F, _, 0x01, 0x05)    => {
        // Sets the delay timer to VX.
        self.delay_timer = self.read_register(x);
        ProgramCounter::Next
      }
      (0x0F, _, 0x01, 0x08)    => {
        // Sets the sound timer to VX.
        self.sound_timer = self.read_register(x);
        ProgramCounter::Next
      }
      (0x0F, _, 0x01, 0x0E)    => {
        // Adds VX to I. VF is not affected.
        self.i += self.read_register(x) as u16;
        ProgramCounter::Next
      }
      (0x0F, _, 0x02, 0x09)    => {
        // Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
        self.i = nnn + self.read_register(x) as u16;
        ProgramCounter::Next
      }
      (0x0F, _, 0x03, 0x03)    => todo!(),
      (0x0F, _, 0x05, 0x05)    => todo!(),
      (0x0F, _, 0x06, 0x05)    => todo!(),
      _ => panic!("Unknown opcode: {:#X}", opcode)
    };

    match pc_operation {
      ProgramCounter::Next => self.pc += OPCODE_SIZE,
      ProgramCounter::Skip => self.pc += OPCODE_SIZE * 2,
      ProgramCounter::Jump(address) => self.pc = address
    }
  }
}
