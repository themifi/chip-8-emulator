pub const DISPLAY_ROWS: usize = 32;
const DISPLAY_COLS: usize = 64;

#[derive(Default)]
pub struct Graphics {
    pub display: [u64; DISPLAY_ROWS],
}

impl Graphics {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clear(&mut self) {
        self.display = [0; DISPLAY_ROWS];
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        assert!(x < DISPLAY_COLS);
        assert!(y < DISPLAY_ROWS);

        let mut is_collision = false;

        for (i, sprite_row) in sprite.iter().enumerate() {
            let row = *sprite_row as u64;
            let row = row.rotate_left(x as u32);
            let row_y = (y + i) % DISPLAY_ROWS;
            is_collision = is_collision || (self.display[row_y] & row) != 0;
            self.display[row_y] ^= row;
        }

        is_collision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_sprite() {
        let mut graphics = Graphics::new();
        let sprite = [0x20, 0x60, 0x20, 0x20, 0x70];
        let is_collision = graphics.draw_sprite(8, 2, &sprite);
        assert_eq!(graphics.display[0..9], [0, 0, 0x2000, 0x6000, 0x2000, 0x2000, 0x7000, 0, 0]);
        assert!(!is_collision);
    }

    #[test]
    #[should_panic]
    fn test_draw_sprite_incorrect_input_x() {
        let mut graphics = Graphics::new();
        graphics.draw_sprite(DISPLAY_COLS, 2, &[]);
    }

    #[test]
    #[should_panic]
    fn test_draw_sprite_incorrect_input_y() {
        let mut graphics = Graphics::new();
        graphics.draw_sprite(0, DISPLAY_ROWS, &[]);
    }

    #[test]
    fn test_draw_sprite_wrapping_x() {
        let mut graphics = Graphics::new();
        let sprite = [0xFF];
        let is_collision = graphics.draw_sprite(60, 0, &sprite);
        assert_eq!(graphics.display[0..2], [0xF00000000000000F, 0]);
        assert!(!is_collision);
    }

    #[test]
    fn test_draw_sprite_wrapping_y() {
        let mut graphics = Graphics::new();
        let sprite = [0xFF, 0xFF];
        let is_collision = graphics.draw_sprite(0, 31, &sprite);
        assert_eq!(graphics.display[0..2], [0xFF, 0]);
        assert_eq!(graphics.display[30..32], [0, 0xFF]);
        assert!(!is_collision);
    }

    #[test]
    fn test_draw_sprite_wrapping_xy() {
        let mut graphics = Graphics::new();
        let sprite = [0xFF, 0xFF];
        let is_collision = graphics.draw_sprite(60, 31, &sprite);
        assert_eq!(graphics.display[0..2], [0xF00000000000000F, 0]);
        assert_eq!(graphics.display[30..32], [0, 0xF00000000000000F]);
        assert!(!is_collision);
    }

    #[test]
    fn test_draw_sprite_collision() {
        let mut graphics = Graphics::new();
        graphics.display[0] = 0b11011100;
        let sprite = [0b01000011];
        let is_collision = graphics.draw_sprite(0, 0, &sprite);
        assert_eq!(graphics.display[0], 0b10011111);
        assert!(is_collision);
    }

    #[test]
    fn test_draw_sprite_collision_one_bit() {
        let mut graphics = Graphics::new();
        graphics.display[0] = 0x1;
        let sprite = [0x1];
        let is_collision = graphics.draw_sprite(0, 0, &sprite);
        assert_eq!(graphics.display[0], 0x0);
        assert!(is_collision);
    }
}
