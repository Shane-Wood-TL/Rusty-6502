#[repr(u8)]
enum Opcodes{
    //LDA commands are implemented, and tested
    LdaImmediate = 0xA9,
    LdaZeropage  = 0xA5,
    LdaZeropageX = 0xB5,
    LdaAbsolute = 0xAD,
    LdaAbsoluteX = 0xBD,
    LdaAbsoluteY = 0xB9,
    LdaIndirectX = 0xA1,
    LdaIndirectY = 0xB1,
    
    AdcImmediate = 	0x69,
    AdcZeropage = 0x65,
    AdcZeropageX = 0x75,
    AdcAbsolute = 0x6D,
    AdcAbsoluteX = 0x7D,
    AdcAbsoluteY = 0x79,
    AdcIndirectX = 0x61,
    AdcIndirectY = 0x71,
    
    AndImmediate = 	0x29,
    AndZeropage = 0x25,
    AndZeropageX = 0x35,
    AndAbsolute = 0x2D,
    AndAbsoluteX = 0x3D,
    AndAbsoluteY = 0x39,
    AndIndirectX = 0x21,
    AndIndirectY = 0x31,
    
    AslAccumulator = 0x0A,
    AslZeropage = 0x06,
    AslZeropageX = 0x16,
    AslAbsolute = 0x0E,
    AslAbsoluteX = 0x1E,
    
    BccRelative = 0x90,
    
    BcsRelative = 0xB0,
    
    BeqRelative = 0xF0,
    
    BitZeropage = 0x24,
    BitAbsolute = 0x2C,
    
    BmiRelative = 0x30,
    
    BneRelative = 0xD0,
    
    BplRelative = 0x10,
    
    BRK = 0x00,
    
    BvcRelative = 0x50,
    
    BvsRelative = 0x70,
    
    Clc = 0x18,
    
    Cld = 0xD8,
    
    Cli = 0x58,
    
    Clv = 0xB8,
    
    CmpImmediate = 0xC9,
    CmpZeropage = 0xC5,
    CmpZeropageX = 0xD5,
    CmpAbsolute = 0xCD,
    CmpAbsoluteX = 0xDD,
    CmpAbsoluteY = 0xD9,
    CmpIndirectX = 0xC1,
    CmpIndirectY = 0xD1,
    
    CpxImmediate = 0xE0,
    CpxZeropage = 0xE4,
    CpxAbsolute = 0xEC,
    
    CpyImmediate = 0xC0,
    CpyZeropage = 0xC4,
    CpyAbsolute = 0xCC,
    
    DecZeropage = 0xC6,
    DecZeropageX = 0xD6,
    DecAbsolute = 0xCE,
    DecAbsoluteX = 0xDE,
    
    Dex = 0xCA,
    
    Dey = 0x88,
    
    EorImmediate = 0x49,
    EorZeropage = 0x45,
    EorZeropageX = 0x55,
    EorAbsolute = 0x4D,
    EorAbsoluteX = 0x5D,
    EorAbsoluteY = 0x59,
    EorIndirectX = 0x41,
    EorIndirectY = 0x51,
    
    IncZeropage = 0xE6,
    IncZeropageX = 0xF6,
    IncAbsolute = 0xEE,
    IncAbsoluteX = 0xFE,
    
    Inx = 0xE8,
    
    Iny = 0xC8,
    
    JmpAbsolute = 0x4C,
    JmpIndirect = 0x6C,
    
    JsrAbsolute = 0x20,
    
    //LDX commands are implemented, not tested
    LdxImmediate = 0xA2,
    LdxZeropage = 0xA6,
    LdxZeropageY = 0xB6,
    LdxAbsolute = 0xAE,
    LdxAbsoluteY = 0xBE,
    
    //LDY commands are implemented, not tested
    LdyImmediate = 0xA0,
    LdyZeropage = 0xA4,
    LdyZeropageX = 0xB4,
    LdyAbsolute = 0xAC,
    LdyAbsoluteX = 0xBC,
    
    LsrAccumulator = 0x4A,
    LsrZeropage = 0x46,
    LsrZeropageX = 0x56,
    LsrAbsolute = 0x4E,
    LsrAbsoluteX = 0x5E,
    
    Nop = 0xEA,
    
    OraImmediate = 0x09,
    OraZeropage = 0x05,
    OraZeropageX = 0x15,
    OraAbsolute = 0x0D,
    OraAbsoluteX = 0x1D,
    OraAbsoluteY = 0x19,
    OraIndirectX = 0x01,
    OraIndirectY = 0x11,
    
    Pha = 0x48,
    
    Php = 0x08,
    
    Pla = 0x68,
    
    Plp = 0x28,
    
    RolAccumulator = 0x2A,
    RolZeropage = 0x26,
    RolZeropageX = 0x36,
    RolAbsolute = 0x2E,
    RolAbsoluteX = 0x3E,
    
    RorAccumulator = 0x6A,
    RorZeropage = 0x66,
    RorZeropageX = 0x76,
    RorAbsolute = 0x6E,
    RorAbsoluteX = 0x7E,
    
    Rti = 0x40,
    
    Rts = 0x60,
    
    SbcImmediate = 0xE9,
    SbcZeropage = 0xE5,
    SbcZeropageX = 0xF5,
    SbcAbsolute = 0xED,
    SbcAbsoluteX = 0xFD,
    SbcAbsoluteY = 0xF9,
    SbcIndirectX = 0xE1,
    SbcIndirectY = 0xF1,
    
    Sec = 0x38,
    
    Sed = 0xF8,
    
    Sei = 0x78,
    
    StaZeropage = 0x85,
    StaZeropageX = 0x95,
    StaAbsolute = 0x8D,
    StaAbsoluteX = 0x9D,
    StaAbsoluteY = 0x99,
    StaIndirectX = 0x81,
    StaIndirectY = 0x91,
    
    StxZeropage = 0x86,
    StxZeropageY = 0x96,
    StxAbsolute = 0x8E,
    
    StyZeropage = 0x84,
    StyZeropageX = 0x94,
    StyAbsolute = 0x8C,
    
    Tax = 0xAA,
    
    Tay = 0xA8,
    
    Tsx = 0xBA,
    
    Txa = 0x8A,
    
    Txs = 0x9A,
    
    Tya = 0x98
}



const MAX_MEMORY_SIZE: usize = 1024 * 64;
pub struct Memory {
    data: [u8; MAX_MEMORY_SIZE as usize],
}

impl Memory{
    pub fn new() -> Self {
        Memory {
            data: [0; MAX_MEMORY_SIZE as usize],
        }
    }
    
    
    pub fn reset(&mut self){
        for i in 0..MAX_MEMORY_SIZE{
            self.data[i as usize] = 0;
        }
    }
    
    pub fn read_byte(&mut self, address: u32) -> u8{
        if address > MAX_MEMORY_SIZE as u32{
            return 0
        }else{
            self.data[address as usize]
        }
    }
    
    pub fn write_byte(&mut self, address: u32, new_data: u8){
        if address > MAX_MEMORY_SIZE as u32{
            return
        }else{
            self.data[address as usize] = new_data;
            return
        }
        
    }
    
    pub fn write_word(&mut self, address: u32, new_data: u16,  cycles: &mut u32)
	{
		self.data[address as usize]	= (new_data & 0xFF)as u8;
		self.data[(address  + 1) as usize]   = (new_data >> 8) as u8;
		*cycles -= 2;
	}
}





pub struct StatusRegisterValues {
    n: bool, //negative
    v: bool, //Overflow
    not_used: bool, //ignored
    b: bool, //break
    d: bool, //decimal (use BCD for math)
    i: bool, //interrupt (IRQ disable)
    z: bool, //zero
    c: bool, //carry
}

impl StatusRegisterValues {
    pub fn new() -> Self {
        StatusRegisterValues {
            n: false,
            v: false,
            not_used: false,
            b: false,
            d: false,
            i: false,
            z: false,
            c: false,
        }
    }
}

pub struct Registors6502 {
    pc: u16, //program counter
    ac: i8, //accumulator
    x: i8, //x register
    y: i8, // y register 
    sr: StatusRegisterValues, //status register
    sp: u16 //stack pointer
}

impl Registors6502 {
    pub fn new() -> Self {
        Registors6502 {
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            sr: StatusRegisterValues::new(),
            sp: 0,
        }
    }
}

pub struct Cpu6502{
    registors: Registors6502,
    memory: Memory,
    cycle_count: u32
} 

impl Cpu6502{
    pub fn new() -> Self {
        Cpu6502 {
            registors: Registors6502::new(),
            memory: Memory::new(),
            cycle_count: 0,
        }
    }
    
    fn read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.memory.read_byte(addr as u32) as u16;
        let hi = self.memory.read_byte((addr + 1) as u32) as u16;
        (hi << 8) | lo
    }
    
    pub fn reset(&mut self) {
        let reset_vector = self.read_u16(0xFFFC);
        self.init_registers(reset_vector);
    }
    
    pub fn init_registers(&mut self, reset_vector: u16){
        self.registors.pc = reset_vector;
        self.registors.sp = 0x0100;
        //self.registors.x = 0;
        //self.registors.y = 0;
        
        self.registors.sr = StatusRegisterValues::new();
        
        
        self.cycle_count = 0;
    }
    
    pub fn step(&mut self) -> i64{
        let pc = self.registors.pc;
        let opcode = self.memory.read_byte(pc as u32);
        
        self.registors.pc += 1; //move to next byte
        
        match opcode{
            x if x == Opcodes::LdaImmediate as u8 => self.lda_immediate(),
            x if x == Opcodes::LdaZeropage as u8 => self.lda_zero_page(),
            x if x == Opcodes::LdaZeropageX as u8 => self.lda_zero_page_x(),
            x if x == Opcodes::LdaAbsolute as u8 => self.lda_absolute(),
            x if x == Opcodes::LdaAbsoluteX as u8 => self.lda_absolute_x(),
            x if x == Opcodes::LdaAbsoluteY as u8 => self.lda_absolute_y(),
            x if x == Opcodes::LdaIndirectX  as u8 => self.lda_indirect_x(),
            x if x == Opcodes::LdaIndirectY  as u8 => self.lda_indirect_y(),
            x if x == Opcodes::LdxImmediate  as u8 => self.ldx_immediate(),
            x if x == Opcodes::LdxZeropage  as u8 => self.ldx_zero_page(),
            x if x == Opcodes::LdxZeropageY  as u8 => self.ldx_zero_page_y(),
            x if x == Opcodes::LdxAbsolute  as u8 => self.ldx_absolute(),
            x if x == Opcodes::LdxAbsoluteY  as u8 => self.ldx_absolute_y(),
            x if x == Opcodes::Nop  as u8 => self.nop(),
            x if x == Opcodes::Clc  as u8 => self.clc(),
            x if x == Opcodes::Cld  as u8 => self.cld(),
            x if x == Opcodes::Cli  as u8 => self.cli(),
            x if x == Opcodes::Clv  as u8 => self.clv(),
            x if x == Opcodes::BRK as u8 => {println!("BRK"); return -1;}
        _ =>{
            println!("opcode: {:02X}", opcode);
            return -1;
        }
        }
        return self.cycle_count as i64;
    }
    
    
    
    
    //LDA Commands
    fn lda_immediate(&mut self){
        let value = self.memory.read_byte(self.registors.pc as u32);
        println!("LDA I");
        self.registors.pc += 1; //LDA immediate takes 2 bytes,
        //one was already done in step
        
        self.registors.ac = value as i8;
        
        self.cycle_count += 2;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn lda_zero_page(&mut self){
        println!("lda_zero_page");
        let address = self.memory.read_byte(self.registors.pc as u32);
        self.cycle_count += 3;
        self.registors.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let value = self.memory.read_byte(address as u32);
        
        self.registors.ac = value as i8;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn lda_zero_page_x(&mut self){
        println!("lda_zero_page");
        let base_address = self.memory.read_byte(self.registors.pc as u32);
        self.cycle_count += 4;
        self.registors.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let address = base_address.wrapping_add(self.registors.x as u8);
        
        let value = self.memory.read_byte(address as u32);
        self.registors.ac = value as i8;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn lda_absolute(&mut self){
        println!("lda_absolute");
        let address = self.read_u16(self.registors.pc);
        
        self.registors.pc += 2; //LDA absolute takes 3 bytes, one was already done in step
        
        self.cycle_count += 4;
        
        let value = self.memory.read_byte(address as u32);
        
        self.registors.ac = value as i8;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn lda_absolute_x(&mut self){
        println!("lda_absolute_x");
        let address = self.read_u16(self.registors.pc);
        
        self.registors.pc += 2; //lda_absolute_x absolute takes 3 bytes, one was already done in step
        
        let x = self.registors.x as u16;
        let effective_address = address.wrapping_add(x);
        
        if (address & 0xFF00) != (effective_address & 0xFF00) {
            self.cycle_count += 5; // 5 if page boundry is crossed
        } else{
            self.cycle_count += 4;  //page boundry is not crossed
        }
        
        
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registors.ac = value as i8;
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn lda_absolute_y(&mut self){
        println!("lda_absolute_y");
        let address = self.read_u16(self.registors.pc);
        
        self.registors.pc += 2; //lda_absolute_x absolute takes 3 bytes, one was already done in step
        
        let y = self.registors.y as u16;
        let effective_address = address.wrapping_add(y);
        
        if (address & 0xFF00) != (effective_address & 0xFF00) {
            self.cycle_count += 5; // 5 if page boundry is crossed
        } else{
            self.cycle_count += 4;  //page boundry is not crossed
        }
        
        
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registors.ac = value as i8;
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn lda_indirect_x(&mut self){
        println!("lda_indirect_x");
        self.cycle_count += 6;
        
        let zero_page_operand = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 1;
        
        let pointer_address = zero_page_operand.wrapping_add(self.registors.x as u8);

        let effective_address = self.read_u16(pointer_address as u16);
        
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registors.ac = value as i8;
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn lda_indirect_y(&mut self){
        println!("lda_indirect_y");
        
        let zero_page_operand = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 1;
        
        let base_address = self.read_u16(zero_page_operand as u16);
        
        let effective_address = base_address.wrapping_add(self.registors.y as u16);

        println!("{}",effective_address);
        if (base_address & 0xFF00) != (effective_address & 0xFF00) {
            self.cycle_count += 6; // 6 if page boundry is crossed
        } else{
            self.cycle_count += 5;  //page boundry is not crossed
        }
        
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registors.ac = value as i8;
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    
    //LDX Commands
    fn ldx_immediate(&mut self){
        let value = self.memory.read_byte(self.registors.pc as u32);
        println!("LDX I");
        self.registors.pc += 1; //LDX immediate takes 2 bytes,
        //one was already done in step
        
        self.registors.x = value as i8;
        
        self.cycle_count += 2;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn ldx_zero_page(&mut self){
        println!("ldx_zero_page");
        let address = self.memory.read_byte(self.registors.pc as u32);
        self.cycle_count += 3;
        self.registors.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let value = self.memory.read_byte(address as u32);
        
        self.registors.x = value as i8;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn ldx_zero_page_y(&mut self){
        println!("ldx_zero_page_y");
        let base_address = self.memory.read_byte(self.registors.pc as u32);
        self.cycle_count += 4;
        self.registors.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let address = base_address.wrapping_add(self.registors.y as u8);
        
        let value = self.memory.read_byte(address as u32);
        self.registors.x = value as i8;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn ldx_absolute(&mut self){
        println!("ldx_absolute");
        let address = self.read_u16(self.registors.pc);
        
        self.registors.pc += 2; //LDX absolute takes 3 bytes, one was already done in step
        
        self.cycle_count += 4;
        
        let value = self.memory.read_byte(address as u32);
        
        self.registors.x = value as i8;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn ldx_absolute_y(&mut self){
        println!("ldx_indirect_y");
        
        let zero_page_operand = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 2;
        
        let base_address = self.read_u16(zero_page_operand as u16);
        
        let effective_address = base_address.wrapping_add(self.registors.y as u16);

        println!("{}",effective_address);
        if (base_address & 0xFF00) != (effective_address & 0xFF00) {
            self.cycle_count += 5; // 6 if page boundry is crossed
        } else{
            self.cycle_count += 4;  //page boundry is not crossed
        }
        
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registors.x = value as i8;
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    
    //LDY Commands
    fn ldy_immediate(&mut self){
        let value = self.memory.read_byte(self.registors.pc as u32);
        println!("LDY I");
        self.registors.pc += 1; //LDY immediate takes 2 bytes,
        //one was already done in step
        
        self.registors.y = value as i8;
        
        self.cycle_count += 2;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn ldy_zero_page(&mut self){
        println!("ldy_zero_page");
        let address = self.memory.read_byte(self.registors.pc as u32);
        self.cycle_count += 3;
        self.registors.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let value = self.memory.read_byte(address as u32);
        
        self.registors.y = value as i8;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn ldy_zero_page_x(&mut self){
        println!("ldy_zero_page_x");
        let base_address = self.memory.read_byte(self.registors.pc as u32);
        self.cycle_count += 4;
        self.registors.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let address = base_address.wrapping_add(self.registors.x as u8);
        
        let value = self.memory.read_byte(address as u32);
        self.registors.y = value as i8;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn ldy_absolute(&mut self){
        println!("ldy_absolute");
        let address = self.read_u16(self.registors.pc);
        
        self.registors.pc += 2; //LDy absolute takes 3 bytes, one was already done in step
        
        self.cycle_count += 4;
        
        let value = self.memory.read_byte(address as u32);
        
        self.registors.y = value as i8;
        
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn ldy_absolute_y(&mut self){
        println!("ldy_indirect_x");
        
        let zero_page_operand = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 2;
        
        let base_address = self.read_u16(zero_page_operand as u16);
        
        let effective_address = base_address.wrapping_add(self.registors.x as u16);

        println!("{}",effective_address);
        if (base_address & 0xFF00) != (effective_address & 0xFF00) {
            self.cycle_count += 5; // 6 if page boundry is crossed
        } else{
            self.cycle_count += 4;  //page boundry is not crossed
        }
        
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registors.y = value as i8;
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    
    //Signle Commands
    fn nop(&mut self){
        println!("nop");
        self.cycle_count += 2;
        //self.registors.pc += 0; //nop takes no operands
    }
    
    fn clc(&mut self){
        println!("clc");
        self.registors.sr.c = false;
        self.cycle_count += 2;
        //self.registors.pc += 0; //clc takes no operands
    }
    
    fn cld(&mut self){
        println!("cld");
        self.registors.sr.c = false;
        self.cycle_count += 2;
        //self.registors.pc += 0; //cld takes no operands
    }
    
    fn cli(&mut self){
        println!("cli");
        self.registors.sr.i = false;
        self.cycle_count += 2;
        //self.registors.pc += 0; //cli takes no operands
    }
    
    fn clv(&mut self){
        println!("clv");
        self.registors.sr.v = false;
        self.cycle_count += 2;
        //self.registors.pc += 0; //clv takes no operands
    }
}





#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_lda_immediate() {
        let mut cpu = Cpu6502::new();

        // Set reset vector to $8000
        cpu.memory.write_byte(0xFFFC, 0x00);
        cpu.memory.write_byte(0xFFFD, 0x80);

        // Program: LDA #$42; BRK
        cpu.memory.write_byte(0x8000, Opcodes::LdaImmediate as u8); // LDA Immediate
        cpu.memory.write_byte(0x8001, 0x42); // Value
        cpu.memory.write_byte(0x8002, Opcodes::BRK as u8); // BRK

        cpu.reset();
        
        let mut processor_running: bool = true;
        while processor_running == true{
            let cycles = cpu.step();
            println!("Cycles: {}", cycles);
            if cycles == -1 {
                processor_running = false
            }
        }
        assert_eq!(cpu.registors.ac, 0x42);
        assert!(!cpu.registors.sr.z); // not zero
        assert!(!cpu.registors.sr.n); // not negative
        assert!(cpu.cycle_count == 2);
    }
    
    #[test]
    fn test_lda_zeropage() {
        let mut cpu = Cpu6502::new();

        // Set reset vector to $8000
        cpu.memory.write_byte(0xFFFC, 0x00);
        cpu.memory.write_byte(0xFFFD, 0x80);
        
        let data_location : u8 = 0x42;
        let data_value : u8 = 0x32;
        //pub fn write_byte(&mut self, address: u32, new_data: u8){
        cpu.memory.write_byte(data_value as u32, data_location); //location, data
    
        cpu.memory.write_byte(0x8000, Opcodes::LdaZeropage as u8); //command
        cpu.memory.write_byte(0x8001, data_value); //add parameter in next location
        cpu.memory.write_byte(0x8002, Opcodes::BRK as u8); //break

        cpu.reset();
        
        let mut processor_running: bool = true;
        while processor_running == true{
            let cycles = cpu.step();
            println!("Cycles: {}", cycles);
            if cycles == -1 {
                processor_running = false
            }
        }
        assert_eq!(cpu.registors.ac, data_location.try_into().unwrap()); 
        assert!(!cpu.registors.sr.z); // not zero
        assert!(!cpu.registors.sr.n); // not negative
        assert!(cpu.cycle_count == 3);
    }
    
    #[test]
    fn test_lda_zeropage_x() {
        let mut cpu = Cpu6502::new();

        // Set reset vector to $8000
        cpu.memory.write_byte(0xFFFC, 0x00);
        cpu.memory.write_byte(0xFFFD, 0x80);
        
        cpu.registors.x = 0x01;

        let base_address: u8 = 0x32;
        let final_address: u8 = base_address.wrapping_add(cpu.registors.x as u8);
        let data_value: u8 = 0x42;
        
        cpu.memory.write_byte(final_address as u32, data_value);
        
        cpu.memory.write_byte(0x8000, Opcodes::LdaZeropageX as u8); //command
        cpu.memory.write_byte(0x8001, base_address); //add parameter in next location
        cpu.memory.write_byte(0x8002, Opcodes::BRK as u8); //break

        cpu.reset();
        
        let mut processor_running: bool = true;
        while processor_running == true{
            let cycles = cpu.step();
            println!("Cycles: {}", cycles);
            if cycles == -1 {
                processor_running = false
            }
        }
        assert_eq!(cpu.registors.ac, data_value.try_into().unwrap()); 
        assert!(!cpu.registors.sr.z); // not zero
        assert!(!cpu.registors.sr.n); // not negative
        assert!(cpu.cycle_count == 4);
    }
    
    #[test]
    fn test_lda_absolute() {
        let mut cpu = Cpu6502::new();

        // Set reset vector to $8000
        cpu.memory.write_byte(0xFFFC, 0x00);
        cpu.memory.write_byte(0xFFFD, 0x80);
        
        let data_address: u16 = 0x1234;
        let data_value: u8 = 0x42;
        
        
        //pub fn write_byte(&mut self, address: u32, new_data: u8){
        cpu.memory.write_byte(data_address as u32, data_value);
    
        cpu.memory.write_byte(0x8000, Opcodes::LdaAbsolute as u8); //command
        cpu.memory.write_byte(0x8001, (data_address & 0xFF) as u8);       // low byte
        cpu.memory.write_byte(0x8002, (data_address >> 8) as u8);         // high byte
        cpu.memory.write_byte(0x8003, Opcodes::BRK as u8); //break

        cpu.reset();
        
        let mut processor_running: bool = true;
        while processor_running == true{
            let cycles = cpu.step();
            println!("Cycles: {}", cycles);
            if cycles == -1 {
                processor_running = false
            }
        }
        assert_eq!(cpu.registors.ac, data_value.try_into().unwrap()); 
        assert!(!cpu.registors.sr.z); // not zero
        assert!(!cpu.registors.sr.n); // not negative
        assert!(cpu.cycle_count == 4);
        
     }   
    
    #[test]
    fn test_lda_absolute_x() {
        let mut cpu = Cpu6502::new();
    
        // Set reset vector to $8000
        cpu.memory.write_byte(0xFFFC, 0x00);
        cpu.memory.write_byte(0xFFFD, 0x80);
    
        let address: u16 = 0x12FF;
        let x: u8 = 0x01;
        let final_address = address.wrapping_add(x as u16);
        let data_value: u8 = 0x42;
    
        cpu.registors.x = x as i8;
    
        cpu.memory.write_byte(final_address as u32, data_value);
    
        cpu.memory.write_byte(0x8000, Opcodes::LdaAbsoluteX as u8);
        cpu.memory.write_byte(0x8001, (address & 0xFF) as u8); // low byte
        cpu.memory.write_byte(0x8002, (address >> 8) as u8);   // high byte
        cpu.memory.write_byte(0x8003, Opcodes::BRK as u8); // break
    
        cpu.reset();

        let mut processor_running: bool = true;
        while processor_running == true{
            let cycles = cpu.step();
            println!("Cycles: {}", cycles);
            if cycles == -1 {
                processor_running = false
            }
        }
        assert_eq!(cpu.registors.ac, data_value.try_into().unwrap()); 
        assert!(!cpu.registors.sr.z); // not zero
        assert!(!cpu.registors.sr.n); // not negative
        assert!(cpu.cycle_count == 5);
    }
    
    
    #[test]
    fn test_lda_absolute_y() {
        let mut cpu = Cpu6502::new();
    
        // Set reset vector to $8000
        cpu.memory.write_byte(0xFFFC, 0x00);
        cpu.memory.write_byte(0xFFFD, 0x80);
    

        let address: u16 = 0x12FF;
        let y: u8 = 0x01;
        let final_address = address.wrapping_add(y as u16);
        let data_value: u8 = 0x42;
    
        cpu.registors.y = y as i8; 
    
        cpu.memory.write_byte(final_address as u32, data_value);
    

        cpu.memory.write_byte(0x8000, Opcodes::LdaAbsoluteY as u8);
        cpu.memory.write_byte(0x8001, (address & 0xFF) as u8); // low byte
        cpu.memory.write_byte(0x8002, (address >> 8) as u8);   // high byte
        cpu.memory.write_byte(0x8003, Opcodes::BRK as u8); // break
    
        cpu.reset();

        
        let mut processor_running: bool = true;
        while processor_running == true{
            let cycles = cpu.step();
            println!("Cycles: {}", cycles);
            if cycles == -1 {
                processor_running = false
            }
        }
        assert_eq!(cpu.registors.ac, data_value.try_into().unwrap()); 
        assert!(!cpu.registors.sr.z); // not zero
        assert!(!cpu.registors.sr.n); // not negative
        assert!(cpu.cycle_count == 5);
    }
    
    #[test]
    fn test_lda_indirect_x() {
        let mut cpu = Cpu6502::new();
    
        // Reset vector to 0x8000
        cpu.memory.write_byte(0xFFFC, 0x00);
        cpu.memory.write_byte(0xFFFD, 0x80);
    
        // x register = 0x04
        cpu.registors.x = 0x04;
    
    
        // pointer at zero page 0x14
        let pointer_address: u16 = 0x1234;
        cpu.memory.write_byte(0x0014, (pointer_address & 0xFF) as u8);
        cpu.memory.write_byte(0x0015, (pointer_address >> 8) as u8);    
    
        // value at address
        let data_value: u8 = 0x42;
        cpu.memory.write_byte(pointer_address as u32, data_value);
    
        // Program at 0x8000:
        cpu.memory.write_byte(0x8000, Opcodes::LdaIndirectX as u8);
        cpu.memory.write_byte(0x8001, 0x10); 
        cpu.memory.write_byte(0x8002, Opcodes::BRK as u8); //break
    
        cpu.reset();
    
        let mut running = true;
        while running {
            let cycles = cpu.step();
            if cycles == -1 {
                running = false;
            }
        }
    
        assert_eq!(cpu.registors.ac, data_value as i8);
        assert!(!cpu.registors.sr.z);
        assert!(!cpu.registors.sr.n);
        assert_eq!(cpu.cycle_count, 6);
    }
    
    #[test]
    fn test_lda_indirect_y_no_page_cross() {
        let mut cpu = Cpu6502::new();
    
        // Reset vector to 0x8000
        cpu.memory.write_byte(0xFFFC, 0x00);
        cpu.memory.write_byte(0xFFFD, 0x80);
    
        // Set Y register to 0 (no offset)
        cpu.registors.y = 0x00;
    
        // Zero page pointer at 0x20 → points to 0x12FF
        let base_pointer_addr: u16 = 0x12FF;
        let data_value: u8 = 0x42;
    
        cpu.memory.write_byte(0x0020, (base_pointer_addr & 0xFF) as u8); // Low byte
        cpu.memory.write_byte(0x0021, (base_pointer_addr >> 8) as u8);   // High byte
        cpu.memory.write_byte(base_pointer_addr as u32, data_value);     // Store value
    
        // Program
        cpu.memory.write_byte(0x8000, Opcodes::LdaIndirectY as u8);
        cpu.memory.write_byte(0x8001, 0x20);
        cpu.memory.write_byte(0x8002, Opcodes::BRK as u8);
    
        cpu.reset();
    
        while cpu.step() != -1 {}
    
        assert_eq!(cpu.registors.ac, data_value as i8);
        assert!(!cpu.registors.sr.z);
        assert!(!cpu.registors.sr.n);
        assert_eq!(cpu.cycle_count, 5); // No page crossing
    }
    
    
    #[test]
    fn test_lda_indirect_y_with_page_cross() {
        let mut cpu = Cpu6502::new();
    
        // Reset vector to 0x8000
        cpu.memory.write_byte(0xFFFC, 0x00);
        cpu.memory.write_byte(0xFFFD, 0x80);

        cpu.registors.y = 0x01;
    
        let base_pointer_address: u16 = 0x12FF;
        let effective_address = base_pointer_address.wrapping_add(1); // 0x1300
        let data_value: u8 = 0x99;
    
        cpu.memory.write_byte(0x0020, (base_pointer_address & 0xFF) as u8); // Low byte
        cpu.memory.write_byte(0x0021, (base_pointer_address >> 8) as u8);   // High byte
        cpu.memory.write_byte(effective_address as u32, data_value); 
    
        // Program
        cpu.memory.write_byte(0x8000, Opcodes::LdaIndirectY as u8);
        cpu.memory.write_byte(0x8001, 0x20);
        cpu.memory.write_byte(0x8002, Opcodes::BRK as u8);
    
        cpu.reset();
    
        while cpu.step() != -1 {}
    
        assert_eq!(cpu.registors.ac, data_value as i8);
        assert!(!cpu.registors.sr.z);
        assert!(cpu.registors.sr.n); //0x99 is negative
        assert_eq!(cpu.cycle_count, 6); // +1 cycle for page crossing
    }
}

