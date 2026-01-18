#![allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
pub enum RegType {
    #[default]
    Val,
    Ptr,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Register64 {
    /// Type of register.
    pub reg_type: RegType,
    /// 64-bit value.
    pub u64: u64,
    /// low 32 bit.
    pub low32: u32,
    /// high 32 bit.
    pub high32: u32,
    /// low 16 bit.
    pub low16: u16,
    /// Mid-low 16 bit.
    pub midlow16: u16,
    /// Mid-high 16 bit.
    pub midhigh16: u16,
    /// High 16 bit.
    pub high16: u16,
    pub b: [u8; 8],
}

/// CPU flags
#[derive(Clone, Copy)]
pub struct Flags {
    /// Zero.
    pub zero: i32,
    /// Negative.
    pub sign: i32,
    /// Greater.
    pub greater: i32,
}

/// Main struct for interpreter.
pub struct Interpreter {
    /// 64-bit register.
    pub registers: [Register64; 20],
    /// Memory
    ///
    /// Memory 
    pub memory: Vec<u8>,
    /// Z memory
    ///
    /// Z memory size 300.
    pub zmemory: Vec<u8>,
    /// Program counter.
    pub pc: u64,
    ///CPU flags.
    pub flags: Flags,
}
pub trait RegisterRead {
    fn read(reg: &Register64) -> Self;
}
impl Interpreter {
    /// Reads the value of a register by index.
    pub fn read_reg(&self, idx: usize) -> u64 {
        self.registers[idx].u64
    }

    /// Writes a value to a register by index.
    pub fn write_reg(&mut self, idx: usize, value: u64) {
        self.registers[idx].u64 = value;
    }

    /// Runs the easy64 interpreter loop.
    pub fn interpret(&mut self) {
        todo!()
    }
}
