use crate::StatusRegisterValues;

pub struct Registers6502 {
    pub pc: u16, //program counter
    pub ac: u8, //accumulator
    pub x: u8, //x register
    pub y: u8, // y register 
    pub sr: StatusRegisterValues, //status register
    pub sp: u8 //stack pointer
}

impl Registers6502 {
    pub fn new() -> Self {
        Registers6502 {
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            sr: StatusRegisterValues::new(),
            sp: 0,
        }
    }
}