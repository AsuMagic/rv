use crate::bitutil::*;
use crate::types::*;

pub fn decode_rd(encoded: EncodedInstruction) -> RegIndex {
    extract_bits(encoded, 7, 11) as RegIndex
}

pub fn decode_rs1(encoded: EncodedInstruction) -> RegIndex {
    extract_bits(encoded, 15, 19)as RegIndex
}

pub fn decode_rs2(encoded: EncodedInstruction) -> RegIndex {
    extract_bits(encoded, 20, 24) as RegIndex
}

pub struct RType {
    pub rd: RegIndex,
    pub funct3: u8,
    pub rs1: RegIndex,
    pub rs2: RegIndex,
    pub funct7: u8
}

impl RType {
    pub fn from(encoded: EncodedInstruction) -> Self {
        Self {
            rd: decode_rd(encoded),
            funct3: extract_bits(encoded, 12, 14) as u8,
            rs1: decode_rs1(encoded),
            rs2: decode_rs2(encoded),
            funct7: extract_bits(encoded, 25, 31) as u8 
        }
    }
}

pub struct IType {
    pub rd: RegIndex,
    pub funct3: u8,
    pub rs1: RegIndex,
    pub imm: FieldBuilder
}

impl IType {
    pub fn from(encoded: EncodedInstruction) -> Self {
        Self {
            rd: decode_rd(encoded),
            funct3: extract_bits(encoded, 12, 14) as u8,
            rs1: decode_rs1(encoded),
            imm: FieldBuilder::default().add_bits(encoded, 20, 31)
        }
    }
}

pub struct UType {
    pub rd: RegIndex,
    pub imm: FieldBuilder,
}

impl UType {
    pub fn from(encoded: EncodedInstruction) -> Self {
        Self {
            rd: decode_rd(encoded),
            imm: FieldBuilder::default().skip_bits(12).add_bits(encoded, 12, 31)
        }
    }
}