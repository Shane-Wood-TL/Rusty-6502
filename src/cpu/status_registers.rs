pub struct StatusRegisterValues {
    pub n: bool, //negative
    pub v: bool, //Overflow
    pub b: bool, //break
    pub d: bool, //decimal (use BCD for math)
    pub i: bool, //interrupt (IRQ disable)
    pub z: bool, //zero
    pub c: bool, //carry
}

impl StatusRegisterValues {
    pub fn new() -> Self {
        StatusRegisterValues {
            n: false,
            v: false,
            b: false,
            d: false,
            i: false,
            z: false,
            c: false,
        }
    }
    pub fn to_byte(&self) -> u8 {
        (self.n as u8) << 7 |
        (self.v as u8) << 6 |
        1 << 5 |
        (self.b as u8) << 4 |
        (self.d as u8) << 3 |
        (self.i as u8) << 2 |
        (self.z as u8) << 1 |
        (self.c as u8)
    }

    pub fn from_byte(&mut self, byte: u8) {
        self.n = (byte & 0x80) != 0;
        self.v = (byte & 0x40) != 0;
        self.b = (byte & 0x10) != 0;
        self.d = (byte & 0x08) != 0;
        self.i = (byte & 0x04) != 0;
        self.z = (byte & 0x02) != 0;
        self.c = (byte & 0x01) != 0;
    }
}