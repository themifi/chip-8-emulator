const KEYS: u8 = 16;

#[derive(Default)]
pub struct Input {
    keypad: u16,
}

impl Input {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with_state(keypad: u16) -> Self {
        Self {
            keypad,
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        assert!(key < KEYS);
        ((1 << key) & self.keypad) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_key_pressed_clear_state() {
        let input = Input::new();
        for key in 0..KEYS {
            assert!(!input.is_key_pressed(key));
        }
    }

    #[test]
    fn test_is_key_pressed_with_key_pressed() {
        for pressed_key in 0..KEYS {
            let input = Input::new_with_state(1 << pressed_key);
            for key in 0..KEYS {
                assert_eq!(input.is_key_pressed(key), pressed_key == key);
            }
        }
    }

    #[test]
    fn test_is_key_pressed_with_all_keys_pressed() {
        let input = Input::new_with_state(0xFFFF);
        for key in 0..KEYS {
            assert!(input.is_key_pressed(key));
        }
    }

    #[test]
    #[should_panic]
    fn test_is_key_pressed_invalid_input() {
        let input = Input::new();
        input.is_key_pressed(KEYS);
    }
}
