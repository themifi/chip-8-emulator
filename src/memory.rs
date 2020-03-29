const MEMORY_SIZE: usize = 4096;
pub const SPRITE_SIZE: usize = 5;
const SPRITE_NUM: usize = 16;
pub const SPRITE_START_LOCATION: usize = 0;

static INITIAL_SPRITES: [u8; SPRITE_SIZE * SPRITE_NUM] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new_with_initial_sprites() -> Self {
        let mut memory = [0; MEMORY_SIZE];

        let sprites_chunk = &mut memory[SPRITE_START_LOCATION..SPRITE_START_LOCATION + INITIAL_SPRITES.len()];
        sprites_chunk.copy_from_slice(&INITIAL_SPRITES);

        Memory {
            memory,
        }
    }

    pub fn get_slice(&self, start: usize, finish: usize) -> &[u8] {
        assert!(start < MEMORY_SIZE);
        assert!(finish < MEMORY_SIZE);
        &self.memory[start..finish]
    }

    pub fn get_slice_mut(&mut self, start: usize, finish: usize) -> &mut [u8] {
        assert!(start < MEMORY_SIZE);
        assert!(finish < MEMORY_SIZE);
        &mut self.memory[start..finish]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_memory_with_initial_sprites() {
        let memory = Memory::new_with_initial_sprites();
        for (i, &byte) in memory.memory[0..80].iter().enumerate() {
            assert_eq!(byte, INITIAL_SPRITES[i]);
        }
        assert!(memory.memory[80..].iter().all(|&byte| byte == 0));
    }
}
