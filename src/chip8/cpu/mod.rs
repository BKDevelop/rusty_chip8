//  CHIP-8 specs:
//      35 opcodes -> 2 bytes long
//      4096 bytes of memory
//      16 chip8 register, each 1 byte
//          -> 15 general puprose (0-15)
//          -> 1 'carry flag' for arithmetic
//      1 index register -> 2 bytes
//      1 program counter -> 2 bytes
//
//      2 timer registers, 1 byte
//          -> delay timer
//          -> sound timer

mod memory;
mod stack;

pub struct Cpu {
    opcode: u16,
    mem: memory::Memory,
    stack: stack::Stack,
    index_register: u16,
    program_counter: u16,
    cpu_register: Vec<u8>,

    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            opcode: 0,
            mem: memory::Memory::new(),
            stack: stack::Stack::new(),
            index_register: 0,
            program_counter: 0x200, // program counter starts at 0x200!
            cpu_register: vec![0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_game(self, game_rom: Vec<u8>) {
        self.mem.load_game(game_rom);
    }

    pub fn emulate_cycle(self) {
        self.read_opcode();
        self.execute_opcode();
    }


    fn read_opcode(&self) {
        // Opcode is stored in 2 bytes in memory, we have to join them!
        // move first entry 8 bits to left, use bitwise or to join
        // 0xA2 << 8 = 0xA200 | 0x00F0 -> 0xA2F0
        let opcode = self.mem.get(self.program_counter) << 8 |
                        self.mem.get(self.program_counter + 1);
    }

    fn execute_opcode(mut self){

    }
}