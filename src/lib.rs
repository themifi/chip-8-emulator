const MEMORY_SIZE: usize = 4096;

struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
    fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
        }
    }
}

const V_REGISTERS_SIZE: usize = 16;

struct Registers {
    v: [u8; V_REGISTERS_SIZE],
    i: u16,
    delay_timer: u16,
    sound_timer: u16,
    program_counter: u16,
}

impl Registers {
    fn new() -> Self {
        Self {
            v: [0; V_REGISTERS_SIZE],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
        }
    }
}

struct Graphics {
    display: [u64; 32],
}

impl Graphics {
    fn new() -> Self {
        Self {
            display: [0; 32],
        }
    }
}

const STACK_SIZE: usize = 16;

struct Stack {
    stack: [u16; STACK_SIZE],
    pointer: u8,
}

impl Stack {
    fn new() -> Self {
        Self {
            stack: [0; STACK_SIZE],
            pointer: 0,
        }
    }
}

struct Input {
    keypad: u16,
}

impl Input {
    fn new() -> Self {
        Self {
            keypad: 0,
        }
    }
}

struct VM {
    memory: Memory,
    registers: Registers,
    stack: Stack,
    graphics: Graphics,
    input: Input,
}

impl VM {
    fn new() -> VM {
        VM {
            memory: Memory::new(),
            registers: Registers::new(),
            stack: Stack::new(),
            graphics: Graphics::new(),
            input: Input::new(),
        }
    }
}
