use crate::cpu::{Cpu, PROGRAM_START_ADDRESS};
use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::ram::Ram;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
const OPCODE_SIZE: u16 = 2;

enum Opcode {
    /// Clears the screen.
    _00e0,
    /// Returns from a subroutine.
    _00ee,
    /// Jumps to address NNN.
    _1nnn(u16),
    /// Calls subroutine at NNN.
    _2nnn(u16),
    /// Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block);
    _3xnn(u8, u8),
    /// Skips the next instruction if VX does not equal NN. (Usually the next instruction is a jump to skip a code block);
    _4xnn(u8, u8),
    /// Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block);
    _5xy0(u8, u8),
    /// Sets VX to NN.
    _6xnn(u8, u8),
    /// Adds NN to VX. (Carry flag is not changed)
    _7xnn(u8, u8),
    /// Sets VX to the value of VY.
    _8xy0(u8, u8),
    /// Sets VX to VX or VY. (Bitwise OR operation)
    _8xy1(u8, u8),
    /// Sets VX to VX and VY. (Bitwise AND operation)
    _8xy2(u8, u8),
    /// Sets VX to VX xor VY. (Bitwise XOR operation)
    _8xy3(u8, u8),
    /// Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
    _8xy4(u8, u8),
    /// VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
    _8xy5(u8, u8),
    /// Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift.
    _8xy6(u8, u8),
    /// Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
    _8xy7(u8, u8),
    /// Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.
    _8xye(u8, u8),
    /// Skips the next instruction if VX doesn't equal VY. (Usually the next instruction is a jump to skip a code block);
    _9xy0(u8, u8),
    /// Sets I to the address NNN.
    _Annn(u16),
    /// Jumps to the address NNN plus V0.
    _Bnnn(u16),
    /// Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
    _Cxnn(u8, u8),
    /// Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value doesn't change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that doesn't happen.
    _Dxyn(u8, u8, u8),
    /// Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block);
    _Ex9e(u8),
    /// Skips the next instruction if the key stored in VX isn't pressed. (Usually the next instruction is a jump to skip a code block);
    _Exa1(u8),
    /// Sets VX to the value of the delay timer.
    _Fx07(u8),
    /// A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event);
    _Fx0a(u8),
    /// Sets the delay timer to VX.
    _Fx15(u8),
    /// Sets the sound timer to VX.
    _Fx18(u8),
    /// Adds VX to I. (VF is not affected)
    _Fx1e(u8),
    /// Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.;
    _Fx29(u8),
    /// Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.);
    _Fx33(u8),
    /// Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.
    _Fx55(u8),
    /// Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.
    _Fx65(u8),
}

#[derive(Debug)]
struct UnknownOpcodeError((u16, u16, u16, u16));

impl TryFrom<u16> for Opcode {
    type Error = UnknownOpcodeError;

    fn try_from(hex_opcode: u16) -> Result<Self, Self::Error> {
        use Opcode::*;
        let hex_nibbles = (
            ((hex_opcode & 0xF000) >> 12),
            ((hex_opcode & 0x0F00) >> 8),
            ((hex_opcode & 0x00F0) >> 4),
            ((hex_opcode & 0x000F) >> 2),
        );

        let nnn = hex_opcode & 0x0FFF;
        let nn = (hex_opcode & 0x00FF) as u8;
        let x = hex_nibbles.1 as u8;
        let y = hex_nibbles.2 as u8;
        let n = hex_nibbles.3 as u8;

        match hex_nibbles {
            (0, 0, 0xE, 0) => Ok(_00e0),
            (0, 0, 0xE, 0xE) => Ok(_00ee),
            (1, _, _, _) => Ok(_1nnn(nnn)),
            (2, _, _, _) => Ok(_2nnn(nnn)),
            (3, _, _, _) => Ok(_3xnn(x, nn)),
            (4, _, _, _) => Ok(_4xnn(x, nn)),
            (5, _, _, _) => Ok(_5xy0(x, y)),
            (6, _, _, _) => Ok(_6xnn(x, nn)),
            (7, _, _, _) => Ok(_7xnn(x, nn)),
            (8, _, _, 0) => Ok(_8xy0(x, y)),
            (8, _, _, 1) => Ok(_8xy1(x, y)),
            (8, _, _, 2) => Ok(_8xy2(x, y)),
            (8, _, _, 3) => Ok(_8xy3(x, y)),
            (8, _, _, 4) => Ok(_8xy4(x, y)),
            (8, _, _, 5) => Ok(_8xy5(x, y)),
            (8, _, _, 6) => Ok(_8xy6(x, y)),
            (8, _, _, 7) => Ok(_8xy7(x, y)),
            (8, _, _, 0xE) => Ok(_8xye(x, y)),
            (9, _, _, 0) => Ok(_9xy0(x, y)),
            (0xA, _, _, _) => Ok(_Annn(nnn)),
            (0xB, _, _, _) => Ok(_Bnnn(nnn)),
            (0xC, _, _, _) => Ok(_Cxnn(x, nn)),
            (0xD, _, _, _) => Ok(_Dxyn(x, y, n)),
            (0xE, _, 9, 0xE) => Ok(_Ex9e(x)),
            (0xE, _, 0xA, 0x1) => Ok(_Exa1(x)),
            (0xF, _, 0, 7) => Ok(_Fx07(x)),
            (0xF, _, 0, 0xA) => Ok(_Fx0a(x)),
            (0xF, _, 3, 3) => Ok(_Fx33(x)),
            (0xF, _, 5, 5) => Ok(_Fx55(x)),
            (0xF, _, 6, 5) => Ok(_Fx65(x)),
            _ => Err(UnknownOpcodeError(hex_nibbles)),
        }
    }
}

#[derive(Debug)]
pub struct Chip8 {
    ram: Ram,
    cpu: Cpu,
    display: Display,
    keyboard: Keyboard,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            ram: Ram::new(),
            cpu: Cpu::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.ram.load_fontset(FONT_SET);
    }

    pub fn load(&mut self, rom: &[u8]) {
        for (i, byte) in rom.iter().enumerate() {
            self.ram
                .write(PROGRAM_START_ADDRESS as u16 + i as u16, *byte);
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = self.fetch_and_decode().unwrap();
        self.run_instruction(opcode);
    }

    fn fetch_and_decode(&mut self) -> Result<Opcode, UnknownOpcodeError> {
        // Grab first half of the opcode
        let lo = self.ram.read(self.cpu.pc) as u16;
        // Grab second half of the opcode
        let hi = self.ram.read(self.cpu.pc + 1) as u16;
        // Combine them
        let hex_opcode = hi << 8 | lo;
        self.cpu.pc += OPCODE_SIZE;
        Opcode::try_from(hex_opcode)
    }

    fn run_instruction(&mut self, opcode: Opcode) {
        let ram = &mut self.ram;
        let cpu = &mut self.cpu;
        let display = &mut self.display;
        let keyboard = &mut self.keyboard;

        // let opcode = self.fetch_opcode(&ram).unwrap();
        use Opcode::*;
        match opcode {
            _00e0 => display.clear(),
            _00ee => {
                // When we enter in a subroutine, we push the current address to the stack.
                // so to exit it, we just need to pop the last address from the stack and set the PC to it.
                let return_addr = cpu.stack_pop();
                cpu.pc = return_addr;
            }
            _1nnn(nnn) => cpu.pc = nnn,
            _2nnn(nnn) => {
                // To enter in a subroutine, we push the current address to the stack.
                cpu.stack_push(cpu.pc);
                cpu.pc = nnn;
            }
            _3xnn(x, nn) => {
                let vx = cpu.read_register(x);
                if vx == nn {
                    cpu.pc += OPCODE_SIZE;
                }
            }
            _4xnn(x, nn) => {
                let vx = cpu.read_register(x);
                if vx != nn {
                    cpu.pc += OPCODE_SIZE;
                }
            }
            _5xy0(x, y) => {
                let vx = cpu.read_register(x);
                let vy = cpu.read_register(y);
                if vx == vy {
                    cpu.pc += OPCODE_SIZE;
                }
            }
            _6xnn(x, nn) => cpu.write_register(x, nn),
            _7xnn(x, nn) => {
                let vx = cpu.read_register(x);
                cpu.write_register(x, vx.wrapping_add(nn));
            }
            _8xy0(x, y) => {
                let vy = cpu.read_register(y);
                cpu.write_register(x, vy);
            }
            _8xy1(x, y) => {
                let or = cpu.read_register(x) | cpu.read_register(y);
                cpu.write_register(x, or);
            }
            _8xy2(x, y) => {
                let and = cpu.read_register(x) & cpu.read_register(y);
                cpu.write_register(x, and);
            }
            _8xy3(x, y) => {
                let xor = cpu.read_register(x) ^ cpu.read_register(y);
                cpu.write_register(x, xor);
            }
            _8xy4(x, y) => {
                let vx = cpu.read_register(x);
                let vy = cpu.read_register(y);
                let (sum, overflow) = vx.overflowing_add(vy);
                cpu.write_register(x, sum);
                cpu.write_register(0xF, overflow as u8);
            }
            _8xy5(x, y) => {
                let vx = cpu.read_register(x);
                let vy = cpu.read_register(y);
                let (sub, overflow) = vx.overflowing_sub(vy);
                cpu.write_register(x, sub);
                cpu.write_register(0xF, overflow as u8);
            }
            _8xy6(x, _) => {
                let vx = cpu.read_register(x);
                let least_significant_bit = vx & 1;
                cpu.write_register(0xF, least_significant_bit);
                cpu.write_register(x, vx >> 1);
            }
            _8xy7(x, y) => {
                let vx = cpu.read_register(x);
                let vy = cpu.read_register(y);
                let (sub, overflow) = vy.overflowing_sub(vx);
                cpu.write_register(x, sub);
                cpu.write_register(0xF, overflow as u8);
            }
            _8xye(x, _) => {
                let vx = cpu.read_register(x);
                let most_significant_bit = (vx >> 7) & 1;
                cpu.write_register(0xF, most_significant_bit);
                cpu.write_register(x, vx << 1);
            }
            _9xy0(x, y) => {
                let vx = cpu.read_register(x);
                let vy = cpu.read_register(y);
                if vx != vy {
                    cpu.pc += OPCODE_SIZE;
                }
            }
            _Annn(nnn) => cpu.i = nnn,
            _Bnnn(nnn) => {
                let v0 = cpu.read_register(0);
                cpu.pc = nnn + v0 as u16;
            }
            _Cxnn(x, nn) => {
                let random_number = rand::random::<u8>();
                cpu.write_register(x, random_number & nn);
            }
            _Dxyn(_, _, _) => todo!(), // draw
            _Ex9e(x) => {
                let vx = cpu.read_register(x);
                if keyboard.is_pressed(vx) {
                    cpu.pc += OPCODE_SIZE;
                }
            }
            _Exa1(x) => {
                let vx = cpu.read_register(x);
                if !keyboard.is_pressed(vx) {
                    cpu.pc += OPCODE_SIZE;
                }
            }
            _Fx07(x) => cpu.write_register(x, cpu.delay_timer),
            _Fx0a(x) => {
                // Wait for a key press, then store it in VX
                let mut pressed = false;
                for (key, is_pressed) in keyboard.iter().enumerate() {
                    if *is_pressed {
                        cpu.write_register(x, key as u8);
                        pressed = true;
                        break;
                    }
                }
                // If no key is pressed, we need to repeat the instruction
                if !pressed {
                    cpu.pc -= OPCODE_SIZE;
                }
            }
            _Fx15(x) => cpu.delay_timer = cpu.read_register(x),
            _Fx18(x) => cpu.sound_timer = cpu.read_register(x),
            _Fx1e(x) => {
                let vx = cpu.read_register(x);
                cpu.i = cpu.i.wrapping_add(vx as u16);
            }
            _Fx29(x) => {
                let vx = cpu.read_register(x);
                // Each character is 5 bytes long, so we multiply by 5
                cpu.i = vx as u16 * 5;
            }
            _Fx33(x) => {
                let vx = cpu.read_register(x);
                let hundreds = vx / 100;
                let tens = (vx % 100) / 10;
                let ones = vx % 10;
                ram.write(cpu.i, hundreds);
                ram.write(cpu.i + 1, tens);
                ram.write(cpu.i + 2, ones);
            }
            _Fx55(x) => {
                for reg in 0..=x {
                    let value = cpu.read_register(reg);
                    let idx = cpu.i + reg as u16;
                    ram.write(idx, value);
                }
            }
            _Fx65(x) => {
                for reg in 0..=x {
                    let idx = cpu.i + reg as u16;
                    let value = ram.read(idx);
                    cpu.write_register(reg, value);
                }
            }
        };
    }
}
