/// CHIP-8 interpreter interface.
pub trait Interpreter {
    /// Return from a subroutine.
    ///
    /// Code: `00EE`
    ///
    /// The interpreter sets the program counter to the address at the top of
    /// the stack, then subtracts 1 from the stack pointer.
    fn ret(&mut self);

    /// Jump to location `addr`.
    ///
    /// Code: `1nnn`
    ///
    /// The interpreter sets the program counter to `addr`.
    fn jp(&mut self, addr: u16);

    /// Clear the display.
    ///
    /// Code: `00E0`
    fn cls(&mut self);

    /// Call subroutine at `addr`.
    ///
    /// Code: `2nnn`
    ///
    /// The interpreter increments the stack pointer, then puts the current
    /// program counter on the top of the stack. The program counter is then
    /// set to `addr`.
    fn call(&mut self, addr: u16);

    /// Skip next instruction if `Vx` = `value`.
    ///
    /// Code: `3xkk`
    ///
    /// The interpreter compares register `Vx` to `value`, and if they are
    /// equal, increments the program counter by 2.
    fn se(&mut self, x: u8, value: u8);

    /// Skip next instruction if `Vx` != `value`.
    ///
    /// Code: `4xkk`
    ///
    /// The interpreter compares register `Vx` to `value`, and if they are not
    /// equal, increments the program counter by 2.
    fn sne(&mut self, x: u8, value: u8);

    /// Skip next instruction if `Vx` = `Vy`.
    ///
    /// Code: `5xy0`
    ///
    /// The interpreter compares register `Vx` to register `Vy`, and if they
    /// are equal, increments the program counter by 2.
    fn se_v(&mut self, x: u8, y: u8);

    /// Set `Vx` = `value`.
    ///
    /// Code: `6xkk`
    ///
    /// The interpreter puts the value `value` into register `Vx`.
    fn ld_vx(&mut self, x: u8, value: u8);

    /// Set `Vx` = `Vx` + `value`.
    ///
    /// Code: `7xkk`
    ///
    /// Adds the value `value` to the value of register `Vx`, then stores the
    /// result in `Vx`.
    fn add_vx(&mut self, x: u8, value: u8);

    /// Set `Vx` = `Vy`.
    ///
    /// Code: `8xy0`
    ///
    /// Stores the value of register `Vy` in register `Vx`.
    fn ld_vx_vy(&mut self, x: u8, y: u8);

    /// Set `Vx` = `Vx` OR `Vy`.
    ///
    /// Code: `8xy1`
    ///
    /// Performs a bitwise OR on the values of `Vx` and `Vy`, then stores the
    /// result in `Vx`. A bitwise OR compares the corrseponding bits from two
    /// values, and if either bit is 1, then the same bit in the result is also
    /// 1. Otherwise, it is 0.
    fn or(&mut self, vx: u8, vy: u8);

    /// Set `Vx` = `Vx` AND `Vy`.
    ///
    /// Code: `8xy2`
    ///
    /// Performs a bitwise AND on the values of `Vx` and `Vy`, then stores the
    /// result in `Vx`. A bitwise AND compares the corrseponding bits from two
    /// values, and if both bits are 1, then the same bit in the result is also
    /// 1. Otherwise, it is 0.
    fn and(&mut self, x: u8, y: u8);

    /// Set `Vx` = `Vx` XOR `Vy`.
    ///
    /// Code: `8xy3`
    ///
    /// Performs a bitwise exclusive OR on the values of `Vx` and `Vy`, then
    /// stores the result in `Vx`. An exclusive OR compares the corrseponding
    /// bits from two values, and if the bits are not both the same, then the
    /// corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn xor(&mut self, vx: u8, vy: u8);

    /// Set `Vx` = `Vx` + `Vy`, set `VF` = carry.
    ///
    /// Code: `8xy4`
    ///
    /// The values of `Vx` and `Vy` are added together. If the result is greater
    /// than 8 bits (i.e., > 255,) `VF` is set to 1, otherwise 0. Only the
    /// lowest 8 bits of the result are kept, and stored in `Vx`.
    fn add_vx_vy(&mut self, x: u8, y: u8);

    /// Set `Vx` = `Vx` - `Vy`, set `VF` = NOT borrow.
    ///
    /// Code: `8xy5`
    ///
    /// If `Vx` > `Vy`, then `VF` is set to 1, otherwise 0. Then `Vy` is
    /// subtracted from `Vx`, and the results stored in `Vx`.
    fn sub(&mut self, x: u8, y: u8);

    /// Set `Vx` = `Vx` SHR 1.
    ///
    /// Code: `8xy6`
    ///
    /// If the least-significant bit of `Vx` is 1, then `VF` is set to 1,
    /// otherwise 0. Then `Vx` is divided by 2.
    fn shr(&mut self, x: u8);

    /// Set `Vx` = `Vy` - `Vx`, set `VF` = NOT borrow.
    ///
    /// Code: `8xy7`
    ///
    /// If `Vy` > `Vx`, then `VF` is set to 1, otherwise 0. Then `Vx` is
    /// subtracted from `Vy`, and the results stored in `Vx`.
    fn subn(&mut self, x: u8, y: u8);

    /// Set `Vx` = `Vx` SHL 1.
    ///
    /// Code: `8xyE`
    ///
    /// If the most-significant bit of `Vx` is 1, then `VF` is set to 1,
    /// otherwise to 0. Then `Vx` is multiplied by 2.
    fn shl(&mut self, x: u8);

    /// Skip next instruction if `Vx` != `Vy`.
    ///
    /// Code: `9xy0`
    ///
    /// The values of `Vx` and `Vy` are compared, and if they are not equal,
    /// the program counter is increased by 2.
    fn sne_vx_vy(&mut self, x: u8, y: u8);

    /// Set `I` = `value`.
    ///
    /// Code: `Annn`
    ///
    /// The value of register `I` is set to `value`.
    fn ld_i(&mut self, value: u16);

    /// Jump to location `addr` + `V0`.
    ///
    /// Code: `Bnnn`
    ///
    /// The program counter is set to `addr` plus the value of `V0`.
    fn jp_v0(&mut self, addr: u16);

    /// Set `Vx` = random byte AND `mask`.
    ///
    /// Code: `Cxkk`
    ///
    /// The interpreter generates a random number from 0 to 255, which is then
    /// ANDed with the value `mask`. The results are stored in `Vx`. See
    /// instruction `8xy2` for more information on AND.
    fn rnd(&mut self, x: u8, mask: u8);

    /// Display `n`-byte sprite starting at memory location `I` at (`Vx`, `Vy`),
    /// set `VF` = collision.
    ///
    /// Code: `Dxyn`
    ///
    /// The interpreter reads `n` bytes from memory, starting at the address
    /// stored in `I`. These bytes are then displayed as sprites on screen at
    /// coordinates (`Vx`, `Vy`). Sprites are XORed onto the existing screen.
    /// If this causes any pixels to be erased, `VF` is set to 1, otherwise it
    /// is set to 0. If the sprite is positioned so part of it is outside the
    /// coordinates of the display, it wraps around to the opposite side of the
    /// screen. See instruction `8xy3` for more information on XOR, and section
    /// Display for more information on the Chip-8 screen and sprites.
    fn drw(&mut self, x: u8, y: u8, n: u8);

    /// Skip next instruction if key with the value of `Vx` is pressed.
    ///
    /// Code: `Ex9E`
    ///
    /// Checks the keyboard, and if the key corresponding to the value of `Vx`
    /// is currently in the down position, program counter is increased by 2.
    fn skp(&mut self, x: u8);

    /// Skip next instruction if key with the value of `Vx` is not pressed.
    ///
    /// Code: `ExA1`
    ///
    /// Checks the keyboard, and if the key corresponding to the value of `Vx`
    /// is currently in the up position, program counter is increased by 2.
    fn sknp(&mut self, x: u8);

    /// Set `Vx` = delay timer value.
    ///
    /// Code: `Fx07`
    ///
    /// The value of delay timer is placed into `Vx`.
    fn ld_vx_dt(&mut self, x: u8);

    /// Set delay timer = `Vx`.
    ///
    /// Code: `Fx15`
    ///
    /// Delay timer is set equal to the value of `Vx`.
    fn ld_dt_vx(&mut self, x: u8);

    /// Wait for a key press, store the value of the key in `Vx`.
    ///
    /// Code: `Fx0A`
    ///
    /// All execution stops until a key is pressed, then the value of that key
    /// is stored in `Vx`.
    fn ld_vx_k(&mut self, x: u8);

    /// Set sound timer = `Vx`.
    ///
    /// Code: `Fx18`
    ///
    /// Sound timer is set equal to the value of `Vx`.
    fn ld_st(&mut self, x: u8);

    /// Set `I` = `I` + `Vx`.
    ///
    /// Code: `Fx1E`
    ///
    /// The values of `I` and `Vx` are added, and the results are stored in `I`.
    fn add_i(&mut self, x: u8);

    /// Set `I` = location of sprite for digit `Vx`.
    ///
    /// Code: `Fx29`
    ///
    /// The value of `I` is set to the location for the hexadecimal sprite
    /// corresponding to the value of `Vx`. See section Display for more
    /// information on the Chip-8 hexadecimal font.
    fn ld_f(&mut self, x: u8);

    /// Store BCD representation of `Vx` in memory locations `I`, `I+1`, and
    /// `I+2`.
    ///
    /// Code: `Fx33`
    ///
    /// The interpreter takes the decimal value of `Vx`, and places the
    /// hundreds digit in memory at location in `I`, the tens digit at location
    /// `I+1`, and the ones digit at location `I+2`.
    fn ld_b(&mut self, x: u8);

    /// Store registers `V0` through `Vx` in memory starting at location `I`.
    ///
    /// Code: `Fx55`
    ///
    /// The interpreter copies the values of registers `V0` through `Vx` into
    /// memory, starting at the address in `I`.
    fn ld_i_vx(&mut self, x: u8);

    /// Read registers `V0` through `Vx` from memory starting at location `I`.
    ///
    /// Code: `Fx65`
    ///
    /// The interpreter reads values from memory starting at location `I` into
    /// registers `V0` through `Vx`.
    fn ld_vx_i(&mut self, x: u8);
}
