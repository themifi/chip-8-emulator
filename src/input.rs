const KEYS: u8 = 16;

#[derive(Default)]
pub struct Input {
    key_pressed: Option<u8>,
}

impl Input {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with_key_pressed(key: u8) -> Self {
        assert!(key < KEYS);
        Self {
            key_pressed: Some(key),
        }
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        self.key_pressed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_key_pressed_clear_state() {
        let input = Input::new();
        assert_eq!(input.get_pressed_key(), None);
    }

    #[test]
    fn test_is_key_pressed_with_key_pressed() {
        for pressed_key in 0..KEYS {
            let input = Input::new_with_key_pressed(pressed_key);
            assert_eq!(input.get_pressed_key(), Some(pressed_key));
        }
    }

    #[test]
    #[should_panic]
    fn test_is_key_pressed_invalid_input() {
        Input::new_with_key_pressed(KEYS);
    }
}
