use crate::inst::INSTRUCTIONS;
use crate::addr::Addr6502;

pub enum Flags {
    N,
    V,
    U,
    B,
    D,
    I,
    Z,
    C,
}

#[derive(Copy,Clone)]
pub struct CpuFlags {
    pub negative: bool,
    pub overflow: bool,
    pub ignored: bool,

    // NOTE: The break flag is not an actual flag implemented
    // in a register, and rather appears only when the status
    // register is pushed onto or pulled from the stack.
    // When pushed, it will be 1 when transferred by a BRK or PHP,
    // and zero otherwise (i.e. when pushed by a H/W interuupt)
    //
    // https://www.masswerk.at/6502/6502_instruction_set.html
    pub brk: bool,
    pub decimal: bool,
    pub interrupt: bool,
    pub zero: bool,
    pub carry: bool,
}
impl CpuFlags {
    pub fn from_byte(byte: u8) -> Self {
        let is_set = |n: u8| (byte >> n) & 0x1 != 0;

        CpuFlags {
            negative: is_set(7),
            overflow: is_set(6),
            ignored: is_set(5),
            brk: is_set(4),
            decimal: is_set(3),
            interrupt: is_set(2),
            zero: is_set(1),
            carry: is_set(0),
        }
    }

    pub fn empty() -> Self {
        CpuFlags {
            negative: false,
            overflow: false,
            ignored: false,
            brk: false,
            decimal: false,
            interrupt: false,
            zero: false,
            carry: false,
        }
    }

    pub fn set(&mut self, flag: Flags, value: bool) {
        match flag {
            Flags::N => self.negative = value,
            Flags::V => self.overflow = value,
            Flags::U => self.ignored = value,
            Flags::B => self.brk = value,
            Flags::D => self.decimal = value,
            Flags::I => self.interrupt = value,
            Flags::Z => self.zero = value,
            Flags::C => self.carry = value,
        };
    }

    pub fn to_byte(&self) -> u8 {
        let to_bit = |x: bool, n: u8| (if x { 1 } else { 0 }) << n;

        to_bit(self.negative , 7) |
        to_bit(self.overflow , 6) |
        to_bit(self.ignored  , 5) |
        to_bit(self.brk      , 4) |
        to_bit(self.decimal  , 3) |
        to_bit(self.interrupt, 2) |
        to_bit(self.zero     , 1) |
        to_bit(self.carry    , 0)
    }

}

pub struct CPU {
    pub pc: u16,

    // NOTE: 0x0100 - 0x01FF is used for stack
    // sp holds the address of top of that space
    // stack grows downward in the NES
    pub sp: u8,
    pub acc: u8,
    pub x: u8,
    pub y: u8,
    pub flags: CpuFlags,

    // 2KB cpu RAM -> 0x800
    pub ram: [u8; 0x800],

    // fetched data
    pub fetched: u8,

    // effective address of fetched data
    pub eff_addr: u16,

    // jump offset in the REL addressing mode
    pub jump_offset: u16,

    pub opcode: u8,

    // how many cycles are left for the current instruction?
    // when this value reaches 0, then execute the next instruction
    pub cycles: u8,
}

impl CPU {
    /**
     * $0000-$07FF: 2KB internal RAM
     * $0800-$0FFF:
     * $1000-$17FF: Mirrors of $0000-07FF
     * $1800-$1FFF:
     * $2000-$2007: PPU Registers
     * $2008-$3FFF: Mirrors of $2000-$2007 (every 8 bytes)
     * $4000-$4017: NES APU and I/O Registers
     * $4018-$401F: APU and I/O functionality
     * $4020-$FFFF: Cartridge space
     *
     * References:
     * https://wiki.nesdev.org/w/index.php/CPU_memory_map
     * https://retrocomputing.stackexchange.com/questions/21793/what-is-the-purpose-of-mirrored-memory-regions-in-ness-cpu-memory-map
     */
    pub fn read(&mut self, addr: u16) -> u8 {
        if addr < 0x2000 {
            let masked_addr = (addr & 0x07FF) as usize;
            return self.ram[masked_addr];
        }

        todo!("Reads to other areas of memory map must be implemented!");
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr < 0x2000 {
            let masked_addr = (addr & 0x07FF) as usize;
            self.ram[masked_addr] = value;
        }

        todo!("Writes to other areas of memory map must be implemented!");
    }

    // push a value to the stack
    pub fn push(&mut self, value: u8) {
        let sp = self.sp as u16;
        self.write(0x100 + sp, value);

        // stack grows downward
        self.sp -= 1;
    }

    // pull a value from the stack
    pub fn pull(&mut self) -> u8 {
        let sp = self.sp as u16;
        self.sp += 1;
        self.read(0x100 + sp)
    }

    fn execute(&mut self) {
        let opcode = self.opcode as usize;
        assert!((opcode as usize) < INSTRUCTIONS.len());
        let operation = &INSTRUCTIONS[opcode];

    }
}
