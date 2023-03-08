use crate::types::*;

#[derive(Debug)]
pub enum MemoryError {
    Unaligned,
    Unmapped
}

#[macro_export]
macro_rules! impl_load {
    ($_name:ident, $_type:ty, $_check_align:ident) => (
        pub fn $_name(&self, address: Word) -> Result<$_type, MemoryError> {
            if ($_check_align && address % std::mem::size_of::<$_type>() as u32 != 0) {
                return Err(MemoryError::Unaligned)
            }

            let mut elem: $_type = 0;

            for i in 0..std::mem::size_of::<$_type>() {
                let byte = self.load8(address.wrapping_add(i as Word))? as $_type;
                elem |= byte << (i * 8);
            }

            Ok(elem)
        } 
    )
}

#[macro_export]
macro_rules! impl_store {
    ($_name:ident, $_type:ty, $_check_align:ident) => (
        pub fn $_name(&mut self, address: Word, value: $_type) -> Result<(), MemoryError> {
            if ($_check_align && address % std::mem::size_of::<$_type>() as u32 != 0) {
                return Err(MemoryError::Unaligned)
            }

            for i in 0..std::mem::size_of::<$_type>() {
                self.store8(address.wrapping_add(i as Word), (value >> (i * 8)) as u8)?;
            }

            Ok(())
        } 
    )
}

pub struct Memory {
    data: Vec<u8>
}

impl Memory {
    pub fn with_size(size: usize) -> Self {
        Self {
            data: vec![0; size]
        }
    }

    pub fn load8(&self, address: Word) -> Result<u8, MemoryError> {
        Ok(
            *self.data.get(address as usize)
            .ok_or(MemoryError::Unmapped)?
        )
    }

    impl_load!(load16_unaligned, u16, false);
    impl_load!(load32_unaligned, u32, false);
    impl_load!(load64_unaligned, u64, false);
    impl_load!(load16_aligned, u16, true);
    impl_load!(load32_aligned, u32, true);
    impl_load!(load64_aligned, u64, true);

    pub fn store8(&mut self, address: Word, value: u8) -> Result<(), MemoryError> {
        let byte_ref = self.data.get_mut(address as usize).ok_or(MemoryError::Unmapped)?;
        *byte_ref = value;
        Ok(())
    }

    impl_store!(store16_unaligned, u16, false);
    impl_store!(store32_unaligned, u32, false);
    impl_store!(store64_unaligned, u64, false);
    impl_store!(store16_aligned, u16, true);
    impl_store!(store32_aligned, u32, true);
    impl_store!(store64_aligned, u64, true);
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    pub fn test_le_loads() {
        let mut mem = Memory::with_size(64);
        mem.store8(0x00, 0xAB).unwrap();
        mem.store8(0x01, 0xCD).unwrap();
        mem.store8(0x02, 0xEF).unwrap();
        mem.store8(0x03, 0x01).unwrap();
        mem.store8(0x04, 0x23).unwrap();
        mem.store8(0x05, 0x45).unwrap();
        mem.store8(0x06, 0x67).unwrap();
        mem.store8(0x07, 0x89).unwrap();
        assert_eq!(mem.load16_unaligned(0x00).unwrap(), 0xCDAB);
        assert_eq!(mem.load32_unaligned(0x00).unwrap(), 0x01EFCDAB);
        assert_eq!(mem.load64_unaligned(0x00).unwrap(), 0x8967452301EFCDAB);
    }

    #[test]
    pub fn test_le_stores() {
        let mut mem = Memory::with_size(64);

        mem.store16_unaligned(0x10, 0xCAFE).unwrap();
        assert_eq!(mem.load8(0x10).unwrap(), 0xFE);

        mem.store32_unaligned(0x20, 0xCAFE1337).unwrap();
        assert_eq!(mem.load32_unaligned(0x20).unwrap(), 0xCAFE1337);

        mem.store64_unaligned(0x30, 0xCAFE1337DEADBEEF).unwrap();
        assert_eq!(mem.load64_unaligned(0x30).unwrap(), 0xCAFE1337DEADBEEF);
    }

    #[test]
    pub fn test_align16_check() {
        let mem = Memory::with_size(64);

        assert!(mem.load16_aligned(0x00).is_ok());
        assert!(mem.load16_aligned(0x01).is_err());
        assert!(mem.load16_aligned(0x02).is_ok());

        assert!(mem.load16_unaligned(0x01).is_ok());
    }

    #[test]
    pub fn test_align32_check() {
        let mem = Memory::with_size(64);

        assert!(mem.load32_aligned(0x00).is_ok());
        assert!(mem.load32_aligned(0x01).is_err());
        assert!(mem.load32_aligned(0x02).is_err());
        assert!(mem.load32_aligned(0x03).is_err());
        assert!(mem.load32_aligned(0x04).is_ok());
        assert!(mem.load32_aligned(0x05).is_err());
    }

    #[test]
    pub fn test_align64_check() {
        let mem = Memory::with_size(64);

        assert!(mem.load64_aligned(0x00).is_ok());
        assert!(mem.load64_aligned(0x01).is_err());
        assert!(mem.load64_aligned(0x02).is_err());
        assert!(mem.load64_aligned(0x03).is_err());
        assert!(mem.load64_aligned(0x08).is_ok());
        assert!(mem.load64_aligned(0x09).is_err());
    }

    #[test]
    pub fn test_unmapped_check() {
        let mem = Memory::with_size(0x10);

        assert!(mem.load16_unaligned(0x00).is_ok());
        assert!(mem.load16_unaligned(0x0E).is_ok());
        assert!(mem.load16_unaligned(0x0F).is_err());
    }
}