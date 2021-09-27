use crate::inst::{Inst6502,INSTRUCTIONS};
use crate::addr::Addr6502;

struct CpuFlags;
impl CpuFlags {
    const N: u8 = 1 << 7;
    const V: u8 = 1 << 6;
    const B: u8 = 1 << 4;
    const D: u8 = 1 << 3;
    const I: u8 = 1 << 2;
    const Z: u8 = 1 << 1;
    const C: u8 = 1 << 0;
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
    pub flags: u8,

    // 2KB cpu RAM -> 0x800
    pub ram: [u8; 0x800],

    // fetched data
    pub fetched: u8,

    // effective address of fetched data
    pub eff_addr: u16,

    pub opcode: u8,
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

    fn execute(&mut self) {
        let opcode = self.opcode as usize;
        assert!((opcode as usize) < INSTRUCTIONS.len());
        let operation = &INSTRUCTIONS[opcode];

    }
}
