use crate::types::*;

pub fn extract_bits(encoded: EncodedInstruction, start_bit: usize, end_bit: usize) -> EncodedInstruction {
    let word_bits = std::mem::size_of::<EncodedInstruction>() * 8;
    let num_bits = end_bit - start_bit + 1;

    (encoded << (word_bits - end_bit - 1)) >> (word_bits - num_bits)
}

pub fn sext_field(value: EncodedInstruction, field_size: usize) -> i32 {
    let word_bits = std::mem::size_of::<EncodedInstruction>() * 8;

    (value << (word_bits - field_size)) as i32 >> (word_bits - field_size)
}

#[derive(Default)]
pub struct FieldBuilder {
    value: EncodedInstruction,
    bit_count: usize
}

impl FieldBuilder {
    pub fn add_bits(mut self, encoded: EncodedInstruction, start_bit: usize, end_bit: usize) -> Self {
        self.value |= extract_bits(encoded, start_bit, end_bit) << self.bit_count;
        self.skip_bits(end_bit - start_bit + 1)
    }

    pub fn skip_bits(mut self, count: usize) -> Self {
        self.bit_count += count;
        debug_assert!(self.bit_count <= std::mem::size_of::<EncodedInstruction>() * 8);
        self
    }

    pub fn zext(&self) -> u32 {
        self.value
    }

    pub fn sext(&self) -> i32 {
        sext_field(self.value, self.bit_count)
    }
}

#[cfg(test)]
mod tests {
    use super::FieldBuilder;
    use super::extract_bits;

    #[test]
    fn test_extract_bits() {
        assert_eq!(extract_bits(0b110101001, 3, 5), 0b101);
    }

    #[test]
    fn test_builder() {
        let simple_test = FieldBuilder::default()
            .skip_bits(2)
            .add_bits(0b11101111, 3, 5);

        assert_eq!(simple_test.zext(), 0b10100);
        assert_eq!(simple_test.sext(), 0b11110100u8 as i8 as i32); // poor man's sign-extend test
    }
}