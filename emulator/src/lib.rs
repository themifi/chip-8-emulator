//! # CHIP-8 emulator
//!
//! My CHIP-8 emulator in [Rust](https://github.com/rust-lang/rust).
//!
//! # What is Chip-8?
//! Chip-8 is a simple, interpreted, programming language which was first used
//! on some do-it-yourself computer systems in the late 1970s and early 1980s.
//! The COSMAC VIP, DREAM 6800, and ETI 660 computers are a few examples. These
//! computers typically were designed to use a television as a display, had
//! between 1 and 4K of RAM, and used a 16-key hexadecimal keypad for input.
//! The interpreter took up only 512 bytes of memory, and programs, which were
//! entered into the computer in hexadecimal, were even smaller.
//!
//! In the early 1990s, the Chip-8 language was revived by a man named Andreas
//! Gustafsson. He created a Chip-8 interpreter for the HP48 graphing
//! calculator, called Chip-48. The HP48 was lacking a way to easily make fast
//! games at the time, and Chip-8 was the answer. Chip-48 later begat Super
//! Chip-48, a modification of Chip-48 which allowed higher resolution
//! graphics, as well as other graphical enhancements.

pub mod graphics;
pub mod input;
pub mod memory;
pub mod registers;
pub mod stack;
pub mod vm;
pub mod interpreter;
pub mod execute_instruction;

pub use vm::VM;
