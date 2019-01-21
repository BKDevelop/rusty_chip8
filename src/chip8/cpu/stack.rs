pub struct Stack {
    stack: Vec<u16>,
    stack_pointer: u8,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: vec![0; 16],
            stack_pointer: 0,
        }
    }

    pub fn push(&mut self, bytes: u16) {
        if self.stack_pointer < 15 {
            self.stack[self.stack_pointer as usize] = bytes;
            self.stack_pointer += 1;
        } else {
            panic!("Tried to push to many items to stack")
        }
    }

    pub fn pop(&mut self) -> u16 {
        if self.stack_pointer > 0 {
            let ret = self.stack[self.stack_pointer as usize];
            self.stack[self.stack_pointer as usize] = 0;
            self.stack_pointer -= 1;

            ret
        } else {
            panic!("Tried to pop empty stash")
        }
    }
}
