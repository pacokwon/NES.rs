use crate::addr::{Addr6502, AddrMode};
use crate::cpu::CPU;

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

    // bitwise AND with accumulator
    fn adc(&mut self);

    // arithmetic shift left
    fn asl(&mut self);

    // test bits
    fn bit(&mut self);

    // branch instructions
    fn bpl(&mut self);
    fn bmi(&mut self);
    fn bvc(&mut self);
    fn bvs(&mut self);
    fn bcc(&mut self);
    fn bcs(&mut self);
    fn bne(&mut self);
    fn beq(&mut self);

    // break
    fn brk(&mut self);

    // comparse accumulator
    fn cmp(&mut self);

    // comparse x register
    fn cpx(&mut self);

    // comparse y register
    fn cpy(&mut self);

    // decrement memory
    fn dec(&mut self);

    // bitwise xor
    fn eor(&mut self);

    // flag instructions
    fn clc(&mut self);
    fn sec(&mut self);
    fn cli(&mut self);
    fn sei(&mut self);
    fn clv(&mut self);
    fn cld(&mut self);
    fn sed(&mut self);

    // increment memory
    fn inc(&mut self);

    // jump
    fn jmp(&mut self);

    // jump to subroutine
    fn jsr(&mut self);

    // load accumulator
    fn lda(&mut self);

    // load y register
    fn ldx(&mut self);

    // load x register
    fn ldy(&mut self);

    // logical shift right
    fn lsr(&mut self);

    // no-op
    fn nop(&mut self);
    fn xxx(&mut self);

    // bistwise OR with accumulator
    fn ora(&mut self);

    // register instructions
    fn tax(&mut self);
    fn txa(&mut self);
    fn dex(&mut self);
    fn inx(&mut self);
    fn tay(&mut self);
    fn tya(&mut self);
    fn dey(&mut self);
    fn iny(&mut self);

    // rotate left
    fn rol(&mut self);

    // rotate right
    fn ror(&mut self);

    // return from interrupt
    fn rti(&mut self);

    // return from subroutine
    fn rts(&mut self);

    // subtract with carry
    fn sbc(&mut self);

    // store accumulator
    fn sta(&mut self);

    // stack instructions
    fn txs(&mut self);
    fn tsx(&mut self);
    fn pha(&mut self);
    fn pla(&mut self);
    fn php(&mut self);
    fn plp(&mut self);

    // store x register
    fn stx(&mut self);

    // store y register
    fn sty(&mut self);
}

impl Inst6502 for CPU {
    fn execute_op(&mut self) {
        todo!()
    }

    fn adc(&mut self) {
        todo!()
    }

    fn asl(&mut self) {
        todo!()
    }

    fn bit(&mut self) {
        todo!()
    }

    fn bpl(&mut self) {
        todo!()
    }

    fn bmi(&mut self) {
        todo!()
    }

    fn bvc(&mut self) {
        todo!()
    }

    fn bvs(&mut self) {
        todo!()
    }

    fn bcc(&mut self) {
        todo!()
    }

    fn bcs(&mut self) {
        todo!()
    }

    fn bne(&mut self) {
        todo!()
    }

    fn beq(&mut self) {
        todo!()
    }

    fn brk(&mut self) {
        todo!()
    }

    fn cmp(&mut self) {
        todo!()
    }

    fn cpx(&mut self) {
        todo!()
    }

    fn cpy(&mut self) {
        todo!()
    }

    fn dec(&mut self) {
        todo!()
    }

    fn eor(&mut self) {
        todo!()
    }

    fn clc(&mut self) {
        todo!()
    }

    fn sec(&mut self) {
        todo!()
    }

    fn cli(&mut self) {
        todo!()
    }

    fn sei(&mut self) {
        todo!()
    }

    fn clv(&mut self) {
        todo!()
    }

    fn cld(&mut self) {
        todo!()
    }

    fn sed(&mut self) {
        todo!()
    }

    fn inc(&mut self) {
        todo!()
    }

    fn jmp(&mut self) {
        todo!()
    }

    fn jsr(&mut self) {
        todo!()
    }

    fn lda(&mut self) {
        todo!()
    }

    fn ldx(&mut self) {
        todo!()
    }

    fn ldy(&mut self) {
        todo!()
    }

    fn lsr(&mut self) {
        todo!()
    }

    fn nop(&mut self) {
        todo!()
    }

    fn xxx(&mut self) {
        todo!()
    }

    fn ora(&mut self) {
        todo!()
    }

    fn tax(&mut self) {
        todo!()
    }

    fn txa(&mut self) {
        todo!()
    }

    fn dex(&mut self) {
        todo!()
    }

    fn inx(&mut self) {
        todo!()
    }

    fn tay(&mut self) {
        todo!()
    }

    fn tya(&mut self) {
        todo!()
    }

    fn dey(&mut self) {
        todo!()
    }

    fn iny(&mut self) {
        todo!()
    }

    fn rol(&mut self) {
        todo!()
    }

    fn ror(&mut self) {
        todo!()
    }

    fn rti(&mut self) {
        todo!()
    }

    fn rts(&mut self) {
        todo!()
    }

    fn sbc(&mut self) {
        todo!()
    }

    fn sta(&mut self) {
        todo!()
    }

    fn txs(&mut self) {
        todo!()
    }

    fn tsx(&mut self) {
        todo!()
    }

    fn pha(&mut self) {
        todo!()
    }

    fn pla(&mut self) {
        todo!()
    }

    fn php(&mut self) {
        todo!()
    }

    fn plp(&mut self) {
        todo!()
    }

    fn stx(&mut self) {
        todo!()
    }

    fn sty(&mut self) {
        todo!()
    }
}
