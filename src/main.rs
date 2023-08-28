use chip_8::{cpu::Cpu, display::Display, keyboard::Keyboard, ram::Ram};

fn main() {
    let mut ram = Ram::new();
    let mut cpu = Cpu::new();
    let mut display = Display::new();
    let mut keyboard = Keyboard::new();

    ram.write(513, 0x7A);
    ram.write(514, 0x00BC);
    // cpu.run_instruction(&mut ram, &mut display, &mut keyboard);
    println!("{:?}", cpu)
}
