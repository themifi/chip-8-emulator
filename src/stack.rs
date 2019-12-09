const STACK_SIZE: usize = 16;

pub struct Stack {
    pub stack: [u16; STACK_SIZE],
    pub pointer: u8,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: [0; STACK_SIZE],
            pointer: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        assert!((self.pointer as usize) < STACK_SIZE-1);
        self.stack[(self.pointer as usize)] = value;
        self.pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        assert!(self.pointer > 0);
        self.pointer -= 1;
        self.stack[(self.pointer as usize)]
    }
}
