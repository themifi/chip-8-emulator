use std::u64;

const MEMORY_SIZE: usize = 4096;

static INITIAL_SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];

struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
    fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
        }
    }

    fn new_with_initial_sprites() -> Self {
        let mut memory = Self::new();
        memory.memory[0..80].copy_from_slice(&INITIAL_SPRITES);
        memory
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

const DISPLAY_ROWS: usize = 32;

struct Graphics {
    display: [u64; DISPLAY_ROWS],
}

impl Graphics {
    fn new() -> Self {
        Self {
            display: [0; DISPLAY_ROWS],
        }
    }

    fn clear(&mut self) {
        self.display = [0; DISPLAY_ROWS];
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

    fn push(&mut self, value: u16) {
        assert!((self.pointer as usize) < STACK_SIZE-1);
        self.stack[(self.pointer as usize)] = value;
        self.pointer += 1;
    }

    fn pop(&mut self) -> u16 {
        assert!(self.pointer > 0);
        self.pointer -= 1;
        self.stack[(self.pointer as usize)]
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
        Self {
            memory: Memory::new_with_initial_sprites(),
            registers: Registers::new(),
            stack: Stack::new(),
            graphics: Graphics::new(),
            input: Input::new(),
        }
    }

    fn jump(&mut self, addr: u16) {
        assert!((addr & 0xF000) == 0);
        self.registers.program_counter = addr;
    }

    fn cls(&mut self) {
        self.graphics.clear();
    }

    fn ret(&mut self) {
        self.registers.program_counter = self.stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_memory_empty() {
        let memory = Memory::new();
        assert!(memory.memory.iter().all(|&byte| byte == 0));
    }

    #[test]
    fn test_initial_memory_with_initial_sprites() {
        let memory = Memory::new_with_initial_sprites();
        for (i, &byte) in memory.memory[0..80].iter().enumerate() {
            assert_eq!(byte, INITIAL_SPRITES[i]);
        }
        assert!(memory.memory[80..].iter().all(|&byte| byte == 0));
    }

    #[test]
    fn test_jump_opcode() {
        let mut vm = VM::new();
        let addr = 16u16;

        vm.jump(addr);

        assert_eq!(vm.registers.program_counter, addr);
    }

    #[test]
    fn test_jump_edge_case() {
        let mut vm = VM::new();
        let addr = 0x0FFF;

        vm.jump(addr);

        assert_eq!(vm.registers.program_counter, addr);
    }

    #[test]
    #[should_panic]
    fn test_jump_incorrect_addr() {
        let mut vm = VM::new();
        vm.jump(0xFFFFu16);
    }

    #[test]
    #[should_panic]
    fn test_jump_incorrect_addr_edge_case() {
        let mut vm = VM::new();
        vm.jump(0x1000);
    }

    #[test]
    fn test_cls() {
        let mut vm = VM::new();
        vm.graphics.display = [u64::MAX; DISPLAY_ROWS];

        vm.cls();

        assert!(vm.graphics.display.iter().all(|&row| row == 0));
    }

    #[test]
    fn test_ret() {
        let mut vm = VM::new();
        vm.registers.program_counter = 1;
        vm.stack.push(2);
        vm.stack.push(3);

        vm.ret();

        assert_eq!(vm.registers.program_counter, 3);
        assert_eq!(vm.stack.pointer, 1);
        assert_eq!(vm.stack.stack[0], 2);
    }
}
