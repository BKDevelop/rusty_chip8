mod chip8;

use std::fs;

fn main() {
    let my_chip8 = chip8::Chip8::new();
    //    let game_rom = fs::read("pong")
    //        .expect("Could not load game!");
    let game_rom = vec![1, 16];

    my_chip8.load_game(game_rom);
}
