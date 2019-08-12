use std::env;

mod chip8;

fn main() {
    if let Some(game) = env::args().nth(1) {
        println!("Loading game: {}", game);
        let mut chip8 = chip8::Chip8::new();
        chip8.run();
    } else {
        println!("Please provide the path to a game.");
    }
}
