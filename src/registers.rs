use crate::types::*;

#[derive(Debug)]
pub struct Registers {
    words: [Word; 33] // x0-x31 + pc
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            words: [0; 33]
        }
    }
}

impl Registers {
    pub fn load(&self, index: RegIndex) -> Word {
        self.words[index as usize]
    }

    pub fn store(&mut self, index: RegIndex, value: Word) {
        // r0 is no-op to value "0". prevent stores so that it is always == 0
        if index == 0 { return }

        self.words[index as usize] = value;
    }

    pub const PC_INDEX: RegIndex = 32;
}