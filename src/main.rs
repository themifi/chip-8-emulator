use chip_8_emulator::VM;
use std::{env, fs};

fn main() {
    let mut args = env::args();
    let program_path = args.nth(1).unwrap();
    let program = fs::read(program_path).unwrap();

    let mut vm = VM::new();
    vm.load_program(&program);
    vm.run_program();
}
