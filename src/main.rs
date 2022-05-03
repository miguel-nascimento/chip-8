use chip_8::{cpu::Cpu, ram::Ram};

fn main() {
  let mut ram = Ram::new();
  let mut cpu = Cpu::new();
  ram.write(0x200, 0x0AF);
  cpu.run_instruction(&mut ram);
}