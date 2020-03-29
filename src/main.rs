use chip_8_emulator::VM;

fn main() {
    let mut vm = VM::new();
    vm.exec_instruction(0x00E0u16.to_be());
}
