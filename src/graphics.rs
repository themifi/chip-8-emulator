pub const DISPLAY_ROWS: usize = 32;

pub struct Graphics {
    pub display: [u64; DISPLAY_ROWS],
}

impl Graphics {
    pub fn new() -> Self {
        Self {
            display: [0; DISPLAY_ROWS],
        }
    }

    pub fn clear(&mut self) {
        self.display = [0; DISPLAY_ROWS];
    }
}
