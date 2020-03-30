pub const V_REGISTERS_SIZE: usize = 16;

#[derive(Default)]
pub struct Registers {
    pub v: [u8; V_REGISTERS_SIZE],
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: u16,
}

impl Registers {
    pub fn new() -> Self {
        Default::default()
    }
}
