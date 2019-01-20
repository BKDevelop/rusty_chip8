mod cpu;

pub struct Chip8 {
    cpu: cpu::Cpu,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: cpu::Cpu::new(),
        }
    }


    pub fn load_game(&mut self, game_rom: Vec<u8>) {
        self.cpu.load_game(game_rom)
    }
    
    pub fn run(&mut self) {
        loop {
            self.cpu.emulate_cycle();
        }
    }
}
