use std::{process::Command, path::Path, env, fs};

use rv::{emulator::Emulator, memory::Memory, fileio};

const DEFAULT_RAM_SIZE: usize = 1024 * 1024 * 16;

pub fn assemble_flat(name: &str) -> String {
    let assembler_path = Path::new("/usr/bin/riscv64-elf-as");
    let linker_path = Path::new("/usr/bin/riscv64-elf-ld");
    let objcopy_path = Path::new("/usr/bin/riscv64-elf-objcopy");

    let source_root = Path::new("./tests/asmtests/");

    let arch_flags = [
        "-march=rv32i"
    ];

    let work_dir = env::temp_dir().join("asurv").join(name);
    fs::create_dir_all(&work_dir).unwrap();

    let source_path = source_root.join(name).with_extension("s");
    let source_path = source_path.to_string_lossy();

    let object_path = work_dir.join(name).with_extension("o");
    let object_path = object_path.to_string_lossy();

    let elf_path = work_dir.join(name).with_extension("elf");
    let elf_path = elf_path.to_string_lossy();

    let binary_path = work_dir.join(name).with_extension("bin");
    let binary_path = binary_path.to_string_lossy();

    assert!(Command::new(assembler_path)
        .arg(source_path.as_ref())
        .args(arch_flags)
        .args(["-o", object_path.as_ref()])
        .status()
        .expect("Failed to assemble program")
        .success()
    );

    assert!(Command::new(linker_path)
        .arg(object_path.as_ref())
        .args(arch_flags)
        .args(["-o", elf_path.as_ref()])
        .args(["-T", "tests/basiclink.ld"])
        .arg("-melf32lriscv")
        .status()
        .expect("Failed to link assembled program")
        .success()
    );

    assert!(Command::new(objcopy_path)
        .arg(elf_path.as_ref())
        .arg(binary_path.as_ref())
        .args(["-O", "binary"])
        .status()
        .expect("Failed to extract binary from ELF")
        .success()
    );

    binary_path.as_ref().into()
}

pub fn boot_file(name: &str) -> Emulator {
    let mut emu = Emulator::new(Memory::with_size(DEFAULT_RAM_SIZE));
    fileio::use_bootrom(&mut emu, Path::new(&assemble_flat(name))).unwrap();
    emu
}