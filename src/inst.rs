use crate::addr::AddrMode;

pub struct Inst {
    mnemonic: &'static str,
    mode: AddrMode,
    length: u8,
    cycles: u8,
}

// illegal / unofficial instructions are not complete
pub static INSTRUCTIONS: [Inst; 0x100] = [
    Inst { mnemonic: "BRK", mode: AddrMode::Imp,        length: 1, cycles: 7 },
    Inst { mnemonic: "ORA", mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 3 },
    Inst { mnemonic: "ORA", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "ASL", mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: "PHP", mode: AddrMode::Imp,        length: 1, cycles: 3 },
    Inst { mnemonic: "ORA", mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: "ASL", mode: AddrMode::Acc,        length: 1, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "ORA", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "ASL", mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },

    Inst { mnemonic: "BPL", mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: "ORA", mode: AddrMode::IndYPlus,   length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "ORA", mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: "ASL", mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: "CLC", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "ORA", mode: AddrMode::AbsYPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "ORA", mode: AddrMode::AbsXPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "ASL", mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: "JSR", mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: "AND", mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "BIT", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "AND", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "ROL", mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: "PLP", mode: AddrMode::Imp,        length: 1, cycles: 4 },
    Inst { mnemonic: "AND", mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: "ROL", mode: AddrMode::Acc,        length: 1, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "BIT", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "AND", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "ROL", mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },

    Inst { mnemonic: "BMI", mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: "AND", mode: AddrMode::IndYPlus,   length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "AND", mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: "ROL", mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: "SEC", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "AND", mode: AddrMode::AbsYPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "AND", mode: AddrMode::AbsXPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "ROL", mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: "RTI", mode: AddrMode::Imp,        length: 1, cycles: 6 },
    Inst { mnemonic: "EOR", mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 3 },
    Inst { mnemonic: "EOR", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "LSR", mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: "PHA", mode: AddrMode::Imp,        length: 1, cycles: 3 },
    Inst { mnemonic: "EOR", mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: "LSR", mode: AddrMode::Acc,        length: 1, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "JMP", mode: AddrMode::Abs,        length: 3, cycles: 3 },
    Inst { mnemonic: "EOR", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "LSR", mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },

    Inst { mnemonic: "BVC", mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: "EOR", mode: AddrMode::IndYPlus,   length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "EOR", mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: "LSR", mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: "CLI", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "EOR", mode: AddrMode::AbsYPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "EOR", mode: AddrMode::AbsXPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "LSR", mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: "RTS", mode: AddrMode::Imp,        length: 1, cycles: 6 },
    Inst { mnemonic: "ADC", mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 3 },
    Inst { mnemonic: "ADC", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "ROR", mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: "PLA", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "ADC", mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: "ROR", mode: AddrMode::Acc,        length: 1, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "JMP", mode: AddrMode::Ind,        length: 3, cycles: 5 },
    Inst { mnemonic: "ADC", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "ROR", mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },

    Inst { mnemonic: "BVS", mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: "ADC", mode: AddrMode::IndYPlus,   length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "ADC", mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: "ROR", mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: "SEI", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "ADC", mode: AddrMode::AbsYPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "ADC", mode: AddrMode::AbsXPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "ROR", mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "STA", mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: "STY", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "STA", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "STX", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 3 },
    Inst { mnemonic: "DEY", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "TXA", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "STY", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "STA", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "STX", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 4 },

    Inst { mnemonic: "BCC", mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: "STA", mode: AddrMode::IndY,       length: 2, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: "STY", mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: "STA", mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: "STX", mode: AddrMode::ZeroY,      length: 2, cycles: 4 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "TYA", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "STA", mode: AddrMode::AbsY,       length: 3, cycles: 4 },
    Inst { mnemonic: "TXS", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: "STA", mode: AddrMode::AbsX,       length: 3, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 5 },

    Inst { mnemonic: "LDY", mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: "LDA", mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: "LDX", mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: "LDY", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "LDA", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "LDX", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 3 },
    Inst { mnemonic: "TAY", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "LDA", mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: "TAX", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "LDY", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "LDA", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "LDX", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 3, cycles: 4 },

    Inst { mnemonic: "BCS", mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: "LDA", mode: AddrMode::IndYPlus,   length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: "LDY", mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: "LDA", mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: "LDX", mode: AddrMode::ZeroY,      length: 2, cycles: 4 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: "CLV", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "LDA", mode: AddrMode::AbsYPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "TSX", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "LDY", mode: AddrMode::AbsXPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "LDA", mode: AddrMode::AbsXPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "LDX", mode: AddrMode::AbsYPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 4 },

    Inst { mnemonic: "CPY", mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: "CMP", mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "CPY", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "CMP", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "DEC", mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: "INY", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "CMP", mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: "DEX", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "CPY", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "CMP", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "DEC", mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: "BNE", mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: "CMP", mode: AddrMode::IndYPlus,   length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "CMP", mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: "DEC", mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: "CLD", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "CMP", mode: AddrMode::AbsYPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "CMP", mode: AddrMode::AbsXPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "DEC", mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },

    Inst { mnemonic: "CPX", mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: "SBC", mode: AddrMode::IndX,       length: 2, cycles: 6 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "CPX", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "SBC", mode: AddrMode::Zero,       length: 2, cycles: 3 },
    Inst { mnemonic: "INC", mode: AddrMode::Zero,       length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 5 },
    Inst { mnemonic: "INX", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "SBC", mode: AddrMode::Imm,        length: 2, cycles: 2 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "CPX", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "SBC", mode: AddrMode::Abs,        length: 3, cycles: 4 },
    Inst { mnemonic: "INC", mode: AddrMode::Abs,        length: 3, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },

    Inst { mnemonic: "BEQ", mode: AddrMode::Rel,        length: 2, cycles: 2 },
    Inst { mnemonic: "SBC", mode: AddrMode::IndYPlus,   length: 2, cycles: 5 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 8 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "SBC", mode: AddrMode::ZeroX,      length: 2, cycles: 4 },
    Inst { mnemonic: "INC", mode: AddrMode::ZeroX,      length: 2, cycles: 6 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 6 },
    Inst { mnemonic: "SED", mode: AddrMode::Imp,        length: 1, cycles: 2 },
    Inst { mnemonic: "SBC", mode: AddrMode::AbsYPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 2 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },
    Inst { mnemonic: "NOP", mode: AddrMode::Imp,        length: 2, cycles: 4 },
    Inst { mnemonic: "SBC", mode: AddrMode::AbsXPlus,   length: 3, cycles: 4 },
    Inst { mnemonic: "INC", mode: AddrMode::AbsX,       length: 3, cycles: 7 },
    Inst { mnemonic: "???", mode: AddrMode::Imp,        length: 2, cycles: 7 },
];

// http://www.6502.org/tutorials/6502opcodes.html
pub trait Inst6502 {
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
