mod chip8;

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    //let game_rom = fs::read(&args[0]).expect("Could not load game!");
    let game_rom = fs::read("Pong.ch8").expect("Could not load game!");

    let mut my_chip8 = chip8::Chip8::new();
    my_chip8.load_game(game_rom);
    my_chip8.start_emulation();
}
