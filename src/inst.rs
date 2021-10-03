use crate::addr::{Addr6502, AddrMode};
use crate::cpu::{CPU,Flags,CpuFlags};

#[derive(Debug,PartialEq,Eq)]
pub enum Mnemonic {
    XXX, ADC, AND, ASL, BCC,
    BCS, BEQ, BIT, BMI, BNE,
    BPL, BRK, BVC, BVS, CLC,
    CLD, CLI, CLV, CMP, CPX,
    CPY, DEC, DEX, DEY, EOR,
    INC, INX, INY, JMP, JSR,
    LDA, LDX, LDY, LSR, NOP,
    ORA, PHA, PHP, PLA, PLP,
    ROL, ROR, RTI, RTS, SBC,
    SEC, SED, SEI, STA, STX,
    STY, TAX, TAY, TSX, TXA,
    TXS, TYA,
}

pub struct Inst {
    pub mnemonic: Mnemonic,
    pub mode: AddrMode,
    pub length: u8,
    pub cycles: u8,
}

// illegal / unofficial instructions are not complete
pub static INSTRUCTIONS: [Inst; 0x100] = [
    Inst { mnemonic: Mnemonic::BRK, mode: AddrMode::Imp,        length: 1, cycles: 7 },
    Inst { mnemonic: Mnemonic::ORA, mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::ORA, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::ASL, mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::PHP, mode: AddrMode::Imp,        length: 1, cycles: 3 },
    Inst { mnemonic: Mnemonic::ORA, mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::ASL, mode: AddrMode::Imp,        length: 1, cycles: 2 }, // ACC
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::ORA, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::ASL, mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },

    Inst { mnemonic: Mnemonic::BPL, mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::ORA, mode: AddrMode::IndY,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::ORA, mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::ASL, mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::CLC, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::ORA, mode: AddrMode::AbsY,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::ORA, mode: AddrMode::AbsX,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::ASL, mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: Mnemonic::JSR, mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: Mnemonic::AND, mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::BIT, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::AND, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::ROL, mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::PLP, mode: AddrMode::Imp,        length: 1, cycles: 4 },
    Inst { mnemonic: Mnemonic::AND, mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::ROL, mode: AddrMode::Imp,        length: 1, cycles: 2 }, // ACC
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::BIT, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::AND, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::ROL, mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },

    Inst { mnemonic: Mnemonic::BMI, mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::AND, mode: AddrMode::IndY,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::AND, mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::ROL, mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::SEC, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::AND, mode: AddrMode::AbsY,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::AND, mode: AddrMode::AbsX,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::ROL, mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: Mnemonic::RTI, mode: AddrMode::Imp,        length: 1, cycles: 6 },
    Inst { mnemonic: Mnemonic::EOR, mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::EOR, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::LSR, mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::PHA, mode: AddrMode::Imp,        length: 1, cycles: 3 },
    Inst { mnemonic: Mnemonic::EOR, mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::LSR, mode: AddrMode::Imp,        length: 1, cycles: 2 }, // ACC
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::JMP, mode: AddrMode::Abs,        length: 3, cycles: 3 },
    Inst { mnemonic: Mnemonic::EOR, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::LSR, mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },

    Inst { mnemonic: Mnemonic::BVC, mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::EOR, mode: AddrMode::IndY,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::EOR, mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::LSR, mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::CLI, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::EOR, mode: AddrMode::AbsY,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::EOR, mode: AddrMode::AbsX,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::LSR, mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: Mnemonic::RTS, mode: AddrMode::Imp,        length: 1, cycles: 6 },
    Inst { mnemonic: Mnemonic::ADC, mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::ADC, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::ROR, mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::PLA, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::ADC, mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::ROR, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::JMP, mode: AddrMode::Ind,        length: 3, cycles: 5 },
    Inst { mnemonic: Mnemonic::ADC, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::ROR, mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },

    Inst { mnemonic: Mnemonic::BVS, mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::ADC, mode: AddrMode::IndY,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::ADC, mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::ROR, mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::SEI, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::ADC, mode: AddrMode::AbsY,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::ADC, mode: AddrMode::AbsX,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::ROR, mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::STA, mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::STY, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::STA, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::STX, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::DEY, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::TXA, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::STY, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::STA, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::STX, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 4 },

    Inst { mnemonic: Mnemonic::BCC, mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::STA, mode: AddrMode::IndY,       length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::STY, mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::STA, mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::STX, mode: AddrMode::ZeroY,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::TYA, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::STA, mode: AddrMode::AbsY,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::TXS, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::STA, mode: AddrMode::AbsX,       length: 3, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 5 },

    Inst { mnemonic: Mnemonic::LDY, mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::LDA, mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::LDX, mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::LDY, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::LDA, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::LDX, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::TAY, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::LDA, mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::TAX, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::LDY, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::LDA, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::LDX, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 3, cycles: 4 },

    Inst { mnemonic: Mnemonic::BCS, mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::LDA, mode: AddrMode::IndY,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::LDY, mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::LDA, mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::LDX, mode: AddrMode::ZeroY,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::CLV, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::LDA, mode: AddrMode::AbsY,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::TSX, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::LDY, mode: AddrMode::AbsX,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::LDA, mode: AddrMode::AbsX,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::LDX, mode: AddrMode::AbsY,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 4 },

    Inst { mnemonic: Mnemonic::CPY, mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::CMP, mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::CPY, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::CMP, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::DEC, mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::INY, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::CMP, mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::DEX, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::CPY, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::CMP, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::DEC, mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: Mnemonic::BNE, mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::CMP, mode: AddrMode::IndY,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::CMP, mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::DEC, mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::CLD, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::CMP, mode: AddrMode::AbsY,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::CMP, mode: AddrMode::AbsX,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::DEC, mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: Mnemonic::CPX, mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::SBC, mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::CPX, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::SBC, mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: Mnemonic::INC, mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::INX, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::SBC, mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::CPX, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::SBC, mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::INC, mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },

    Inst { mnemonic: Mnemonic::BEQ, mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::SBC, mode: AddrMode::IndY,       length: 2, cycles: 5 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::SBC, mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::INC, mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: Mnemonic::SED, mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: Mnemonic::SBC, mode: AddrMode::AbsY,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: Mnemonic::NOP, mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: Mnemonic::SBC, mode: AddrMode::AbsX,       length: 3, cycles: 4 },
    Inst { mnemonic: Mnemonic::INC, mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: Mnemonic::XXX, mode: AddrMode::Imp,        length: 2, cycles: 7 },
];

// http://www.6502.org/tutorials/6502opcodes.html
pub trait Inst6502 {
    // run operation
    fn execute_op(&mut self);

    fn jump_if(&mut self, pred: bool) -> bool;

    // the boolean return value indicates
    // whether or not the operation has a possibility
    // of an additional cycle

    // bitwise AND with accumulator
    fn adc(&mut self) -> bool;

    // arithmetic shift left
    fn asl(&mut self) -> bool;

    // test bits
    fn bit(&mut self) -> bool;

    // branch instructions
    fn bpl(&mut self) -> bool;
    fn bmi(&mut self) -> bool;
    fn bvc(&mut self) -> bool;
    fn bvs(&mut self) -> bool;
    fn bcc(&mut self) -> bool;
    fn bcs(&mut self) -> bool;
    fn bne(&mut self) -> bool;
    fn beq(&mut self) -> bool;

    // break
    fn brk(&mut self) -> bool;

    // comparse accumulator
    fn cmp(&mut self) -> bool;

    // comparse x register
    fn cpx(&mut self) -> bool;

    // comparse y register
    fn cpy(&mut self) -> bool;

    // decrement memory
    fn dec(&mut self) -> bool;

    // bitwise xor
    fn eor(&mut self) -> bool;

    // flag instructions
    fn clc(&mut self) -> bool;
    fn sec(&mut self) -> bool;
    fn cli(&mut self) -> bool;
    fn sei(&mut self) -> bool;
    fn clv(&mut self) -> bool;
    fn cld(&mut self) -> bool;
    fn sed(&mut self) -> bool;

    // increment memory
    fn inc(&mut self) -> bool;

    // jump
    fn jmp(&mut self) -> bool;

    // jump to subroutine
    fn jsr(&mut self) -> bool;

    // load accumulator
    fn lda(&mut self) -> bool;

    // load y register
    fn ldx(&mut self) -> bool;

    // load x register
    fn ldy(&mut self) -> bool;

    // logical shift right
    fn lsr(&mut self) -> bool;

    // no-op
    fn nop(&mut self) -> bool;
    fn xxx(&mut self) -> bool;

    // bistwise OR with accumulator
    fn ora(&mut self) -> bool;

    // register instructions
    fn tax(&mut self) -> bool;
    fn txa(&mut self) -> bool;
    fn dex(&mut self) -> bool;
    fn inx(&mut self) -> bool;
    fn tay(&mut self) -> bool;
    fn tya(&mut self) -> bool;
    fn dey(&mut self) -> bool;
    fn iny(&mut self) -> bool;

    // rotate left
    fn rol(&mut self) -> bool;

    // rotate right
    fn ror(&mut self) -> bool;

    // return from interrupt
    fn rti(&mut self) -> bool;

    // return from subroutine
    fn rts(&mut self) -> bool;

    // subtract with carry
    fn sbc(&mut self) -> bool;

    // store accumulator
    fn sta(&mut self) -> bool;

    // stack instructions
    fn txs(&mut self) -> bool;
    fn tsx(&mut self) -> bool;
    fn pha(&mut self) -> bool;
    fn pla(&mut self) -> bool;
    fn php(&mut self) -> bool;
    fn plp(&mut self) -> bool;

    // store x register
    fn stx(&mut self) -> bool;

    // store y register
    fn sty(&mut self) -> bool;
}

impl Inst6502 for CPU {
    fn execute_op(&mut self) {
        todo!()
    }

    fn jump_if(&mut self, pred: bool) -> bool {
        if pred {
            // +1 if branch occurs
            self.cycles += 1;

            // NOTE: jump_offset is set in the REL function
            let addr = self.pc + self.jump_offset;

            // +1 if jump to different page
            if (addr & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.eff_addr = addr;
        }

        false
    }

    // ADC: Add with carry
    // Affects: N V Z C
    fn adc(&mut self) -> bool {
        self.fetch();

        let acc: u16 = self.acc.into();
        let fetched: u16 = self.fetched.into();

        let mut sum: u16 = acc + fetched;
        sum += if self.flags.carry { 1 } else { 0 };

        self.flags.set(Flags::N, sum & 0x80 != 0);

        // http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        self.flags.set(Flags::V, (!(acc ^ fetched) & (acc ^ sum) & 0x80) != 0);
        self.flags.set(Flags::Z, sum == 0);
        self.flags.set(Flags::C, sum > 0xFF);

        self.acc = (sum & 0x00FF) as u8;

        true
    }

    // ASL: Shift left one bit
    // Affects: N Z C
    fn asl(&mut self) -> bool {
        self.fetch();

        let shifted = self.fetched << 1;

        self.flags.set(Flags::N, (shifted & 0x80) != 0);
        self.flags.set(Flags::Z, shifted == 0);
        self.flags.set(Flags::C, (self.fetched & 0x80) != 0);

        let opcode = self.opcode as usize;
        if INSTRUCTIONS[opcode].mode == AddrMode::Imp {
            self.acc = shifted & 0x00FF;
        } else {
            self.write(self.eff_addr, shifted & 0x00FF);
        }

        false
    }

    // BIT: Test bits in memory with accumulator
    // Affects: N Z V
    fn bit(&mut self) -> bool {
        self.fetch();

        let and = self.acc & self.fetched;

        self.flags.set(Flags::Z, and == 0);
        self.flags.set(Flags::N, and & (1 << 7) != 0);
        self.flags.set(Flags::V, and & (1 << 6) != 0);

        false
    }

    // BPL: Branch on result plus
    fn bpl(&mut self) -> bool {
        // a manual at masswerk and several implementations
        // jump when the N flag is not set
        self.jump_if(!self.flags.negative)
    }

    // BMI: Branch on result minus
    fn bmi(&mut self) -> bool {
        self.jump_if(self.flags.negative)
    }

    // BVC: Branch on overflow clear
    fn bvc(&mut self) -> bool {
        self.jump_if(!self.flags.overflow)
    }

    // BVS: Branch on overflow set
    fn bvs(&mut self) -> bool {
        self.jump_if(self.flags.overflow)
    }

    // BCC: Branch on carry clear
    fn bcc(&mut self) -> bool {
        self.jump_if(!self.flags.carry)
    }

    // BCS: Branch on carry set
    fn bcs(&mut self) -> bool {
        self.jump_if(self.flags.carry)
    }

    // BNE: Branch on result not zero
    fn bne(&mut self) -> bool {
        self.jump_if(!self.flags.zero)
    }

    // BEQ: Branch on result zero
    fn beq(&mut self) -> bool {
        self.jump_if(self.flags.zero)
    }

    // BRK: Force break
    // BRK initiates a software interrupt similar to a hardware
    // interrupt (IRQ). The return address pushed to the stack is
    // PC + 2
    // https://wiki.nesdev.org/w/index.php?title=CPU_interrupts
    fn brk(&mut self) -> bool {
        self.pc += 1;
        self.flags.set(Flags::I, true);

        self.push(((self.pc >> 8) & 0x00FF) as u8);
        self.push((self.pc & 0x00FF) as u8);

        // we're pushing the status flag in a S/W context
        self.flags.set(Flags::B, true);

        todo!()
    }

    // CMP: Compare memory with accumulator
    // Affects: N Z C
    fn cmp(&mut self) -> bool {
        self.fetch();

        let sub = self.acc.wrapping_sub(self.fetched);

        self.flags.set(Flags::N, sub & 0x80 != 0);
        self.flags.set(Flags::Z, sub == 0);
        self.flags.set(Flags::C, self.acc >= self.fetched);

        true
    }

    // CPX: Compare X register
    // Affects: N Z C
    fn cpx(&mut self) -> bool {
        self.fetch();

        let sub = self.x.wrapping_sub(self.fetched);

        self.flags.set(Flags::N, sub & 0x80 != 0);
        self.flags.set(Flags::Z, sub == 0);
        self.flags.set(Flags::C, self.x >= self.fetched);

        true
    }

    // CPY: Compare Y register
    // Affects: N Z C
    fn cpy(&mut self) -> bool {
        self.fetch();

        let sub = self.y.wrapping_sub(self.fetched);

        self.flags.set(Flags::N, sub & 0x80 != 0);
        self.flags.set(Flags::Z, sub == 0);
        self.flags.set(Flags::C, self.y >= self.fetched);

        true
    }

    // DEC: Decrement memory
    // Affects: N Z
    fn dec(&mut self) -> bool {
        self.fetch();

        let sub = self.fetched as u16 - 1;
        self.write(self.eff_addr, (sub & 0x00FF) as u8);

        self.flags.set(Flags::N, sub & 0x0080 != 0);
        self.flags.set(Flags::Z, sub & 0x00FF == 0);

        false
    }

    // EOR: Bitwise exclusive OR
    // Affects: N Z
    fn eor(&mut self) -> bool {
        self.fetch();

        let xor = self.acc ^ self.fetched;

        self.flags.set(Flags::N, xor & 0x80 != 0);
        self.flags.set(Flags::Z, xor == 0);

        true
    }

    // CLC: Clear carry
    fn clc(&mut self) -> bool {
        self.flags.set(Flags::C, false);
        false
    }

    // SEC: Set carry
    fn sec(&mut self) -> bool {
        self.flags.set(Flags::C, true);
        false
    }

    // CLI: Clear interrupt
    fn cli(&mut self) -> bool {
        self.flags.set(Flags::I, false);
        false
    }

    // SEI: Set interrupt
    fn sei(&mut self) -> bool {
        self.flags.set(Flags::I, true);
        false
    }

    // CLV: Clear overflow
    fn clv(&mut self) -> bool {
        self.flags.set(Flags::V, false);
        false
    }

    // CLD: Clear decimal
    fn cld(&mut self) -> bool {
        self.flags.set(Flags::D, false);
        false
    }

    // SED: Set decimal
    fn sed(&mut self) -> bool {
        self.flags.set(Flags::D, true);
        false
    }

    // INC: Increment memory
    // Affects: N Z
    fn inc(&mut self) -> bool {
        self.fetch();

        let inc = self.fetched as u16 + 1;
        self.write(self.eff_addr, (inc & 0x00FF) as u8);

        self.flags.set(Flags::N, inc & 0x0080 != 0);
        self.flags.set(Flags::Z, inc & 0x00FF == 0);

        false
    }

    // JMP: Jump
    // Affects: None
    fn jmp(&mut self) -> bool {
        self.pc = self.eff_addr;
        false
    }

    // JSR: Jump to subroutine
    // Affects: None
    fn jsr(&mut self) -> bool {
        // NOTE: we assume that sp points to the topmost empty space
        let ret_addr = self.pc - 1;

        self.push(((ret_addr >> 8) & 0xFF) as u8);
        self.push((ret_addr & 0xFF) as u8);

        self.pc = self.eff_addr;

        false
    }

    // LDA: Load accumulator
    // Affects: N Z
    fn lda(&mut self) -> bool {
        self.fetch();

        self.acc = self.fetched;

        self.flags.set(Flags::N, self.acc & 0x80 != 0);
        self.flags.set(Flags::Z, self.acc == 0);

        true
    }

    // LDX: Load X register
    // Affects: N Z
    fn ldx(&mut self) -> bool {
        self.fetch();

        self.x = self.fetched;

        self.flags.set(Flags::N, self.x & 0x80 != 0);
        self.flags.set(Flags::Z, self.x == 0);

        true
    }

    // LDY: Load Y register
    // Affects: N Z
    fn ldy(&mut self) -> bool {
        self.fetch();

        self.y = self.fetched;

        self.flags.set(Flags::N, self.y & 0x80 != 0);
        self.flags.set(Flags::Z, self.y == 0);

        true
    }

    // LSR: Logical shift right
    // Affects: N Z C
    // 0 -> [76543210] -> C
    fn lsr(&mut self) -> bool {
        self.fetch();

        let carry = self.fetched & 0x1;
        let shifted = self.fetched >> 1;

        self.flags.set(Flags::N, (shifted & 0x80) != 0);
        self.flags.set(Flags::Z, shifted == 0);
        self.flags.set(Flags::C, carry != 0);

        let opcode = self.opcode as usize;
        if INSTRUCTIONS[opcode].mode == AddrMode::Imp {
            self.acc = shifted & 0x00FF;
        } else {
            self.write(self.eff_addr, shifted);
        }

        false
    }

    fn nop(&mut self) -> bool {
        match self.opcode {
            | 0x1C
            | 0x3C
            | 0x5C
            | 0x7C
            | 0xDC
            | 0xFC => true,
            _ => false,
        }
    }

    fn xxx(&mut self) -> bool {
        false
    }

    // ORA: Bitwise OR with accumulator
    // Affects: N Z
    fn ora(&mut self) -> bool {
        self.fetch();

        self.acc = self.acc | self.fetched;
        self.flags.set(Flags::N, (self.acc & 0x80) != 0);
        self.flags.set(Flags::Z, self.acc == 0);

        true
    }

    // TAX: Transfer A to X
    // Affects: N Z
    fn tax(&mut self) -> bool {
        self.x = self.acc;

        self.flags.set(Flags::N, self.x == 0);
        self.flags.set(Flags::Z, (self.x & 0x80) != 0);

        false
    }

    // TXA: Transfer X to A
    // Affects: N Z
    fn txa(&mut self) -> bool {
        self.acc = self.x;

        self.flags.set(Flags::N, self.acc == 0);
        self.flags.set(Flags::Z, (self.acc & 0x80) != 0);

        false
    }

    // DEX: Decrement X
    // Affects: N Z
    fn dex(&mut self) -> bool {
        self.x -= 1;

        self.flags.set(Flags::N, self.x == 0);
        self.flags.set(Flags::Z, (self.x & 0x80) != 0);

        false
    }

    // INX: Increment X
    // Affects: N Z
    fn inx(&mut self) -> bool {
        self.x += 1;

        self.flags.set(Flags::N, self.x == 0);
        self.flags.set(Flags::Z, (self.x & 0x80) != 0);

        false
    }

    // TAY: Transfer A to Y
    // Affects: N Z
    fn tay(&mut self) -> bool {
        self.y = self.acc;

        self.flags.set(Flags::N, self.y == 0);
        self.flags.set(Flags::Z, (self.y & 0x80) != 0);

        false
    }

    // TAY: Transfer Y to A
    // Affects: N Z
    fn tya(&mut self) -> bool {
        self.acc = self.y;

        self.flags.set(Flags::N, self.acc == 0);
        self.flags.set(Flags::Z, (self.acc & 0x80) != 0);

        false
    }

    // DEY: Decrement Y
    // Affects: N Z
    fn dey(&mut self) -> bool {
        self.y -= 1;

        self.flags.set(Flags::N, self.y == 0);
        self.flags.set(Flags::Z, (self.y & 0x80) != 0);

        false
    }

    // INY: Increment Y
    // Affects: N Z
    fn iny(&mut self) -> bool {
        self.y += 1;

        self.flags.set(Flags::N, self.y == 0);
        self.flags.set(Flags::Z, (self.y & 0x80) != 0);

        false
    }

    // ROL: Rotate left
    // Affects: N Z C
    fn rol(&mut self) -> bool {
        self.fetch();

        let shifted = self.fetched << 1 | self.flags.carry as u8;

        self.flags.set(Flags::N, (shifted & 0x80) != 0);
        self.flags.set(Flags::Z, shifted == 0);
        self.flags.set(Flags::C, (self.fetched & 0x80) != 0);

        let opcode = self.opcode as usize;
        if INSTRUCTIONS[opcode].mode == AddrMode::Imp {
            self.acc = shifted & 0x00FF;
        } else {
            self.write(self.eff_addr, shifted & 0x00FF);
        }

        false
    }

    // ROR: Rotate right
    // Affects: N Z C
    fn ror(&mut self) -> bool {
        self.fetch();

        let shifted = self.fetched >> 1 | (self.flags.carry as u8) << 7;

        self.flags.set(Flags::N, (shifted & 0x80) != 0);
        self.flags.set(Flags::Z, shifted == 0);
        self.flags.set(Flags::C, (self.fetched & 0x1) != 0);

        let opcode = self.opcode as usize;
        if INSTRUCTIONS[opcode].mode == AddrMode::Imp {
            self.acc = shifted & 0x00FF;
        } else {
            self.write(self.eff_addr, shifted & 0x00FF);
        }

        false
    }

    fn rti(&mut self) -> bool {
        todo!()
    }

    // RTS: Return from subroutine
    // Affects: None
    fn rts(&mut self) -> bool {
        let lo: u16 = self.pull().into();
        let hi: u16 = self.pull().into();

        self.pc = (hi << 8 | lo) + 1;

        false
    }

    // SBC: Subtract with carry
    // Affects: N V Z C
    fn sbc(&mut self) -> bool {
        //    A - M - ~C
        // -> A - M - (1 - C)
        // -> A - M - 1 + C
        // -> A + ~M + C
        // everything's the same as ADC, but only the fetched data is inversed

        self.fetch();

        // invert the lower byte
        // NOTE: upper byte should be 0x00, but carry
        // out might change it
        let fetched: u16 =(self.fetched as u16) ^ 0x00FF;
        let acc: u16 = self.acc.into();

        let mut sub: u16 = acc + fetched;
        sub += if self.flags.carry { 1 } else { 0 };

        self.flags.set(Flags::N, sub & 0x80 != 0);

        // http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        self.flags.set(Flags::V, (!(acc ^ fetched) & (acc ^ sub) & 0x80) != 0);
        self.flags.set(Flags::Z, sub == 0);
        self.flags.set(Flags::C, sub > 0xFF);

        self.acc = (sub & 0x00FF) as u8;

        true
    }

    // STA: Store accumulator in memory
    // Affects: None
    fn sta(&mut self) -> bool {
        self.write(self.eff_addr, self.acc);
        false
    }

    // TXS: Transfer X to stack pointer
    // Affects: None
    fn txs(&mut self) -> bool {
        self.sp = self.x;
        false
    }

    // TSX: Transfer stack pointer to X
    // Affects: None
    fn tsx(&mut self) -> bool {
        self.x = self.sp;
        false
    }

    // PHA: Push accumulator
    // Affects: None
    fn pha(&mut self) -> bool {
        self.push(self.acc);
        false
    }

    // PLA: Pull accumulator
    // Affects: N Z
    fn pla(&mut self) -> bool {
        self.acc = self.pull();

        self.flags.set(Flags::N, (self.acc & 0x80) != 0);
        self.flags.set(Flags::Z, self.acc == 0);

        false
    }

    // PHP: Push processor status
    // Affects: None
    fn php(&mut self) -> bool {
        let flags = CpuFlags { brk: true, ignored: true, ..self.flags};
        self.push(flags.to_byte());
        false
    }

    // PLP: Pull processor status
    // Affects: None
    fn plp(&mut self) -> bool {
        let mut flags = CpuFlags::from_byte(self.pull());
        // The break flag will be masked and cleared (0), whenever transferred from
        // the stack to the status register, either by PLP or during a return from
        // interrupt (RTI).
        // https://www.masswerk.at/6502/6502_instruction_set.html
        flags.set(Flags::B, false);
        flags.set(Flags::U, false); // just in case
        self.flags = flags;
        false
    }

    // STX: Store index X in memory
    // Affects: None
    fn stx(&mut self) -> bool {
        self.write(self.eff_addr, self.x);
        false
    }

    // STY: Store index Y in memory
    // Affects: None
    fn sty(&mut self) -> bool {
        self.write(self.eff_addr, self.y);
        false
    }
}
