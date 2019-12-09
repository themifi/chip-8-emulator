mod memory;
mod registers;
mod graphics;
mod stack;
mod input;

use memory::Memory;
use registers::Registers;
use graphics::Graphics;
use stack::Stack;
use input::Input;

pub struct VM {
    memory: Memory,
    registers: Registers,
    stack: Stack,
    graphics: Graphics,
    input: Input,
}

impl VM {
    pub fn new() -> VM {
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

    fn call(&mut self, addr: u16) {
        assert!((addr & 0xF000) == 0);
        self.stack.push(self.registers.program_counter);
        self.registers.program_counter = addr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::u64;

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
        vm.graphics.display = [u64::MAX; graphics::DISPLAY_ROWS];

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

    #[test]
    fn test_call() {
        let mut vm = VM::new();
        vm.registers.program_counter = 1;
        vm.stack.push(2);
        vm.stack.push(3);

        vm.call(4);

        assert_eq!(vm.registers.program_counter, 4);
        assert_eq!(vm.stack.pointer, 3);
        assert_eq!(vm.stack.stack[0], 2);
        assert_eq!(vm.stack.stack[1], 3);
        assert_eq!(vm.stack.stack[2], 1);
    }

    #[test]
    #[should_panic]
    fn test_call_invalid_addr() {
        let mut vm = VM::new();
        vm.call(0x1111);
    }

    #[test]
    #[should_panic]
    fn test_call_invalid_addr_edge_case() {
        let mut vm = VM::new();
        vm.call(0x1000);
    }
}
