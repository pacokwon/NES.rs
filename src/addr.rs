use crate::cpu::CPU;
use crate::inst::INSTRUCTIONS;

#[derive(Debug,PartialEq,Eq)]
pub enum AddrMode {
    Imp,    // Implied
    Imm,    // Immediate
    Zero,
    ZeroX,
    ZeroY,
    Abs,
    AbsX,   // *
    AbsY,   // *
    Ind,
    IndX,
    IndY,   // *
    Rel,    // Relative
}

// NOTE: use this trait to read data!
pub trait Addr6502 {
    // assumes that one of the functions below is called
    // just before this function is called.
    // i.e. the `eff_addr` field must be valid upon calling this function
    fn fetch(&mut self);

    // the return value is for evaluating whether
    // or not an additional cycle will be needed.
    // refer to: http://www.6502.org/tutorials/6502opcodes.html
    fn imp(&mut self) -> bool;
    fn imm(&mut self) -> bool;
    fn zero(&mut self) -> bool;
    fn zero_x(&mut self) -> bool;
    fn zero_y(&mut self) -> bool;
    fn abs(&mut self) -> bool;
    fn abs_x(&mut self) -> bool;
    fn abs_y(&mut self) -> bool;
    fn ind(&mut self) -> bool;
    fn ind_x(&mut self) -> bool;
    fn ind_y(&mut self) -> bool;
    fn rel(&mut self) -> bool;
}

impl Addr6502 for CPU {
    fn fetch(&mut self) {
        let opcode = self.opcode as usize;

        // the IMP addressing mode function fetches the accumulator
        self.fetched = if INSTRUCTIONS[opcode].mode == AddrMode::Imp {
            self.read(self.eff_addr)
        } else {
            self.acc
        };
    }

    // NOTE: we also use Imp for accumulator mode,
    //       but we take care of that in the `fetch` function above
    fn imp(&mut self) -> bool {
        return false;
    }

    fn imm(&mut self) -> bool {
        self.eff_addr = self.pc;
        self.pc += 1;
        return false;
    }

    fn zero(&mut self) -> bool {
        self.eff_addr = self.read(self.pc) as u16 & 0x00FF;
        self.pc += 1;
        return false;
    }

    fn zero_x(&mut self) -> bool {
        let addr: u16 = self.read(self.pc).into();
        self.eff_addr = (addr + self.x as u16) & 0x00FF;
        self.pc += 1;
        return false;
    }

    fn zero_y(&mut self) -> bool {
        let addr: u16 = self.read(self.pc).into();
        self.eff_addr = (addr + self.y as u16) & 0x00FF;
        self.pc += 1;
        return false;
    }

    fn abs(&mut self) -> bool {
        let lo: u16 = self.read(self.pc + 0).into();
        let hi: u16 = self.read(self.pc + 1).into();
        self.eff_addr = hi << 8 | lo;
        self.pc += 2;
        return false;
    }

    fn abs_x(&mut self) -> bool {
        let lo: u16 = self.read(self.pc + 0).into();
        let hi: u16 = self.read(self.pc + 1).into();
        self.eff_addr = (hi << 8 | lo) + self.x as u16;
        self.pc += 2;
        return (self.eff_addr & 0xFF00) != (hi << 8);
    }

    fn abs_y(&mut self) -> bool {
        let lo: u16 = self.read(self.pc + 0).into();
        let hi: u16 = self.read(self.pc + 1).into();
        self.eff_addr = (hi << 8 | lo) + self.y as u16;
        self.pc += 2;
        return (self.eff_addr & 0xFF00) != (hi << 8);
    }

    // ($C000)
    fn ind(&mut self) -> bool {
        let ind_lo: u16 = self.read(self.pc + 0).into();
        let ind_hi: u16 = self.read(self.pc + 1).into();
        let ind_hilo = ind_hi << 8 | ind_lo;
        self.pc += 2;

        // https://www.nesdev.com/6502bugs.txt
        // *An indirect JMP (xxFF) will fail because the MSB will be fetched from
        // address xx00 instead of page xx+1.
        let (lo, hi): (u16, u16) =
            if ind_lo == 0x00FF {
                (self.read(ind_hilo).into(), self.read(ind_hi << 8).into())
            } else {
                (self.read(ind_hilo).into(), self.read(ind_hilo + 1).into())
            };

        self.eff_addr = hi << 8 | lo;
        return false;
    }

    // ($C0, X)
    fn ind_x(&mut self) -> bool {
        let zero_offset: u16 = self.read(self.pc).into();
        self.pc += 1;

        let lo: u16 = self.read((zero_offset + self.x as u16 + 0) & 0x00FF).into();
        let hi: u16 = self.read((zero_offset + self.x as u16 + 1) & 0x00FF).into();

        self.eff_addr = hi << 8 | lo;
        return false;
    }

    // ($C0), Y
    fn ind_y(&mut self) -> bool {
        let zero_addr: u16 = self.read(self.pc).into();
        self.pc += 1;

        let ind_lo: u16 = self.read((zero_addr + 0) & 0x00FF).into();
        let ind_hi: u16 = self.read((zero_addr + 1) & 0x00FF).into();
        self.eff_addr = (ind_hi << 8 | ind_lo) + self.y as u16;

        return (self.eff_addr & 0xFF00) != ind_hi << 8;
    }

    fn rel(&mut self) -> bool {
        // the address is 16 bits, so make the offset 16 bits as well
        let mut offset: u16 = self.read(self.pc).into();
        self.pc += 1;

        // if sign bit is set, make the 16 bit offset negative
        if offset & 0x80 != 0 {
            offset = offset | 0xFF00;
        }

        self.eff_addr = self.pc.wrapping_add(offset);
        return false;
    }
}
