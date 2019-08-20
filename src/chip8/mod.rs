use std::fmt;

mod cpu;
mod input;
mod graphics;

extern crate sdl2;

pub struct Chip8 {
    cpu: cpu::Cpu,
    graphics: sdl2::render::Canvas<sdl2::video::Window>,
    input: sdl2::EventPump
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let sdl_context = sdl2::init().unwrap();

        let mut chip8 = Chip8 {
            cpu: cpu::Cpu::initialize(),
            graphics: graphics::initialize(&sdl_context),
            input: input::initialize(&sdl_context)
        };

        chip8.cpu.load_font_set(&graphics::FONT_SET);

        return chip8;
    }

    pub fn load_game(&mut self, game: &String) {
        self.cpu.load_game(game);
    }

    pub fn run(&mut self) {
        loop {
            println!("{:?}", self);
            //self.cpu.cycle();
            input::poll_for_event(&mut self.input);
            graphics::draw(&mut self.graphics);
        }
    }
}

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.cpu)
    }
}
