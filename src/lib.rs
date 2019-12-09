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
        self.registers.program_counter += 1;
    }

    fn ret(&mut self) {
        self.registers.program_counter = self.stack.pop();
    }

    fn call(&mut self, addr: u16) {
        assert!((addr & 0xF000) == 0);

        self.stack.push(self.registers.program_counter);
        self.registers.program_counter = addr;
    }

    fn se(&mut self, vx: u8, value: u8) {
        if self.registers.v[vx as usize] == value {
            self.registers.program_counter += 2;
        } else {
            self.registers.program_counter += 1;
        }
    }

    fn sne(&mut self, vx: u8, value: u8) {
        if self.registers.v[vx as usize] != value {
            self.registers.program_counter += 2;
        } else {
            self.registers.program_counter += 1;
        }
    }

    fn sev(&mut self, vx: u8, vy: u8) {
        if self.registers.v[vx as usize] == self.registers.v[vy as usize] {
            self.registers.program_counter += 2;
        } else {
            self.registers.program_counter += 1;
        }
    }

    fn ld(&mut self, vx: u8, value: u8) {
        self.registers.v[vx as usize] = value;
        self.registers.program_counter += 1;
    }

    fn or(&mut self, vx: u8, vy: u8) {
        self.registers.v[vx as usize] |= self.registers.v[vy as usize];
        self.registers.program_counter += 1;
    }

    fn and(&mut self, vx: u8, vy: u8) {
        self.registers.v[vx as usize] &= self.registers.v[vy as usize];
        self.registers.program_counter += 1;
    }

    fn xor(&mut self, vx: u8, vy: u8) {
        self.registers.v[vx as usize] ^= self.registers.v[vy as usize];
        self.registers.program_counter += 1;
    }

    fn add(&mut self, vx: u8, vy: u8) {
        let (result, is_overflow) = self.registers.v[vx as usize].overflowing_add(self.registers.v[vy as usize]);
        self.registers.v[vx as usize] = result;
        self.registers.v[0xF] = if is_overflow { 1 } else { 0 };
        self.registers.program_counter += 1;
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
        assert_eq!(vm.registers.program_counter, 0);

        vm.cls();

        assert!(vm.graphics.display.iter().all(|&row| row == 0));
        assert_eq!(vm.registers.program_counter, 1);
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

    #[test]
    fn test_se_equal() {
        let mut vm = VM::new();
        vm.registers.v[1] = 1;
        vm.registers.program_counter = 5;

        vm.se(1, 1);

        assert_eq!(vm.registers.v[1], 1);
        assert_eq!(vm.registers.program_counter, 7);
    }

    #[test]
    fn test_se_not_equal() {
        let mut vm = VM::new();
        vm.registers.v[1] = 1;
        vm.registers.program_counter = 5;

        vm.se(1, 2);

        assert_eq!(vm.registers.v[1], 1);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_se_invalid() {
        let mut vm = VM::new();
        vm.sne(16, 1);
    }

    #[test]
    fn test_sne_equal() {
        let mut vm = VM::new();
        vm.registers.v[1] = 1;
        vm.registers.program_counter = 5;

        vm.sne(1, 1);

        assert_eq!(vm.registers.v[1], 1);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_sne_not_equal() {
        let mut vm = VM::new();
        vm.registers.v[1] = 1;
        vm.registers.program_counter = 5;

        vm.sne(1, 2);

        assert_eq!(vm.registers.v[1], 1);
        assert_eq!(vm.registers.program_counter, 7);
    }

    #[test]
    #[should_panic]
    fn test_sne_invalid() {
        let mut vm = VM::new();
        vm.sne(16, 1);
    }

    #[test]
    fn test_sev_equal() {
        let mut vm = VM::new();
        vm.registers.v[1] = 4;
        vm.registers.v[2] = 4;
        vm.registers.program_counter = 5;

        vm.sev(1, 2);

        assert_eq!(vm.registers.v[1], 4);
        assert_eq!(vm.registers.v[2], 4);
        assert_eq!(vm.registers.program_counter, 7);
    }

    #[test]
    fn test_sev_not_equal() {
        let mut vm = VM::new();
        vm.registers.v[1] = 4;
        vm.registers.v[2] = 5;
        vm.registers.program_counter = 5;

        vm.sev(1, 2);

        assert_eq!(vm.registers.v[1], 4);
        assert_eq!(vm.registers.v[2], 5);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_sev_invalid_first() {
        let mut vm = VM::new();
        vm.sev(16, 1);
    }

    #[test]
    #[should_panic]
    fn test_sev_invalid_second() {
        let mut vm = VM::new();
        vm.sev(0, 16);
    }

    #[test]
    fn test_ld() {
        let mut vm = VM::new();
        vm.registers.v[1] = 4;
        vm.registers.program_counter = 5;

        vm.ld(1, 2);

        assert_eq!(vm.registers.v[1], 2);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_ld_invalid() {
        let mut vm = VM::new();
        vm.ld(16, 1);
    }

    #[test]
    fn test_or() {
        let mut vm = VM::new();
        vm.registers.v[1] = 0xF0;
        vm.registers.v[2] = 0x0F;
        vm.registers.program_counter = 5;

        vm.or(1, 2);

        assert_eq!(vm.registers.v[1], 0xFF);
        assert_eq!(vm.registers.v[2], 0x0F);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_or_invalid_first() {
        let mut vm = VM::new();
        vm.or(16, 1);
    }

    #[test]
    #[should_panic]
    fn test_or_invalid_second() {
        let mut vm = VM::new();
        vm.or(0, 16);
    }

    #[test]
    fn test_and() {
        let mut vm = VM::new();
        vm.registers.v[1] = 0b0101;
        vm.registers.v[2] = 0b1110;
        vm.registers.program_counter = 5;

        vm.and(1, 2);

        assert_eq!(vm.registers.v[1], 0b0100);
        assert_eq!(vm.registers.v[2], 0b1110);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_and_invalid_first() {
        let mut vm = VM::new();
        vm.and(16, 1);
    }

    #[test]
    #[should_panic]
    fn test_and_invalid_second() {
        let mut vm = VM::new();
        vm.and(0, 16);
    }

    #[test]
    fn test_xor() {
        let mut vm = VM::new();
        vm.registers.v[1] = 0b0100;
        vm.registers.v[2] = 0b1110;
        vm.registers.program_counter = 5;

        vm.xor(1, 2);

        assert_eq!(vm.registers.v[1], 0b1010);
        assert_eq!(vm.registers.v[2], 0b1110);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_xor_invalid_first() {
        let mut vm = VM::new();
        vm.xor(16, 1);
    }

    #[test]
    #[should_panic]
    fn test_xor_invalid_second() {
        let mut vm = VM::new();
        vm.xor(0, 16);
    }

    #[test]
    fn test_add() {
        let mut vm = VM::new();
        vm.registers.v[1] = 200;
        vm.registers.v[2] = 100;
        vm.registers.v[0xF] = 4;
        vm.registers.program_counter = 5;

        vm.add(1, 2);

        assert_eq!(vm.registers.v[1], 44);
        assert_eq!(vm.registers.v[2], 100);
        assert_eq!(vm.registers.v[0xF], 1);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_add_invalid_first() {
        let mut vm = VM::new();
        vm.add(16, 1);
    }

    #[test]
    #[should_panic]
    fn test_add_invalid_second() {
        let mut vm = VM::new();
        vm.add(0, 16);
    }

}
