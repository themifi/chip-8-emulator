const MEMORY_SIZE: usize = 4096;
pub const SPRITE_SIZE: usize = 5;
const SPRITE_NUM: usize = 16;
pub const SPRITE_START_LOCATION: usize = 0;
pub const PROGRAM_START_LOCATION: usize = 0x200;

static INITIAL_SPRITES: [u8; SPRITE_SIZE * SPRITE_NUM] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new_with_initial_sprites() -> Self {
        let mut memory = [0; MEMORY_SIZE];

        let sprites_chunk =
            &mut memory[SPRITE_START_LOCATION..SPRITE_START_LOCATION + INITIAL_SPRITES.len()];
        sprites_chunk.copy_from_slice(&INITIAL_SPRITES);

        Memory { memory }
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

    pub fn load_program(&mut self, program: &[u8]) {
        let start = PROGRAM_START_LOCATION;
        let finish = start + program.len();
        let program_chunk = self.get_slice_mut(start, finish);
        program_chunk.copy_from_slice(program);
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

    #[test]
    fn test_load_program() {
        let mut memory = Memory::new_with_initial_sprites();
        let test_program_code = [0x1, 0x2, 0x3];

        memory.load_program(&test_program_code);

        let program_in_memory = memory.get_slice(
            PROGRAM_START_LOCATION,
            PROGRAM_START_LOCATION + test_program_code.len(),
        );
        assert_eq!(program_in_memory, test_program_code);
    }
}
