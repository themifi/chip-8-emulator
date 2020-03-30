use chip_8_emulator::VM;
use std::{env, fs};

fn main() {
    let args = env::args();
    let program_path = args.skip(1).next().unwrap();
    let program = fs::read(program_path).unwrap();

    let mut vm = VM::new();
    vm.load_program(&program);
}
