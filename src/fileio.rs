use std::{path::Path, fs::File, io::{self, Read}};

use crate::{emulator::Emulator, types::Word};

pub fn use_bootrom(emulator: &mut Emulator, path: &Path) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let bytes = io::BufReader::new(file).bytes();

    for (i, byte) in bytes.into_iter().enumerate() {
        let base_address = 0 as Word;
        emulator.mem.store8(base_address.wrapping_add(i as Word), byte.unwrap()).unwrap();
    }

    Ok(())
}