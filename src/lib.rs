mod memory;
mod registers;
mod graphics;
mod stack;
mod input;

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

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
    rng: SmallRng,
}

impl VM {
    pub fn new() -> VM {
        Self {
            memory: Memory::new_with_initial_sprites(),
            registers: Registers::new(),
            stack: Stack::new(),
            graphics: Graphics::new(),
            input: Input::new(),
            rng: SmallRng::seed_from_u64(0),
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

    fn sub(&mut self, vx: u8, vy: u8) {
        let (result, is_overflow) = self.registers.v[vx as usize].overflowing_sub(self.registers.v[vy as usize]);
        self.registers.v[vx as usize] = result;
        self.registers.v[0xF] = if is_overflow { 1 } else { 0 };
        self.registers.program_counter += 1;
    }

    fn shr(&mut self, vx: u8) {
        self.registers.v[0xF] = self.registers.v[vx as usize] % 2;
        self.registers.v[vx as usize] >>= 1;
        self.registers.program_counter += 1;
    }

    fn subn(&mut self, vx: u8, vy: u8) {
        let (result, is_overflow) = self.registers.v[vy as usize].overflowing_sub(self.registers.v[vx as usize]);
        self.registers.v[vx as usize] = result;
        self.registers.v[0xF] = if is_overflow { 1 } else { 0 };
        self.registers.program_counter += 1;
    }

    fn shl(&mut self, vx: u8) {
        self.registers.v[0xF] = if self.registers.v[vx as usize] >= 0b10000000 { 1 } else { 0 };
        self.registers.v[vx as usize] <<= 1;
        self.registers.program_counter += 1;
    }

    fn ldi(&mut self, value: u16) {
        assert!((value & 0xF000) == 0);
        self.registers.i = value;
        self.registers.program_counter += 1;
    }

    fn jpv0(&mut self, addr: u16) {
        assert!((addr & 0xF000) == 0);
        self.registers.program_counter = addr + (self.registers.v[0] as u16);
    }

    fn rnd(&mut self, vx: u8, mask: u8) {
        let value = self.rng.gen::<u8>() & mask;
        self.registers.v[vx as usize] = value;
        self.registers.program_counter += 1;
    }

    fn drw(&mut self, vx: u8, vy: u8, n: u8) {
        let sprite = self.memory.get_slice(self.registers.i as usize, self.registers.i as usize + n as usize);
        let is_collision = self.graphics.draw_sprite(vx as usize, vy as usize, sprite);
        self.registers.v[0xF] = if is_collision { 1 } else { 0 };
        self.registers.program_counter += 1;
    }

    fn skp(&mut self, x: u8) {
        self.registers.program_counter += if self.input.is_key_pressed(x) { 2 } else { 1 };
    }

    fn sknp(&mut self, x: u8) {
        self.registers.program_counter += if self.input.is_key_pressed(x) { 1 } else { 2 };
    }

    fn ld_dt(&mut self, x: u8) {
        self.registers.delay_timer = self.registers.v[x as usize] as u16;
        self.registers.program_counter += 1;
    }

    fn ld_st(&mut self, x: u8) {
        self.registers.sound_timer = self.registers.v[x as usize] as u16;
        self.registers.program_counter += 1;
    }

    fn add_i(&mut self, x: u8) {
        self.registers.i += self.registers.v[x as usize] as u16;
        self.registers.program_counter += 1;       
    }

    fn ld_f(&mut self, x: u8) {
        let sprite_num = self.registers.v[x as usize] as usize;
        let sprite_location = memory::SPRITE_START_LOCATION + (sprite_num * memory::SPRITE_SIZE);
        self.registers.i = sprite_location as u16;
        self.registers.program_counter += 1;       
    }

    fn ld_b(&mut self, x: u8) {
        let number = self.registers.v[x as usize];
        let ones = number % 10;
        let tens = number / 10 % 10;
        let hundreds = number / 100;

        let start_position = self.registers.i as usize;
        let slice = self.memory.get_slice_mut(start_position, start_position + 3);
        slice[0] = hundreds;
        slice[1] = tens;
        slice[2] = ones;
        self.registers.program_counter += 1;       
    }

    fn ld_i(&mut self, x: u8) {
        let registers = &self.registers.v[0..=x as usize];
        let start_memory_pos = self.registers.i as usize;
        let finis_memory_pos = start_memory_pos + registers.len();
        let memory = self.memory.get_slice_mut(start_memory_pos, finis_memory_pos);

        memory.copy_from_slice(registers);

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
    fn test_add_with_overflow() {
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
    fn test_add_without_overflow() {
        let mut vm = VM::new();
        vm.registers.v[1] = 50;
        vm.registers.v[2] = 100;
        vm.registers.v[0xF] = 4;
        vm.registers.program_counter = 5;

        vm.add(1, 2);

        assert_eq!(vm.registers.v[1], 150);
        assert_eq!(vm.registers.v[2], 100);
        assert_eq!(vm.registers.v[0xF], 0);
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

    #[test]
    fn test_sub_with_overflow() {
        let mut vm = VM::new();
        vm.registers.v[1] = 100;
        vm.registers.v[2] = 200;
        vm.registers.v[0xF] = 4;
        vm.registers.program_counter = 5;

        vm.sub(1, 2);

        assert_eq!(vm.registers.v[1], 156);
        assert_eq!(vm.registers.v[2], 200);
        assert_eq!(vm.registers.v[0xF], 1);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_sub_without_overflow() {
        let mut vm = VM::new();
        vm.registers.v[1] = 150;
        vm.registers.v[2] = 100;
        vm.registers.v[0xF] = 4;
        vm.registers.program_counter = 5;

        vm.sub(1, 2);

        assert_eq!(vm.registers.v[1], 50);
        assert_eq!(vm.registers.v[2], 100);
        assert_eq!(vm.registers.v[0xF], 0);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_sub_invalid_first() {
        let mut vm = VM::new();
        vm.sub(16, 1);
    }

    #[test]
    #[should_panic]
    fn test_sub_invalid_second() {
        let mut vm = VM::new();
        vm.sub(0, 16);
    }

    #[test]
    fn test_shr_with_overflow() {
        let mut vm = VM::new();
        vm.registers.v[1] = 0b0101;
        vm.registers.v[0xF] = 4;
        vm.registers.program_counter = 5;

        vm.shr(1);

        assert_eq!(vm.registers.v[1], 0b0010);
        assert_eq!(vm.registers.v[0xF], 1);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_shr_without_overflow() {
        let mut vm = VM::new();
        vm.registers.v[1] = 0b1010;
        vm.registers.v[0xF] = 4;
        vm.registers.program_counter = 5;

        vm.shr(1);

        assert_eq!(vm.registers.v[1], 0b0101);
        assert_eq!(vm.registers.v[0xF], 0);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_shr_invalid() {
        let mut vm = VM::new();
        vm.shr(16);
    }

    #[test]
    fn test_subn_with_overflow() {
        let mut vm = VM::new();
        vm.registers.v[1] = 200;
        vm.registers.v[2] = 100;
        vm.registers.v[0xF] = 4;
        vm.registers.program_counter = 5;

        vm.subn(1, 2);

        assert_eq!(vm.registers.v[1], 156);
        assert_eq!(vm.registers.v[2], 100);
        assert_eq!(vm.registers.v[0xF], 1);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_subn_without_overflow() {
        let mut vm = VM::new();
        vm.registers.v[1] = 100;
        vm.registers.v[2] = 150;
        vm.registers.v[0xF] = 4;
        vm.registers.program_counter = 5;

        vm.subn(1, 2);

        assert_eq!(vm.registers.v[1], 50);
        assert_eq!(vm.registers.v[2], 150);
        assert_eq!(vm.registers.v[0xF], 0);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_subn_invalid_first() {
        let mut vm = VM::new();
        vm.subn(16, 1);
    }

    #[test]
    #[should_panic]
    fn test_subn_invalid_second() {
        let mut vm = VM::new();
        vm.subn(0, 16);
    }

    #[test]
    fn test_shl_with_overflow() {
        let mut vm = VM::new();
        vm.registers.v[1] = 0b10101010;
        vm.registers.v[0xF] = 4;
        vm.registers.program_counter = 5;

        vm.shl(1);

        assert_eq!(vm.registers.v[1], 0b01010100);
        assert_eq!(vm.registers.v[0xF], 1);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_shl_without_overflow() {
        let mut vm = VM::new();
        vm.registers.v[1] = 0b01101010;
        vm.registers.v[0xF] = 4;
        vm.registers.program_counter = 5;

        vm.shl(1);

        assert_eq!(vm.registers.v[1], 0b11010100);
        assert_eq!(vm.registers.v[0xF], 0);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_shl_invalid() {
        let mut vm = VM::new();
        vm.shr(16);
    }

    #[test]
    fn test_ldi() {
        let mut vm = VM::new();
        vm.registers.i = 5;
        vm.registers.program_counter = 5;

        vm.ldi(0x0111);

        assert_eq!(vm.registers.i, 0x0111);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    #[should_panic]
    fn test_ldi_invalid() {
        let mut vm = VM::new();
        vm.ldi(0xF000);
    }

    #[test]
    fn test_jpv0() {
        let mut vm = VM::new();
        vm.registers.program_counter = 100;
        vm.registers.v[0] = 5;

        vm.jpv0(20);

        assert_eq!(vm.registers.program_counter, 25);
    }

    #[test]
    #[should_panic]
    fn test_jpv0_invalid() {
        let mut vm = VM::new();
        vm.jpv0(0xF000);
    }

    #[test]
    fn test_rnd() {
        let mut vm = VM::new();
        vm.rng = SmallRng::seed_from_u64(0xFF);
        vm.registers.program_counter = 5;
        vm.registers.v[1] = 0xAF;

        vm.rnd(1, 0xFF);

        assert_eq!(vm.registers.v[1], 181);
        assert_eq!(vm.registers.program_counter, 6);

        vm.rnd(1, 0x0F);

        assert_eq!(vm.registers.v[1], 5);
        assert_eq!(vm.registers.program_counter, 7);
    }

    #[test]
    #[should_panic]
    fn test_rnd_invalid() {
        let mut vm = VM::new();
        vm.rnd(0xFF, 0);
    }

    #[test]
    fn test_drw() {
        let mut vm = VM::new();
        vm.registers.program_counter = 5;
        let location = 0x100;
        vm.registers.i = location as u16;
        vm.registers.v[0xF] = 2;
        let sprite = [0x20, 0x60, 0x20, 0x20, 0x70];
        vm.memory.get_slice_mut(location, location + sprite.len()).copy_from_slice(&sprite);

        vm.drw(4, 4, 5);

        let screen = [0, 0, 0, 0, 0x200, 0x600, 0x200, 0x200, 0x700, 0];
        assert_eq!(&vm.graphics.display[0..10], &screen);
        assert_eq!(vm.registers.v[0xF], 0);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_drw_collision() {
        let mut vm = VM::new();
        vm.registers.program_counter = 5;
        let location = 0x100;
        vm.registers.i = location as u16;
        vm.registers.v[0xF] = 2;
        let sprite = [0xFF];
        vm.memory.get_slice_mut(location, location + sprite.len()).copy_from_slice(&sprite);
        vm.graphics.display[0] = 0x1;

        vm.drw(0, 0, 1);

        assert_eq!(vm.graphics.display[0], 0xFE);
        assert_eq!(vm.registers.v[0xF], 1);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_skp_key_pressed() {
        let mut vm = VM::new();
        vm.input = Input::new_with_state(0b100);
        vm.registers.program_counter = 5;

        vm.skp(2);

        assert_eq!(vm.registers.program_counter, 7);
    }

    #[test]
    fn test_skp_key_unpressed() {
        let mut vm = VM::new();
        vm.input = Input::new_with_state(0b100);
        vm.registers.program_counter = 5;

        vm.skp(4);

        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_sknp_key_pressed() {
        let mut vm = VM::new();
        vm.input = Input::new_with_state(0b100);
        vm.registers.program_counter = 5;

        vm.sknp(2);

        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_sknp_key_unpressed() {
        let mut vm = VM::new();
        vm.input = Input::new_with_state(0b100);
        vm.registers.program_counter = 5;

        vm.sknp(4);

        assert_eq!(vm.registers.program_counter, 7);
    }

    #[test]
    fn test_ld_dt() {
        let mut vm = VM::new();
        vm.registers.program_counter = 5;
        let delay_timer_value = 0xFA;
        vm.registers.v[0x1] = delay_timer_value;

        vm.ld_dt(0x1);

        assert_eq!(vm.registers.v[0x1], delay_timer_value);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_ld_st() {
        let mut vm = VM::new();
        vm.registers.program_counter = 5;
        let sound_timer_value = 0xFA;
        vm.registers.v[0x2] = sound_timer_value;

        vm.ld_st(0x2);

        assert_eq!(vm.registers.v[0x2], sound_timer_value);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_add_i() {
        let mut vm = VM::new();
        vm.registers.program_counter = 5;
        vm.registers.i = 10;
        vm.registers.v[0x2] = 5;

        vm.add_i(0x2);

        assert_eq!(vm.registers.i, 15);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_ld_f() {
        let mut vm = VM::new();
        vm.registers.program_counter = 5;
        vm.registers.i = 10;
        vm.registers.v[0x2] = 5;

        vm.ld_f(0x2);

        assert_eq!(vm.registers.i, 25);
        let sprite_five = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
        let sprite = vm.memory.get_slice(vm.registers.i as usize, vm.registers.i as usize + memory::SPRITE_SIZE);
        assert_eq!(sprite, &sprite_five);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_ld_b() {
        let mut vm = VM::new();
        vm.registers.program_counter = 5;
        vm.registers.v[0x5] = 123;
        vm.registers.i = 100;

        vm.ld_b(0x5);

        assert_eq!(vm.memory.get_slice(100, 103), &[1, 2, 3]);
        assert_eq!(vm.registers.i, 100);
        assert_eq!(vm.registers.program_counter, 6);
    }

    #[test]
    fn test_ld_i() {
        let mut vm = VM::new();
        vm.registers.program_counter = 5;
        vm.registers.i = 0x100;
        let registers = (0x0..=0xF).collect::<Vec<u8>>();
        vm.registers.v.copy_from_slice(&registers);

        vm.ld_i(0xF);

        assert_eq!(vm.memory.get_slice(0x100, 0x110), registers.as_slice());
        assert_eq!(vm.registers.program_counter, 6);
    }
}
