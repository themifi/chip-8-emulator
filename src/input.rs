pub struct Input {
    keypad: u16,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keypad: 0,
        }
    }
}
