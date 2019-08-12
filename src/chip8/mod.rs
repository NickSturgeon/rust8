use std::fmt;

mod cpu;
mod input;
mod graphics;

pub struct Chip8 {
    cpu: cpu::Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            cpu: cpu::Cpu::initialize()
        };

        chip8.cpu.load_font_set(&graphics::FONT_SET);

        return chip8;
    }

    pub fn run(&mut self) {
       self.cpu.cycle();
    }
}

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cpu: {:?}", self.cpu)
    }
}