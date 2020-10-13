use super::interpreter::Interpreter;

/// Parse instruction and call interpreter
#[allow(clippy::cognitive_complexity)]
pub fn execute_instruction(inst: u16, interpreter: &mut impl Interpreter) {
    match inst {
        0x00E0 => interpreter.cls(),
        0x00EE => interpreter.ret(),
        inst if inst & 0xF000 == 0x1000 => {
            let addr = inst & 0x0FFF;
            interpreter.jp(addr);
        }
        inst if inst & 0xF000 == 0x2000 => {
            let addr = inst & 0x0FFF;
            interpreter.call(addr);
        }
        inst if inst & 0xF000 == 0x3000 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let value = (inst & 0x00FF) as u8;
            interpreter.se(x, value);
        }
        inst if inst & 0xF000 == 0x4000 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let value = (inst & 0x00FF) as u8;
            interpreter.sne(x, value);
        }
        inst if inst & 0xF00F == 0x5000 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let y = ((inst & 0x00F0) >> 4) as u8;
            interpreter.se_v(x, y);
        }
        inst if inst & 0xF000 == 0x6000 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let value = (inst & 0x00FF) as u8;
            interpreter.ld_vx(x, value);
        }
        inst if inst & 0xF000 == 0x7000 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let value = (inst & 0x00FF) as u8;
            interpreter.add_vx(x, value);
        }
        inst if inst & 0xF00F == 0x8000 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let y = ((inst & 0x00F0) >> 4) as u8;
            interpreter.ld_vx_vy(x, y);
        }
        inst if inst & 0xF00F == 0x8001 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let y = ((inst & 0x00F0) >> 4) as u8;
            interpreter.or(x, y);
        }
        inst if inst & 0xF00F == 0x8002 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let y = ((inst & 0x00F0) >> 4) as u8;
            interpreter.and(x, y);
        }
        inst if inst & 0xF00F == 0x8003 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let y = ((inst & 0x00F0) >> 4) as u8;
            interpreter.xor(x, y);
        }
        inst if inst & 0xF00F == 0x8004 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let y = ((inst & 0x00F0) >> 4) as u8;
            interpreter.add_vx_vy(x, y);
        }
        inst if inst & 0xF00F == 0x8005 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let y = ((inst & 0x00F0) >> 4) as u8;
            interpreter.sub(x, y);
        }
        inst if inst & 0xF00F == 0x8006 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.shr(x);
        }
        inst if inst & 0xF00F == 0x8007 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let y = ((inst & 0x00F0) >> 4) as u8;
            interpreter.subn(x, y);
        }
        inst if inst & 0xF00F == 0x800E => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.shl(x);
        }
        inst if inst & 0xF00F == 0x9000 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let y = ((inst & 0x00F0) >> 4) as u8;
            interpreter.sne_vx_vy(x, y);
        }
        inst if inst & 0xF000 == 0xA000 => {
            let value = inst & 0x0FFF;
            interpreter.ld_i(value);
        }
        inst if inst & 0xF000 == 0xB000 => {
            let addr = inst & 0x0FFF;
            interpreter.jp_v0(addr);
        }
        inst if inst & 0xF000 == 0xC000 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let mask = (inst & 0x00FF) as u8;
            interpreter.rnd(x, mask);
        }
        inst if inst & 0xF000 == 0xD000 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            let y = ((inst & 0x00F0) >> 4) as u8;
            let n = (inst & 0x000F) as u8;
            interpreter.drw(x, y, n);
        }
        inst if inst & 0xF0FF == 0xE09E => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.skp(x);
        }
        inst if inst & 0xF0FF == 0xE0A1 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.sknp(x);
        }
        inst if inst & 0xF0FF == 0xF007 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.ld_vx_dt(x);
        }
        inst if inst & 0xF0FF == 0xF00A => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.ld_vx_k(x);
        }
        inst if inst & 0xF0FF == 0xF015 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.ld_dt_vx(x);
        }
        inst if inst & 0xF0FF == 0xF018 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.ld_st(x);
        }
        inst if inst & 0xF0FF == 0xF01E => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.add_i(x);
        }
        inst if inst & 0xF0FF == 0xF029 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.ld_f(x);
        }
        inst if inst & 0xF0FF == 0xF033 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.ld_b(x);
        }
        inst if inst & 0xF0FF == 0xF055 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.ld_i_vx(x);
        }
        inst if inst & 0xF0FF == 0xF065 => {
            let x = ((inst & 0x0F00) >> 8) as u8;
            interpreter.ld_vx_i(x);
        }
        _ => panic!("unexpected instruction: {:#06X}", inst),
    }
}
