const STACK_SIZE: usize = 0x10; // 16
pub const PROGRAM_START_ADDRESS: u16 = 0x200; // 512

#[derive(Debug)]
pub struct Cpu {
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub pc: u16,
    pub sp: u8,
    pub i: u16,
    stack: [u16; STACK_SIZE],
    register: [u8; 16],
}

impl Default for Cpu {
    fn default() -> Cpu {
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
}

impl Cpu {
    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("todo: sound beep!");
            }
            self.sound_timer -= 1;
        }
    }

    pub fn read_register(&self, register: u8) -> u8 {
        self.register[register as usize]
    }

    pub fn write_register(&mut self, register: u8, value: u8) {
        self.register[register as usize] = value;
    }

    pub fn stack_push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    pub fn stack_pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}
