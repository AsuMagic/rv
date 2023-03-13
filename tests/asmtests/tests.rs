use rv::{emulator::Emulator, memory::Memory, fileio, types::RegIndex};

#[test]
fn test_lui() {
    let mut emu = boot_file("lui");
    emu.step();
    assert_eq!(emu.regs.load(2 as RegIndex), 0xBEEF000);
}
