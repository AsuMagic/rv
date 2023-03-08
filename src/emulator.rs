use crate::types::*;
use crate::memory::*;
use crate::registers::*;
use crate::ops::*;

#[derive(Debug)]
pub enum DecodeError {
    MemoryError(MemoryError),
    Illegal
}

impl From<MemoryError> for DecodeError {
    fn from(value: MemoryError) -> Self {
        Self::MemoryError(value)
    }
}

pub struct Emulator {
    regs: Registers,
    mem: Memory
}

impl Emulator {
    pub fn decode(&self, ip: Word) -> Result<Instruction, DecodeError> {
        let first_word: Word = self.mem.load32_aligned(ip)?;

        let first_bits = first_word & 0b1111111;
        let num_words = 1;

        // see RV32/64G Instruction Set Listings
        // the match arms should be ordered according to the order in the mentioned chapter.
        let op = match first_bits {
            0b0110111 => Op::Lui(UType::from(first_word)),
            0b0000011 => {
                let operands = IType::from(first_word);

                match operands.funct3 {
                    0b000 => Op::Lb(operands),
                    0b001 => Op::Lh(operands),
                    0b010 => Op::Lw(operands),
                    0b100 => Op::Lbu(operands),
                    0b101 => Op::Lhu(operands),
                    _ => return Err(DecodeError::Illegal)
                }
            },
            _ => return Err(DecodeError::Illegal)
        };

        Ok(Instruction {
            op,
            num_words
        })
    }

    pub fn smp_noop(&self) {
        // currently does nothing; can be edited to error out whenever one desires implementing SMP
    }

    pub fn compute_address_load_store(&self, p: &IType) -> Word {
        let source_address = self.regs.load(p.rs1);
        let offset = p.imm.sext();
        source_address.wrapping_add_signed(offset)
    }

    pub fn step(&mut self) {
        let ip = self.regs.load(Registers::PC_INDEX);

        let ins = self.decode(ip).unwrap(); // TODO: trap

        let mut next_ip = ip + (ins.num_words * 4) as Word;

        match ins.op {
            Op::Lui(p) => {
                self.regs.store(p.rd, p.imm.zext());
            }
            Op::Auipc => todo!(),
            Op::Jal => todo!(),
            Op::Jalr => todo!(),
            Op::Beq => todo!(),
            Op::Bne => todo!(),
            Op::Blt => todo!(),
            Op::Bge => todo!(),
            Op::Bltu => todo!(),
            Op::Bgeu => todo!(),
            Op::Lb(p) => {
                let computed_address = self.compute_address_load_store(&p);
                let value = self.mem.load8(computed_address).unwrap(); // TODO: handle
                self.regs.store(p.rd, value as i8 as Word);
            }
            Op::Lh(p) => {
                let computed_address = self.compute_address_load_store(&p);
                let value = self.mem.load16_unaligned(computed_address).unwrap(); // TODO: handle
                self.regs.store(p.rd, value as i16 as SignedWord as Word);
            }
            Op::Lw(p) => {
                let computed_address = self.compute_address_load_store(&p);
                let value = self.mem.load32_unaligned(computed_address).unwrap(); // TODO: handle
                self.regs.store(p.rd, value as i32 as SignedWord as Word);
            }
            Op::Lbu(p) => {
                let computed_address = self.compute_address_load_store(&p);
                let value = self.mem.load8(computed_address).unwrap(); // TODO: handle
                self.regs.store(p.rd, value as Word);
            }
            Op::Lhu(p) => {
                let computed_address = self.compute_address_load_store(&p);
                let value = self.mem.load16_unaligned(computed_address).unwrap(); // TODO: handle
                self.regs.store(p.rd, value as Word);
            }
            Op::Sb => todo!(),
            Op::Sh => todo!(),
            Op::Sw => todo!(),
            Op::Addi => todo!(),
            Op::Slti => todo!(),
            Op::Sltiu => todo!(),
            Op::Xori => todo!(),
            Op::Ori => todo!(),
            Op::Andi => todo!(),
            Op::Slli => todo!(),
            Op::Srli => todo!(),
            Op::Srai => todo!(),
            Op::Add => todo!(),
            Op::Sub => todo!(),
            Op::Sll => todo!(),
            Op::Slt => todo!(),
            Op::Sltu => todo!(),
            Op::Xor => todo!(),
            Op::Srl => todo!(),
            Op::Sra => todo!(),
            Op::Or => todo!(),
            Op::And => todo!(),
            Op::Fence => self.smp_noop(),
            Op::FenceI => self.smp_noop(),
            Op::Ecall => todo!(),
            Op::Ebreak => {
                println!("EBREAK currently is a no-op");
                println!("Dumping registers: {:?}", self.regs);
            },
            Op::Csrrw => todo!(),
            Op::Csrrs => todo!(),
            Op::Csrrc => todo!(),
            Op::Csrrwi => todo!(),
            Op::Csrrsi => todo!(),
            Op::Csrrci => todo!(),
        }

        self.regs.store(Registers::PC_INDEX, next_ip);
    }
}