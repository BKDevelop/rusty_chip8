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
        Memory {
            mem: Memory::init_mem_with_fonts(),
        }
    }

    pub fn put(mut self, position: u8, byte: u8) {
        self.mem[position as usize] = byte;
    }

    pub fn get(&self, position: u16) -> u8 {
        self.mem[position as usize]
    }

    pub fn load_game(&mut self, game_rom: Vec<u8>) {
        let mut mem_pos = 0x200;

        for byte in game_rom {
            self.mem[mem_pos as usize] = byte;

            mem_pos += 1;
        }

        println!("Game loaded!")
    }

    pub fn get_display_memory(&self) -> Vec<u8> {
        vec![0; 1024]
    }

    fn init_mem_with_fonts() -> Vec<u8> {
        let font: Vec<u8> = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        let mut font_position = 0x050;
        let mut mem = vec![0; 4096];
        for byte in font {
            mem[font_position] = byte;
            font_position += 1;
        }

        mem
    }
}
