struct Memory {
    memory: [u8; 4096],
}

struct Registers {
    v: [u8; 16],
    i: u16,
    delay_timer: u16,
    sound_timer: u16,
    program_counter: u16,
}

struct Graphics {
    display: [u64; 32],
}

struct Stack {
    stack: [u16; 16],
    pointer: u8,
}

struct Input {
    keypad: u16,
}

struct VM {
    memory: Memory,
    registers: Registers,
    stack: Stack,
    graphics: Graphics,
    input: Input,
}
