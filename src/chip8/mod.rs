use std::fmt;

mod cpu;

extern crate sdl2;

pub struct Chip8 {
    cpu: cpu::Cpu
}

impl Chip8 {
    pub fn new(game: &String) -> Chip8 {
        let mut chip8 = Chip8 {
            cpu: cpu::Cpu::initialize()
        };

        chip8.cpu.load_font_set();
        chip8.cpu.load_game(game);

        return chip8;
    }

    pub fn run(&mut self) {
        loop {
            println!("{:?}", self);
            self.cpu.cycle();
        }
    }
}

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cpu)
    }
}
