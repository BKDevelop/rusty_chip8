mod chip8;

use std::fs;

fn main() {
    let mut my_chip8 = chip8::Chip8::new();
//    let game_rom = fs::read("src/pong").expect("Could not load game!");

//    my_chip8.load_game(game_rom);
    my_chip8.emulate_cycle();

}