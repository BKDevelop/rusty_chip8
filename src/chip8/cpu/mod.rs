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
            mem: memory::Memory::new(),
            stack: stack::Stack::new(),
            index_register: 0,
            program_counter: 0x200, // program counter starts at 0x200!
            cpu_register: vec![0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_game(&mut self, game_rom: Vec<u8>) {
        self.mem.load_game(game_rom);
    }

    pub fn emulate_cycle(self) {
        let opcode = self.read_opcode();
        self.execute_opcode(opcode);
    }

    fn read_opcode(&self) -> u16 {
        // Opcode is stored in 2 bytes in memory, we have to join them!
        // move first entry 8 bits to left, use bitwise or to join
        // 0xA2 << 8 = 0xA200 | 0x00F0 -> 0xA2F0
        let opcode = (self.mem.get(self.program_counter) as u16) << 8
            | (self.mem.get(self.program_counter + 1) as u16);
        opcode
    }

    fn execute_opcode(mut self, opcode: u16) {
        // opcode is hex number -> 0xXXXX
        // consists of 4 x 4 bit "pins"
        let pins = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );

        // opcodes have some values that are often used
        // here singled out for convinence

        // Ox?NNN
        let nnn = opcode & 0x0FFF;
        // 0x??NN
        let nn = (opcode & 0x0FF) as u8;
        // 0x?X??
        let x = ((opcode & 0x0F00) >> 8) as u8;
        // 0x??Y?
        let y = ((opcode & 0x00F0) >> 4) as u8;

        // see opcode table https://en.wikipedia.org/wiki/CHIP-8#Opcode_table
        self.program_counter = match pins {
            (0x0, 0x0, 0xE, 0x0) => panic!("opcode {} not implemented yet", opcode),
            (0x0, 0x0, 0xE, 0xE) => self.stack.pop(),
            (0x1, _, _, _) => nnn,
            (0x2, _, _, _) => {
                self.stack.push(self.program_counter);
                nnn
            }
            (0x3, _, _, _) => {
                if self.cpu_register[x as usize] == nn {
                    self.skip_next_opcode()
                } else {
                    self.next_opcode()
                }
            }
            (0x4, _, _, _) => {
                if self.cpu_register[x as usize] != nn {
                    self.skip_next_opcode()
                } else {
                    self.next_opcode()
                }
            }
            (0x5, _, _, _) => {
                if self.cpu_register[x as usize] == self.cpu_register[y as usize] {
                    self.skip_next_opcode()
                } else {
                    self.next_opcode()
                }
            }
            (0x6, _, _, _) => {
                self.cpu_register[x as usize] = nn;
                self.next_opcode()
            }
            (0x7, _, _, _) => {
                self.cpu_register[x as usize] += nn;
                self.next_opcode()
            }
            (0x8, _, _, 0x0) => {
                self.cpu_register[x as usize] = self.cpu_register[y as usize];
                self.next_opcode()
            }
            (0x8, _, _, 0x1) => {
                self.cpu_register[x as usize] = self.cpu_register[x as usize] | self.cpu_register[y as usize];
                self.next_opcode()
            }
            (0x8, _, _, 0x2) => {
                self.cpu_register[x as usize] = self.cpu_register[x as usize] & self.cpu_register[y as usize];
                self.next_opcode()
            }
            (0x8, _, _, 0x3) => panic!("opcode {} not implemented yet", opcode),
            (0x8, _, _, 0x4) => {
                self.cpu_register[x as usize] += self.cpu_register[y as usize];
                self.next_opcode()
            }
            (0x8, _, _, 0x5) => {
                self.cpu_register[x as usize] -= self.cpu_register[y as usize];
                self.next_opcode()
            }

            (0x8, _, _, 0x6) => panic!("opcode {} not implemented yet", opcode),
            (0x8, _, _, 0x7) => {
                self.cpu_register[x as usize] = self.cpu_register[y as usize] - self.cpu_register[x as usize];
                self.next_opcode()
            }
            (0x8, _, _, 0xE) => panic!("opcode {} not implemented yet", opcode),
            (0x9, _, _, 0x0) => {
                if self.cpu_register[x as usize] != self.cpu_register[y as usize] {
                    self.skip_next_opcode()
                } else {
                    self.next_opcode()
                }
            }
            (0xA, _, _, _) => {
                self.index_register = nnn;
                self.next_opcode()
            }
            (0xB, _, _, _) => (self.cpu_register[0] as u16) + nnn,
            (0xC, _, _, _) => panic!("opcode {} not implemented yet", opcode),
            (0xD, _, _, _) => panic!("opcode {} not implemented yet", opcode),
            (0xE, _, _, _) => panic!("opcode {} not implemented yet", opcode),
            (0xF, _, _, _) => panic!("opcode {} not implemented yet", opcode),
            _ => panic!("unknown opcode received: {}", opcode),
        }
    }

    fn next_opcode(&self) -> u16 {
        self.program_counter + 2
    }

    fn skip_next_opcode(&self) -> u16 {
        self.next_opcode();
        self.next_opcode()
    }
}
