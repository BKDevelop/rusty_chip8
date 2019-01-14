//  CHIP-8 Memory specs:
//      4096 bytes of memory
//
//      Memory Map
//      0x000-0x1FF - Chip 8 interpreter (this is not needed in emu, we could store something else here)
//      0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
//      0x200-0xFFF - Program ROM and work RAM
pub struct Memory {
    mem: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { mem: vec![0; 4096] }
    }

    pub fn put(mut self, position: u8, byte: u8) {
        self.mem[position as usize] = byte;
    }

    pub fn get(&self, position: u16) -> u8 {
        self.mem[position as usize]
    }

    pub fn load_game(mut self, game_rom: Vec<u8>) {
        let mut mem_pos = 0x200;

        for byte in game_rom {
            self.mem[mem_pos as usize] = byte;

            mem_pos += 1;
        }

        println!("Game loaded!")
    }
}
