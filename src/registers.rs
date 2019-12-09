const V_REGISTERS_SIZE: usize = 16;

pub struct Registers {
    pub v: [u8; V_REGISTERS_SIZE],
    pub i: u16,
    pub delay_timer: u16,
    pub sound_timer: u16,
    pub program_counter: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            v: [0; V_REGISTERS_SIZE],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
        }
    }
}
