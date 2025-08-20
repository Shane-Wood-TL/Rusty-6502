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

    pub fn stack_push(&mut self, sp: &mut u8, value: u8) {
        let addr = 0x0100 | (*sp as u16);
        self.write_byte(addr as u32, value);
        *sp = sp.wrapping_sub(1);
    }
    pub fn stack_pop(&mut self, sp: &mut u8) -> u8 {
        *sp = sp.wrapping_add(1);
        let addr = 0x0100 | (*sp as u16);
        self.read_byte(addr as u32)
    }
}





pub struct StatusRegisterValues {
    n: bool, //negative
    v: bool, //Overflow
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

pub struct Registers6502 {
    pc: u16, //program counter
    ac: u8, //accumulator
    x: u8, //x register
    y: u8, // y register 
    sr: StatusRegisterValues, //status register
    sp: u8 //stack pointer
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

pub struct Cpu6502{
    registers: Registers6502,
    memory: Memory,
    cycle_count: u32
} 

impl Cpu6502{
    pub fn new() -> Self {
        Cpu6502 {
            registers: Registers6502::new(),
            memory: Memory::new(),
            cycle_count: 0,
        }
    }
    
    fn read_u16(&mut self, address: u16) -> u16 {
        let lo = self.memory.read_byte(address as u32) as u16;
        let hi = self.memory.read_byte((address + 1) as u32) as u16;
        (hi << 8) | lo
    }
    
    fn read_u16_zero_page(&mut self, address: u8) -> u16 {
        let lo = self.memory.read_byte(address as u32) as u16;
        let hi = self.memory.read_byte(((address + 1)& 0xFF) as u32) as u16;
        (hi << 8) | lo
    }

    pub fn reset(&mut self) {
        let reset_vector = self.read_u16(0xFFFC);
        self.init_registers(reset_vector);
    }
    
    pub fn init_registers(&mut self, reset_vector: u16){
        self.registers.pc = reset_vector;
        self.registers.sp = 0xFD;
        
        self.registers.sr = StatusRegisterValues::new();
        
        
        self.cycle_count = 0;
    }
    
    pub fn step(&mut self) -> i64{
        let pc = self.registers.pc;
        let opcode = self.memory.read_byte(pc as u32);
        
        self.registers.pc += 1; //move to next byte
        
        match opcode{
            x if x == Opcodes::LdaImmediate   as u8 => self.lda_immediate(),
            x if x == Opcodes::LdaZeropage    as u8 => self.lda_zero_page(),
            x if x == Opcodes::LdaZeropageX   as u8 => self.lda_zero_page_x(),
            x if x == Opcodes::LdaAbsolute    as u8 => self.lda_absolute(),
            x if x == Opcodes::LdaAbsoluteX   as u8 => self.lda_absolute_x(),
            x if x == Opcodes::LdaAbsoluteY   as u8 => self.lda_absolute_y(),
            x if x == Opcodes::LdaIndirectX   as u8 => self.lda_indirect_x(),
            x if x == Opcodes::LdaIndirectY   as u8 => self.lda_indirect_y(),

            
            x if x == Opcodes::AdcImmediate   as u8 => self.adc_immediate(),
            x if x == Opcodes::AdcZeropage    as u8 => self.adc_zero_page(),
            x if x == Opcodes::AdcZeropageX   as u8 => self.adc_zero_page_x(),
            x if x == Opcodes::AdcAbsolute    as u8 => self.adc_absolute(),
            x if x == Opcodes::AdcAbsoluteX   as u8 => self.adc_absolute_x(),
            x if x == Opcodes::AdcAbsoluteY   as u8 => self.adc_absolute_y(),
            x if x == Opcodes::AdcIndirectX   as u8 => self.adc_indirect_x(),
            x if x == Opcodes::AdcIndirectY   as u8 => self.adc_indirect_y(),
            
            x if x == Opcodes::AndImmediate   as u8 => self.and_immediate(),
            x if x == Opcodes::AndZeropage    as u8 => self.and_zero_page(),
            x if x == Opcodes::AndZeropageX   as u8 => self.and_zero_page_x(),
            x if x == Opcodes::AndAbsolute    as u8 => self.and_absolute(),
            x if x == Opcodes::AndAbsoluteX   as u8 => self.and_absolute_x(),
            x if x == Opcodes::AndAbsoluteY   as u8 => self.and_absolute_y(),
            x if x == Opcodes::AndIndirectX   as u8 => self.and_indirect_x(),
            x if x == Opcodes::AndIndirectY   as u8 => self.and_indirect_y(),
            
            x if x == Opcodes::AslAccumulator as u8 => self.asl_accumulator(),
            x if x == Opcodes::AslZeropage    as u8 => self.asl_zero_page(),
            x if x == Opcodes::AslZeropageX   as u8 => self.asl_zero_page_x(),
            x if x == Opcodes::AslAbsolute    as u8 => self.asl_absolute(),           
            x if x == Opcodes::AslAbsoluteX   as u8 => self.asl_absolute_x(),   
            
            x if x == Opcodes::BccRelative    as u8 => self.bcc_relative(),
            
            x if x == Opcodes::BcsRelative    as u8 => self.bcs_relative(),

            x if x == Opcodes::BeqRelative    as u8 => self.beq_relative(),
            
            x if x == Opcodes::BitZeropage    as u8 => self.bit_zero_page(),
            x if x == Opcodes::BitAbsolute    as u8 => self.bit_absolute(),
            
            x if x == Opcodes::BmiRelative    as u8 => self.bmi_relative(),
            
            x if x == Opcodes::BneRelative    as u8 => self.bne_relative(),
            
            x if x == Opcodes::BplRelative    as u8 => self.bpl_relative(),
            
            x if x == Opcodes::BRK            as u8 => {self.cycle_count+=7; self.registers.sr.i = true; return -1;}
            
            x if x == Opcodes::BvcRelative    as u8 => self.bvc_relative(),
            
            x if x == Opcodes::BvsRelative    as u8 => self.bvs_relative(),
            
            x if x == Opcodes::Clc            as u8 => self.clc(),
            
            x if x == Opcodes::Cld            as u8 => self.cld(),
            
            x if x == Opcodes::Cli            as u8 => self.cli(),
            
            x if x == Opcodes::Clv            as u8 => self.clv(),
            
            x if x == Opcodes::CmpImmediate   as u8 => self.cmp_immediate(),
            x if x == Opcodes::CmpZeropage    as u8 => self.cmp_zero_page(),
            x if x == Opcodes::CmpZeropageX   as u8 => self.cmp_zero_page_x(),
            x if x == Opcodes::CmpAbsolute    as u8 => self.cmp_absolute(),
            x if x == Opcodes::CmpAbsoluteX   as u8 => self.cmp_absolute_x(),
            x if x == Opcodes::CmpAbsoluteY   as u8 => self.cmp_absolute_y(),
            x if x == Opcodes::CmpIndirectX   as u8 => self.cmp_indirect_x(),
            x if x == Opcodes::CmpIndirectY   as u8 => self.cmp_indirect_y(),
            
            x if x == Opcodes::CpxImmediate   as u8 => self.cpx_immediate(),
            x if x == Opcodes::CpxZeropage    as u8 => self.cpx_zero_page(),
            x if x == Opcodes::CpxAbsolute    as u8 => self.cpx_absolute(),
            
            x if x == Opcodes::CpyImmediate   as u8 => self.cpy_immediate(),
            x if x == Opcodes::CpyZeropage    as u8 => self.cpy_zero_page(),
            x if x == Opcodes::CpyAbsolute    as u8 => self.cpy_absolute(),
            
            x if x == Opcodes::DecZeropage    as u8 => self.dec_zero_page(),
            x if x == Opcodes::DecZeropageX   as u8 => self.dec_zero_page_x(),
            x if x == Opcodes::DecAbsolute    as u8 => self.dec_absolute(),
            x if x == Opcodes::DecAbsoluteX   as u8 => self.dec_absolute_x(),
            
            x if x == Opcodes::Dex            as u8 => self.dex(),
            
            x if x == Opcodes::Dey            as u8 => self.dey(),
            
            x if x == Opcodes::EorImmediate   as u8 => self.eor_immediate(),
            x if x == Opcodes::EorZeropage    as u8 => self.eor_zero_page(),
            x if x == Opcodes::EorZeropageX   as u8 => self.eor_zero_page_x(),
            x if x == Opcodes::EorAbsolute    as u8 => self.eor_absolute(),
            x if x == Opcodes::EorAbsoluteX   as u8 => self.eor_absolute_x(),
            x if x == Opcodes::EorAbsoluteY   as u8 => self.eor_absolute_y(),
            x if x == Opcodes::EorIndirectX   as u8 => self.eor_indirect_x(),
            x if x == Opcodes::EorIndirectY   as u8 => self.eor_indirect_y(),
            
            x if x == Opcodes::IncZeropage    as u8 => self.inc_zero_page(),
            x if x == Opcodes::IncZeropageX   as u8 => self.inc_zero_page_x(),
            x if x == Opcodes::IncAbsolute    as u8 => self.inc_absolute(),
            x if x == Opcodes::IncAbsoluteX   as u8 => self.inc_absolute_x(),
            
            x if x == Opcodes::Inx            as u8 => self.inx(),
            
            x if x == Opcodes::Iny            as u8 => self.iny(),
            
            x if x == Opcodes::JmpAbsolute    as u8 => self.jmp_absolute(),
            x if x == Opcodes::JmpIndirect    as u8 => self.jmp_indirect(),
            
            x if x == Opcodes::JsrAbsolute    as u8 => self.jsr_absolute(),
            
            x if x == Opcodes::LdxImmediate   as u8 => self.ldx_immediate(),
            x if x == Opcodes::LdxZeropage    as u8 => self.ldx_zero_page(),
            x if x == Opcodes::LdxZeropageY   as u8 => self.ldx_zero_page_y(),
            x if x == Opcodes::LdxAbsolute    as u8 => self.ldx_absolute(),
            x if x == Opcodes::LdxAbsoluteY   as u8 => self.ldx_absolute_y(),
            
            x if x == Opcodes::LdyImmediate   as u8 => self.ldy_immediate(),
            x if x == Opcodes::LdyZeropage    as u8 => self.ldy_zero_page(),
            x if x == Opcodes::LdyZeropageX   as u8 => self.ldy_zero_page_x(),
            x if x == Opcodes::LdyAbsolute    as u8 => self.ldy_absolute(),
            x if x == Opcodes::LdyAbsoluteX   as u8 => self.ldy_absolute_x(),
            
            x if x == Opcodes::LsrAccumulator as u8 => self.lsr_accumulator(),
            x if x == Opcodes::LsrZeropage    as u8 => self.lsr_zero_page(),
            x if x == Opcodes::LsrZeropageX   as u8 => self.lsr_zero_page_x(),
            x if x == Opcodes::LsrAbsolute    as u8 => self.lsr_absolute(),
            x if x == Opcodes::LsrAbsoluteX   as u8 => self.lsr_absolute_x(),
            
            x if x == Opcodes::Nop            as u8 => self.nop(),
            
            x if x == Opcodes::OraImmediate   as u8 => self.ora_immediate(),
            x if x == Opcodes::OraZeropage    as u8 => self.ora_zero_page(),
            x if x == Opcodes::OraZeropageX   as u8 => self.ora_zero_page_x(),
            x if x == Opcodes::OraAbsolute    as u8 => self.ora_absolute(),
            x if x == Opcodes::OraAbsoluteX   as u8 => self.ora_absolute_x(),
            x if x == Opcodes::OraAbsoluteY   as u8 => self.ora_absolute_y(),
            x if x == Opcodes::OraIndirectX   as u8 => self.ora_indirect_x(),
            x if x == Opcodes::OraIndirectY   as u8 => self.ora_indirect_y(),
            
            x if x == Opcodes::Pha            as u8 => self.pha(),
            
            x if x == Opcodes::Php            as u8 => self.php(),
            
            x if x == Opcodes::Pla            as u8 => self.pla(),
            
            x if x == Opcodes::Plp            as u8 => self.plp(),
            
            x if x == Opcodes::RolAccumulator as u8 => self.rol_accumulator(),
            x if x == Opcodes::RolZeropage    as u8 => self.rol_zero_page(),
            x if x == Opcodes::RolZeropageX   as u8 => self.rol_zero_page_x(),
            x if x == Opcodes::RolAbsolute    as u8 => self.rol_absolute(),
            x if x == Opcodes::RolAbsoluteX   as u8 => self.rol_absolute_x(),
            
            x if x == Opcodes::RorAccumulator as u8 => self.ror_accumulator(),
            x if x == Opcodes::RorZeropage    as u8 => self.ror_zero_page(),
            x if x == Opcodes::RorZeropageX   as u8 => self.ror_zero_page_x(),
            x if x == Opcodes::RorAbsolute    as u8 => self.ror_absolute(),
            x if x == Opcodes::RorAbsoluteX   as u8 => self.ror_absolute_x(),
            
            x if x == Opcodes::Rti            as u8 => self.rti(),
            
            x if x == Opcodes::Rts            as u8 => self.rts(),
            
            x if x == Opcodes::SbcImmediate   as u8 => self.sbc_immediate(),
            x if x == Opcodes::SbcZeropage    as u8 => self.sbc_zero_page(),
            x if x == Opcodes::SbcZeropageX   as u8 => self.sbc_zero_page_x(),
            x if x == Opcodes::SbcAbsolute    as u8 => self.sbc_absolute(),
            x if x == Opcodes::SbcAbsoluteX   as u8 => self.sbc_absolute_x(),
            x if x == Opcodes::SbcAbsoluteY   as u8 => self.sbc_absolute_y(),
            x if x == Opcodes::SbcIndirectX   as u8 => self.sbc_indirect_x(),
            x if x == Opcodes::SbcIndirectY   as u8 => self.sbc_indirect_y(),
            
            x if x == Opcodes::Sec            as u8 => self.sec(),
            
            x if x == Opcodes::Sed            as u8 => self.sed(),
            
            x if x == Opcodes::Sei            as u8 => self.sei(),
            
            x if x == Opcodes::StaZeropage    as u8 => self.sta_zero_page(),
            x if x == Opcodes::StaZeropageX   as u8 => self.sta_zero_page_x(),
            x if x == Opcodes::StaAbsolute    as u8 => self.sta_absolute(),
            x if x == Opcodes::StaAbsoluteX   as u8 => self.sta_absolute_x(),
            x if x == Opcodes::StaAbsoluteY   as u8 => self.sta_absolute_y(),
            x if x == Opcodes::StaIndirectX   as u8 => self.sta_indirect_x(),
            x if x == Opcodes::StaIndirectY   as u8 => self.sta_indirect_y(),
            
            x if x == Opcodes::StxZeropage    as u8 => self.stx_zero_page(),
            x if x == Opcodes::StxZeropageY   as u8 => self.stx_zero_page_y(),
            x if x == Opcodes::StxAbsolute    as u8 => self.stx_absolute(),
            
            x if x == Opcodes::StyZeropage    as u8 => self.sty_zero_page(),
            x if x == Opcodes::StyZeropageX   as u8 => self.sty_zero_page_x(),
            x if x == Opcodes::StyAbsolute    as u8 => self.sty_absolute(),
            
            x if x == Opcodes::Tax            as u8 => self.tax(),
            
            x if x == Opcodes::Tay            as u8 => self.tay(),
            
            x if x == Opcodes::Tsx            as u8 => self.tsx(),
            
            x if x == Opcodes::Txa            as u8 => self.txa(),
            
            x if x == Opcodes::Txs            as u8 => self.txs(),
            
            x if x == Opcodes::Tya            as u8 => self.tya(),
            
        _ =>{
            println!("opcode: {:02X}", opcode);
            return -1;
        }
        }
        return self.cycle_count as i64;
    }
    
    fn page_boundary_cross(&self, base_address: u16, offset: u8) -> bool {
        let effective_address = base_address.wrapping_add(offset as u16);
        (base_address & 0xFF00) != (effective_address & 0xFF00)
    }
    
    fn page_boundary_cross_signed(&self, base_address: u16, offset: i8) -> bool {
        let effective_address = base_address.wrapping_add(offset as i16 as u16);
        (base_address & 0xFF00) != (effective_address & 0xFF00)
    }


    //LDA Commands
    fn lda_immediate(&mut self){
        let value = self.memory.read_byte(self.registers.pc as u32);
        println!("LDA I");
        self.registers.pc += 1; //LDA immediate takes 2 bytes,
        //one was already done in step
        
        self.registers.ac = value as u8;
        
        self.cycle_count += 2;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn lda_zero_page(&mut self){
        println!("lda_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 3;
        self.registers.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let value = self.memory.read_byte(address as u32);
        
        self.registers.ac = value as u8;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
        
        println!("Store: {:02X} at 0x{:02X}", address, value);
    }
    fn lda_zero_page_x(&mut self){
        println!("lda_zero_page");
        let base_address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 4;
        self.registers.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let address = base_address.wrapping_add(self.registers.x as u8);
        
        let value = self.memory.read_byte(address as u32);
        self.registers.ac = value as u8;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn lda_absolute(&mut self){
        println!("lda_absolute");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2; //LDA absolute takes 3 bytes, one was already done in step
        
        self.cycle_count += 4;
        
        let value = self.memory.read_byte(address as u32);
        
        self.registers.ac = value as u8;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn lda_absolute_x(&mut self){
        println!("lda_absolute_x");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2; //lda_absolute_x absolute takes 3 bytes, one was already done in step
        
        let x = self.registers.x;

        if self.page_boundary_cross(address, x as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(x as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registers.ac = value as u8;
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn lda_absolute_y(&mut self){
        println!("lda_absolute_y");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2; //lda_absolute_x absolute takes 3 bytes, one was already done in step
        
        let y = self.registers.y;
        
        if self.page_boundary_cross(address, y as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registers.ac = value as u8;
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn lda_indirect_x(&mut self){
        println!("lda_indirect_x");
        self.cycle_count += 6;
        
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        let pointer_address = zero_page_operand.wrapping_add(self.registers.x as u8);

        let effective_address = self.read_u16_zero_page(pointer_address as u8);
        
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registers.ac = value as u8;
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn lda_indirect_y(&mut self){
        println!("lda_indirect_y");
        
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let y = self.registers.y;

        if self.page_boundary_cross(base_address, y as u8) {
            self.cycle_count += 6; // Page boundary crossed
        } else {
            self.cycle_count += 5; // No crossing
        }
        
        let effective_address = base_address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registers.ac = value as u8;
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    
    
    
    fn adc_immediate(&mut self) {
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let acc = self.registers.ac as u8;
        let carry_in = if self.registers.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registers.sr.c = result > 0xFF;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 2;
    }
    fn adc_zero_page(&mut self){
        println!("adc_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let acc = self.registers.ac as u8;
        let carry_in = if self.registers.sr.c { 1 } else { 0 };
        println!("A: {}", acc);
        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registers.sr.c = result > 0xFF;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 3;
        println!("ADC: A: {:#X} + mem[{:#X}]={:#X}, carry={}, result = {}", 
    self.registers.ac, address, value, self.registers.sr.c, result_byte);

    }
    fn adc_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;


        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let acc = self.registers.ac as u8;
        let carry_in = if self.registers.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registers.sr.c = result > 0xFF;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn adc_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        let value = self.memory.read_byte(address as u32);

        let acc = self.registers.ac as u8;
        let carry_in = if self.registers.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registers.sr.c = result > 0xFF;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn adc_absolute_x(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2;
        
        let x = self.registers.x;

        if self.page_boundary_cross(address, x as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(x as u16);
        let value = self.memory.read_byte(effective_address as u32);

        let acc = self.registers.ac as u8;
        let carry_in = if self.registers.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registers.sr.c = result > 0xFF;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn adc_absolute_y(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2; 
        
        let y = self.registers.y;
        
        if self.page_boundary_cross(address, y as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let acc = self.registers.ac as u8;
        let carry_in = if self.registers.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registers.sr.c = result > 0xFF;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn adc_indirect_x(&mut self){      
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        self.cycle_count += 6;

        let pointer_address = zero_page_operand.wrapping_add(self.registers.x as u8);

        let effective_address = self.read_u16_zero_page(pointer_address as u8);
        
        let value = self.memory.read_byte(effective_address as u32);
        
        let acc = self.registers.ac as u8;
        let carry_in = if self.registers.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registers.sr.c = result > 0xFF;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn adc_indirect_y(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        let y = self.registers.y;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        let effective_address = base_address.wrapping_add(y as u16);
        
        println!("base_address: {:04X}, Y: {:02X}", base_address, y);
        let crossed = self.page_boundary_cross(base_address, y as u8);
        println!("page crossed? {}", crossed);
        
        if self.page_boundary_cross(base_address, y as u8) {
            self.cycle_count += 6; // Page boundary crossed
        } else {
            self.cycle_count += 5; // No crossing
        }
        

        let value = self.memory.read_byte(effective_address as u32);
        
        let acc = self.registers.ac as u8;
        let carry_in = if self.registers.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registers.sr.c = result > 0xFF;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }


    fn and_immediate(&mut self){
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
   
        let result_byte = value & self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 2;
    }
    fn and_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 3;
        self.registers.pc += 1; 
        
        let value = self.memory.read_byte(address as u32);
        
        let result_byte = value & self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn and_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let result_byte = value & self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn and_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        let value = self.memory.read_byte(address as u32);

        let result_byte = value & (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn and_absolute_x(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2;
        
        let x = self.registers.x;

        if self.page_boundary_cross(address, x as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(x as u16);
        let value = self.memory.read_byte(effective_address as u32);

        let result_byte = value & (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn and_absolute_y(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2; 
        
        let y = self.registers.y;
        
        if self.page_boundary_cross(address, y as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value & (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn and_indirect_x(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        self.cycle_count += 6;

        let pointer_address = zero_page_operand.wrapping_add(self.registers.x as u8);

        let effective_address = self.read_u16_zero_page(pointer_address as u8);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value & (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn and_indirect_y(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let y = self.registers.y;

        if self.page_boundary_cross(base_address, y as u8) {
            self.cycle_count += 6; // Page boundary crossed
        } else {
            self.cycle_count += 5; // No crossing
        }
        
        let effective_address = base_address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value & (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }


    fn asl_accumulator(&mut self){
        //self.registers.pc += 0; only one byte, already taken care of
        let acc = self.registers.ac as u8;
        
        self.registers.sr.c = (acc & 0x80) != 0;
        let result = acc << 1;
        
        self.registers.sr.z = result == 0;
        self.registers.sr.n = (result & 0x80) != 0;
        self.registers.ac = result as u8;

        self.cycle_count += 2;
    }
    fn asl_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1; 
        
        let value = self.memory.read_byte(address as u32);
        
        self.registers.sr.c = (value & 0x80) != 0;
        let result_byte = value << 1;
        
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.memory.write_byte(address as u32, result_byte);

        self.cycle_count += 5;
    }
    fn asl_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        self.registers.sr.c = (value & 0x80) != 0;
        let result_byte = value << 1;
        
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.memory.write_byte(address as u32, result_byte);

        self.cycle_count += 6;
    }
    fn asl_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        let value = self.memory.read_byte(address as u32);

        self.registers.sr.c = (value & 0x80) != 0;
        let result_byte = value << 1;
        
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.memory.write_byte(address as u32, result_byte);

        self.cycle_count += 6;
    }
    fn asl_absolute_x(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2;
        
        let x = self.registers.x;
        
        let effective_address = address.wrapping_add(x as u16);
        let value = self.memory.read_byte(effective_address as u32);

        self.registers.sr.c = (value & 0x80) != 0;
        let result_byte = value << 1;
        
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.memory.write_byte(effective_address as u32, result_byte);

        self.cycle_count += 7;
    }
    
    
    fn bcc_relative(&mut self){ //branch on c = 0
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc += 1;

        if !self.registers.sr.c {
            let old_pc = self.registers.pc;
            let new_pc = self.registers.pc.wrapping_add(offset as u16);

            self.cycle_count += 1;  // +1 cycle for taken branch
            if self.page_boundary_cross(old_pc, offset as u8) {
                // Page crossed
                self.cycle_count += 1;  //+2 total increase
            }

            self.registers.pc = new_pc;
        } 
        self.cycle_count += 2;  //base cycle count
    }
    
    fn bcs_relative(&mut self){ //branch on c = 1
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc += 1;

        if self.registers.sr.c {
            let old_pc = self.registers.pc;
            let new_pc = self.registers.pc.wrapping_add(offset as u16);

            self.cycle_count += 1;  // +1 cycle for taken branch
            if self.page_boundary_cross(old_pc, offset as u8) {
                // Page crossed
                self.cycle_count += 1;  //+2 total increase
            }

            self.registers.pc = new_pc;
        }

        self.cycle_count += 2;  //cycle count for not take
    }


    fn beq_relative(&mut self){ //branch on z = 1
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc += 1;

        if self.registers.sr.z {
            let old_pc = self.registers.pc;
            let new_pc = self.registers.pc.wrapping_add(offset as u16);

            self.cycle_count += 1;  // +1 cycle for taken branch
            if self.page_boundary_cross(old_pc, offset as u8) {
                // Page crossed
                self.cycle_count += 1;  //+2 total increase
            }

            self.registers.pc = new_pc;
        }

        self.cycle_count += 2;  //cycle count for not take
    }
    
    
    fn bit_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1; 
        
        let value = self.memory.read_byte(address as u32);
        
        let and_result = (self.registers.ac as u8) & value;

        self.registers.sr.z = and_result == 0;
        self.registers.sr.n = (value & 0x80) != 0;
        self.registers.sr.v = (value & 0x40) != 0;

        self.cycle_count += 3; 
    }

    fn bit_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        let value = self.memory.read_byte(address as u32);
        
        let and_result = (self.registers.ac as u8) & value;

        
        self.registers.sr.z = and_result == 0;
        self.registers.sr.n = (value & 0x80) != 0;
        self.registers.sr.v = (value & 0x40) != 0;

        self.cycle_count += 4;
    }
    
    
    fn bmi_relative(&mut self){ //branch on n = 1
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc += 1;

        if self.registers.sr.n {
            let old_pc = self.registers.pc;
            let new_pc = self.registers.pc.wrapping_add(offset as u16);

            self.cycle_count += 1;  // +1 cycle for taken branch
            if self.page_boundary_cross(old_pc, offset as u8) {
                // Page crossed
                self.cycle_count += 1;  //+2 total increase
            }

            self.registers.pc = new_pc;
        }

        self.cycle_count += 2;  //cycle count for not take
    }
    
    fn bne_relative(&mut self){ //branch on z = 0
        println!("bne relative");
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc = self.registers.pc.wrapping_add(1);

        if !self.registers.sr.z {
            let old_pc = self.registers.pc;
            let new_pc = ((self.registers.pc as i32) + (offset as i32)) as u16;

            self.cycle_count += 1;  // +1 cycle for taken branch
            if self.page_boundary_cross_signed(old_pc, offset) {
                // Page crossed
                self.cycle_count += 1;  //+2 total increase
            }

            self.registers.pc = new_pc;
        } 
        self.cycle_count += 2;  //cycle count for not take
        
    }
    
    fn bpl_relative(&mut self){ //branch on n = 0
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
        if !self.registers.sr.n {
            let old_pc = self.registers.pc;
            let new_pc = old_pc.wrapping_add(offset as i16 as u16);


            println!(
                "old_pc: {:04X}, offset: {}, new_pc: {:04X}",
                old_pc, offset, new_pc
            );
            self.cycle_count += 1;
            if self.page_boundary_cross_signed(old_pc, offset) {
                self.cycle_count += 1; 
            }

            self.registers.pc = new_pc;
        }

        self.cycle_count += 2;
    }
    
    fn bvc_relative(&mut self){ //branch on v = 0
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc += 1;

        if !self.registers.sr.v {
            let old_pc = self.registers.pc;
            let new_pc = old_pc.wrapping_add(offset as i16 as u16);

            self.cycle_count += 1;  // +1 cycle for taken branch
            if self.page_boundary_cross_signed(old_pc, offset) {
                // Page crossed
                self.cycle_count += 1;  //+2 total increase
            }

            self.registers.pc = new_pc;
        }

        self.cycle_count += 2;  //cycle count for not take 
    }
    
    fn  bvs_relative(&mut self){ //branch on v = 1
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc += 1;

        if self.registers.sr.v {
            let old_pc = self.registers.pc;
            let new_pc = self.registers.pc.wrapping_add(offset as u16);

            self.cycle_count += 1;  // +1 cycle for taken branch
            if self.page_boundary_cross(old_pc, offset as u8) {
                // Page crossed
                self.cycle_count += 1;  //+2 total increase
            }

            self.registers.pc = new_pc;
        }

        self.cycle_count += 2;  //cycle count for not take 
    }
    
    fn clc(&mut self){
        println!("clc");
        self.registers.sr.c = false;
        self.cycle_count += 2;
        //self.registers.pc += 0; //clc takes no operands
    }
    
    fn cld(&mut self){
        println!("cld");
        self.registers.sr.d = false;
        self.cycle_count += 2;
        //self.registers.pc += 0; //cld takes no operands
    }
    
    fn cli(&mut self){
        println!("cli");
        self.registers.sr.i = false;
        self.cycle_count += 2;
        //self.registers.pc += 0; //cli takes no operands
    }
    
    fn clv(&mut self){
        println!("clv");
        self.registers.sr.v = false;
        self.cycle_count += 2;
        //self.registers.pc += 0; //clv takes no operands
    }
    
    
    fn cmp_immediate(&mut self){
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let acc = self.registers.ac;
        let result = acc.wrapping_sub(value);

        self.registers.sr.c = acc >= value;
        self.registers.sr.z = acc == value;
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    fn cmp_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let acc = self.registers.ac;
        let result = acc.wrapping_sub(value);

        self.registers.sr.c = acc >= value;
        self.registers.sr.z = acc == value;
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 3;
    }
    fn cmp_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let acc = self.registers.ac;
        let result = acc.wrapping_sub(value);

        self.registers.sr.c = acc >= (value);
        self.registers.sr.z = acc == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 4;
    }
    fn cmp_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        let value = self.memory.read_byte(address as u32);

        let acc = self.registers.ac;
        let result = acc.wrapping_sub(value);

        self.registers.sr.c = acc >= (value);
        self.registers.sr.z = acc == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 4;
    }
    fn cmp_absolute_x(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2;
        
        let x = self.registers.x;

        if self.page_boundary_cross(address, x as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(x as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let acc = self.registers.ac;
        let result = acc.wrapping_sub(value);

        self.registers.sr.c = acc >= (value);
        self.registers.sr.z = acc == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;
    }
    fn cmp_absolute_y(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2;
        
        let y = self.registers.y;
        
        if self.page_boundary_cross(address, y as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let acc = self.registers.ac;
        let result = acc.wrapping_sub(value);

        self.registers.sr.c = acc >= (value);
        self.registers.sr.z = acc == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;
    }
    fn cmp_indirect_x(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        let pointer_address = zero_page_operand.wrapping_add(self.registers.x as u8);

        let effective_address = self.read_u16_zero_page(pointer_address as u8);
        
        let value = self.memory.read_byte(effective_address as u32);
        
        let acc = self.registers.ac;
        let result = acc.wrapping_sub(value);

        self.registers.sr.c = acc >= (value);
        self.registers.sr.z = acc == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 6;
    }
    fn cmp_indirect_y(&mut self){       
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let y = self.registers.y;

        if self.page_boundary_cross(base_address, y as u8) {
            self.cycle_count += 6; // Page boundary crossed
        } else {
            self.cycle_count += 5; // No crossing
        }
        
        let effective_address = base_address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let acc = self.registers.ac;
        let result = acc.wrapping_sub(value);

        self.registers.sr.c = acc >= (value);
        self.registers.sr.z = acc == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;
    }


    fn cpx_immediate(&mut self){
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let x = self.registers.x;
        let result = x.wrapping_sub(value);

        self.registers.sr.c = x >= (value);
        self.registers.sr.z = x == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    fn cpx_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let x = self.registers.x;
        let result = x.wrapping_sub(value);

        self.registers.sr.c = x >= (value);
        self.registers.sr.z = x == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 3;
    }
    fn cpx_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        let value = self.memory.read_byte(address as u32);

        let x = self.registers.x;
        let result = x.wrapping_sub(value);

        self.registers.sr.c = x >= (value);
        self.registers.sr.z = x == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 4;
    }
    
    
    fn cpy_immediate(&mut self){
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let y = self.registers.y;
        let result = y.wrapping_sub(value);

        self.registers.sr.c = y >= (value);
        self.registers.sr.z = y == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    fn cpy_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let y = self.registers.y;
        let result = y.wrapping_sub(value);

        self.registers.sr.c = y >= (value);
        self.registers.sr.z = y == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 3;
    }
    fn cpy_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        let value = self.memory.read_byte(address as u32);

        let y = self.registers.y;
        let result = y.wrapping_sub(value);

        self.registers.sr.c = y >= (value);
        self.registers.sr.z = y == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 4;
    }
    
    
    fn dec_zero_page(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let mut value = self.memory.read_byte(base_address as u32);
        value = value.wrapping_sub(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value as u8 & 0x80) != 0;

        self.memory.write_byte(base_address as u32, value as u8);

        self.cycle_count += 5;
    }
    fn dec_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let mut value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        value = value.wrapping_sub(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value as u8 & 0x80) != 0;

        self.memory.write_byte(address as u32, value as u8);

        self.cycle_count += 6;
    }
    fn dec_absolute(&mut self){
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
 
        let mut value = self.memory.read_byte(base_address as u32);
        value = value.wrapping_sub(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;

        self.memory.write_byte(base_address as u32, value);

        self.cycle_count += 6;
    }
    fn dec_absolute_x(&mut self){
        let zero_page_operand = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
        
        let effective_address = zero_page_operand.wrapping_add(self.registers.x as u16);

        let mut value = self.memory.read_byte(effective_address as u32);
        value = value.wrapping_sub(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;

        self.memory.write_byte(effective_address as u32, value);

        self.cycle_count += 7;
    }
    
    
    fn dex(&mut self){
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.registers.sr.z  = self.registers.x == 0;
        self.registers.sr.n = (self.registers.x & 0x80) != 0;
        self.cycle_count += 2;
    }
    
    fn dey(&mut self){
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.registers.sr.z  = self.registers.y == 0;
        self.registers.sr.n = (self.registers.y & 0x80) != 0;
        self.cycle_count += 2;
    }
    
    
    fn eor_immediate(&mut self){
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
   
        let result_byte = value ^ self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 2;
    }
    fn eor_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 3;
        self.registers.pc += 1; 
        
        let value = self.memory.read_byte(address as u32);
        
        let result_byte = value ^ self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn eor_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let result_byte = value ^ self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn eor_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        let value = self.memory.read_byte(address as u32);

        let result_byte = value ^ (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn eor_absolute_x(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2;
        
        let x = self.registers.x;

        if self.page_boundary_cross(address, x as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(x as u16);
        let value = self.memory.read_byte(effective_address as u32);

        let result_byte = value ^ (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn eor_absolute_y(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2; 
        
        let y = self.registers.y;
        
        if self.page_boundary_cross(address, y as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value ^ (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn eor_indirect_x(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        self.cycle_count += 6;

        let pointer_address = zero_page_operand.wrapping_add(self.registers.x as u8);

        let effective_address = self.read_u16_zero_page(pointer_address as u8);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value ^ (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn eor_indirect_y(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let y = self.registers.y;

        if self.page_boundary_cross(base_address, y as u8) {
            self.cycle_count += 6; // Page boundary crossed
        } else {
            self.cycle_count += 5; // No crossing
        }
        
        let effective_address = base_address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value ^ (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    
    
    fn inc_zero_page(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let mut value = self.memory.read_byte(base_address as u32);
        value = value.wrapping_add(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value as u8 & 0x80) != 0;

        self.memory.write_byte(base_address as u32, value as u8);

        self.cycle_count += 5;
    }
    fn inc_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let mut value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        value = value.wrapping_add(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value as u8 & 0x80) != 0;

        self.memory.write_byte(address as u32, value as u8);

        self.cycle_count += 6;
    }
    fn inc_absolute(&mut self){
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
 
        let mut value = self.memory.read_byte(base_address as u32);
        value = value.wrapping_add(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;

        self.memory.write_byte(base_address as u32, value);

        self.cycle_count += 6;
    }
    fn inc_absolute_x(&mut self){
        let zero_page_operand = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
        
        let pointer_address = zero_page_operand.wrapping_add(self.registers.x as u16);

        let mut value = self.memory.read_byte(pointer_address as u32);
        value = value.wrapping_add(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;

        self.memory.write_byte(pointer_address as u32, value);

        self.cycle_count += 7;
    }
    
    
    fn inx(&mut self){
        self.registers.x = self.registers.x.wrapping_add(1);
        self.registers.sr.z  = self.registers.x == 0;
        self.registers.sr.n = (self.registers.x & 0x80) != 0;
        self.cycle_count += 2;
    }
    
    fn iny(&mut self){
        self.registers.y = self.registers.y.wrapping_add(1);
        self.registers.sr.z  = self.registers.y == 0;
        self.registers.sr.n = (self.registers.y & 0x80) != 0;
        self.cycle_count += 2;
    }
    
    
    fn jmp_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
        self.registers.pc = address;
        self.cycle_count += 3;
    }

    fn jmp_indirect(&mut self) {
        let addr_ptr = self.read_u16(self.registers.pc);
        self.registers.pc += 2;


        let low_byte = self.memory.read_byte(addr_ptr as u32);
        let high_addr = if (addr_ptr & 0x00FF) == 0x00FF {
            (addr_ptr & 0xFF00) as u32 
        } else {
            (addr_ptr + 1) as u32
        };
        let high_byte = self.memory.read_byte(high_addr);

        let target = ((high_byte as u16) << 8) | low_byte as u16;
        self.registers.pc = target;
        self.cycle_count += 5;
    }
    
    fn jsr_absolute(&mut self){
        println!("PC at start: {:04X}", self.registers.pc);
        let base_address = self.read_u16(self.registers.pc);
        let return_address = self.registers.pc + 1;

        let high = (return_address >> 8) as u8;
        let low = (return_address & 0xFF) as u8;

        self.memory.stack_push(&mut self.registers.sp, high);
        self.memory.stack_push(&mut self.registers.sp, low);

        self.registers.pc = base_address;

        self.cycle_count += 6;
    }
    
    
    //LDX Commands
    fn ldx_immediate(&mut self){
        let value = self.memory.read_byte(self.registers.pc as u32);
        println!("LDX I");
        self.registers.pc += 1; //LDX immediate takes 2 bytes,
        //one was already done in step
        
        self.registers.x = value;
        
        self.cycle_count += 2;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn ldx_zero_page(&mut self){
        println!("ldx_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 3;
        self.registers.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let value = self.memory.read_byte(address as u32);
        
        self.registers.x = value;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn ldx_zero_page_y(&mut self){
        println!("ldx_zero_page_y");
        let base_address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 4;
        self.registers.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let address = base_address.wrapping_add(self.registers.y as u8);
        
        let value = self.memory.read_byte(address as u32);
        self.registers.x = value;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn ldx_absolute(&mut self){
        println!("ldx_absolute");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2); //LDX absolute takes 3 bytes, one was already done in step
        
        self.cycle_count += 4;
        
        let value = self.memory.read_byte(address as u32);
        
        self.registers.x = value;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn ldx_absolute_y(&mut self){
        println!("ldx_absolute_y");
        
        let zero_page_operand = self.read_u16(self.registers.pc as u16);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
        let effective_address = zero_page_operand.wrapping_add(self.registers.y as u16);

        if self.page_boundary_cross(zero_page_operand, self.registers.y as u8) {
            self.cycle_count += 5;
        } else {
            self.cycle_count += 4;
        }
        
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registers.x = value;
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    
    //LDY Commands
    fn ldy_immediate(&mut self){
        let value = self.memory.read_byte(self.registers.pc as u32);
        println!("LDY I");
        self.registers.pc += 1; //LDY immediate takes 2 bytes,
        //one was already done in step
        
        self.registers.y = value;
        
        self.cycle_count += 2;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn ldy_zero_page(&mut self){
        println!("ldy_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 3;
        self.registers.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let value = self.memory.read_byte(address as u32);
        
        self.registers.y = value;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn ldy_zero_page_x(&mut self){
        println!("ldy_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 4;
        self.registers.pc += 1; //LDA zero page takes 2 bytes, one was already done in step
        
        let address = base_address.wrapping_add(self.registers.x as u8);
        
        let value = self.memory.read_byte(address as u32);
        self.registers.y = value;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn ldy_absolute(&mut self){
        println!("ldy_absolute");
        let address = self.read_u16(self.registers.pc);
        
       self.registers.pc = self.registers.pc.wrapping_add(2); //LDY absolute takes 3 bytes, one was already done in step
        
        self.cycle_count += 4;
        
        let value = self.memory.read_byte(address as u32);
        
        self.registers.y = value;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn ldy_absolute_x(&mut self){
        println!("ldy_absolute_x");
        
        let zero_page_operand = self.read_u16(self.registers.pc as u16);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
        let effective_address = zero_page_operand.wrapping_add(self.registers.x as u16);

        if self.page_boundary_cross(zero_page_operand, self.registers.x as u8) {
            self.cycle_count += 5;
        } else {
            self.cycle_count += 4;
        }
        
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registers.y = value;
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    
    fn lsr_accumulator(&mut self){
        let acc = self.registers.ac;

        self.registers.sr.c = (acc & 0x01) != 0;

        let value = acc >> 1;
        self.registers.ac = value;

        self.registers.sr.z = value == 0;
        self.registers.sr.n = false;

        self.cycle_count += 2;
    }
    fn lsr_zero_page(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let mut value = self.memory.read_byte(base_address as u32);

        self.registers.sr.c = (value & 0x01) != 0;

        value = value >> 1;

        self.registers.sr.z = value == 0;
        self.registers.sr.n = false;

        self.memory.write_byte(base_address as u32, value as u8);

        self.cycle_count += 5;
    }
    fn lsr_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let mut value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory

        self.registers.sr.c = (value & 0x01) != 0;

        value = value >> 1;

        self.registers.sr.z = value == 0;
        self.registers.sr.n = false;

        self.memory.write_byte(address as u32, value as u8);

        self.cycle_count += 6;
    }
    fn lsr_absolute(&mut self){
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
 
        let mut value = self.memory.read_byte(base_address as u32);

        self.registers.sr.c = (value & 0x01) != 0;

        value = value >> 1;

        self.registers.sr.z = value == 0;
        self.registers.sr.n = false;

        self.memory.write_byte(base_address as u32, value as u8);

        self.cycle_count += 6;
    }
    fn lsr_absolute_x(&mut self){
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        let effective_address = base_address.wrapping_add(self.registers.x as u16);
        
        let mut value = self.memory.read_byte(effective_address as u32);

        self.registers.sr.c = (value & 0x01) != 0;

        value = value >> 1;

        self.registers.sr.z = value == 0;
        self.registers.sr.n = false;

        self.memory.write_byte(effective_address as u32, value);

        self.cycle_count += 7;
    }
    
    fn nop(&mut self){
        println!("nop");
        self.cycle_count += 2;
        //self.registers.pc += 0; //nop takes no operands
    }
    
    
    fn ora_immediate(&mut self){
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
   
        let result_byte = value | self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 2;
    }
    fn ora_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 3;
        self.registers.pc += 1; 
        
        let value = self.memory.read_byte(address as u32);
        
        let result_byte = value | self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn ora_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let result_byte = value | self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn ora_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        let value = self.memory.read_byte(address as u32);

        let result_byte = value | (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn ora_absolute_x(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2;
        
        let x = self.registers.x;

        if self.page_boundary_cross(address, x as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(x as u16);
        let value = self.memory.read_byte(effective_address as u32);

        let result_byte = value | (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn ora_absolute_y(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2; 
        
        let y = self.registers.y;
        
        if self.page_boundary_cross(address, y as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value | (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn ora_indirect_x(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        self.cycle_count += 6;

        let pointer_address = zero_page_operand.wrapping_add(self.registers.x as u8);

        let effective_address = self.read_u16_zero_page(pointer_address as u8);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value | (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn ora_indirect_y(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let y = self.registers.y;

        if self.page_boundary_cross(base_address, y as u8) {
            self.cycle_count += 6; // Page boundary crossed
        } else {
            self.cycle_count += 5; // No crossing
        }
        
        let effective_address = base_address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value | (self.registers.ac as u8);

        
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    
    fn pha(&mut self){
        self.memory.stack_push(&mut self.registers.sp, self.registers.ac as u8);
        self.cycle_count += 3;
    }
    
    fn php(&mut self){
        self.memory.stack_push(&mut self.registers.sp, self.registers.sr.to_byte() as u8);
        self.cycle_count += 3;
    }
    
    fn pla(&mut self){
        self.registers.ac = self.memory.stack_pop(&mut self.registers.sp);

        self.registers.sr.z = self.registers.ac == 0;
        self.registers.sr.n = (self.registers.ac & 0x80) != 0;

        self.cycle_count += 4;
    }
    
    fn plp(&mut self){
        let new_status = self.memory.stack_pop(&mut self.registers.sp);
        println!("PLP read status byte: 0x{:02X}", new_status);
        self.registers.sr.from_byte(new_status);
        self.cycle_count += 4;
    }
    
    
    
    fn rol_accumulator(&mut self){
        let acc = self.registers.ac;

        let carry_in = if self.registers.sr.c { 1 } else { 0 };

        self.registers.sr.c = (acc & 0x80) != 0;

        let value = (acc << 1) | carry_in;
        self.registers.ac = value;
        
        self.registers.sr.z = self.registers.ac == 0;
        self.registers.sr.n = (self.registers.ac & 0x80) != 0;

        self.cycle_count += 2;
    }
    fn rol_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1; 
        
        let value = self.memory.read_byte(address as u32);

        let carry_in = if self.registers.sr.c { 1 } else { 0 };
        self.registers.sr.c = (value & 0x80) != 0;
        
        let result_byte = (value << 1) | carry_in; 

        self.memory.write_byte(address as u32, result_byte as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;

        self.cycle_count += 5;
    }
    fn rol_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;
        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;
        let value = self.memory.read_byte(address as u32);


        let carry_in = if self.registers.sr.c { 1 } else { 0 };
        self.registers.sr.c = (value & 0x80) != 0;
        

        let result_byte = (value << 1) | carry_in; 


        self.memory.write_byte(address as u32, result_byte as u8);
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;

        self.cycle_count += 6;
    }
    fn rol_absolute(&mut self){
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
 
        let value = self.memory.read_byte(base_address as u32);

        let carry_in = if self.registers.sr.c { 1 } else { 0 };
        self.registers.sr.c = (value & 0x80) != 0;
        

        let result_byte = (value << 1) | carry_in; 


        self.memory.write_byte(base_address as u32, result_byte as u8);
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;

        self.cycle_count += 6;
    }
    fn rol_absolute_x(&mut self){
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
        
        let effective_address = base_address.wrapping_add(self.registers.x as u16);
        
        let value = self.memory.read_byte(effective_address as u32);
        

        let carry_in = if self.registers.sr.c { 1 } else { 0 };
        self.registers.sr.c = (value & 0x80) != 0;
        
        let result_byte = (value << 1) | carry_in; 


        self.memory.write_byte(effective_address as u32, result_byte as u8);
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;

        self.cycle_count += 7;
    }
    


    fn ror_accumulator(&mut self){
        let acc = self.registers.ac;

        let carry_in = if self.registers.sr.c { 1 } else { 0 };

        self.registers.sr.c = (acc & 0x01) != 0;

        let value = (acc >> 1) | (carry_in << 7);
        self.registers.ac = value;
        
        self.registers.sr.z = self.registers.ac == 0;
        self.registers.sr.n = (self.registers.ac & 0x80) != 0;

        self.cycle_count += 2;
    }
    fn ror_zero_page(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;
        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;
        let value = self.memory.read_byte(address as u32);


        let carry_in = if self.registers.sr.c { 1 } else { 0 };
        self.registers.sr.c = (value & 0x01) != 0;
        

        let result_byte = (value >> 1) |  (carry_in << 7); 


        self.memory.write_byte(address as u32, result_byte as u8);
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;

        self.cycle_count += 5;
    }
    fn ror_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;
        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;
        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let carry_in = if self.registers.sr.c { 1 } else { 0 };
        self.registers.sr.c = (value & 0x01) != 0;

        let result_byte = (value >> 1) |  (carry_in << 7); 


        self.memory.write_byte(address as u32, result_byte as u8);
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;

        self.cycle_count += 6;
    }
    fn ror_absolute(&mut self){
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
 
        let value = self.memory.read_byte(base_address as u32);

        let carry_in = if self.registers.sr.c { 1 } else { 0 };
        self.registers.sr.c = (value & 0x01) != 0;
        

        let result_byte = (value >> 1) |  (carry_in << 7); 


        self.memory.write_byte(base_address as u32, result_byte as u8);
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;

        self.cycle_count += 6;
    }
    fn ror_absolute_x(&mut self){
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
        
       let effective_address = base_address.wrapping_add(self.registers.x as u16);
        
        let value = self.memory.read_byte(effective_address as u32);
        

        let carry_in = if self.registers.sr.c { 1 } else { 0 };
        self.registers.sr.c = (value & 0x01) != 0;
        
        let result_byte = (value >> 1) |  (carry_in << 7); 


        self.memory.write_byte(effective_address as u32, result_byte as u8);
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;

        self.cycle_count += 7;
    }
    
    fn rti(&mut self){
        //The status register is pulled with the break flag
        //and bit 5 ignored. Then PC is pulled from the stack.
        let status = self.memory.stack_pop(&mut self.registers.sp);

        let status_byte = status & 0b11101111; // Clear b and not used
    
        self.registers.sr.from_byte(status_byte);

        let pcl = self.memory.stack_pop(&mut self.registers.sp);
        let pch = self.memory.stack_pop(&mut self.registers.sp);

        self.registers.pc = ((pch as u16) << 8) | (pcl as u16);

        self.cycle_count += 6;
    }
    
    fn rts(&mut self){
        //pull PC, PC+1 -> P
        let pcl = self.memory.stack_pop(&mut self.registers.sp);
        let pch = self.memory.stack_pop(&mut self.registers.sp);
        
        self.registers.pc = ((pch as u16) << 8) | (pcl as u16);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
        self.cycle_count += 6;
    }
    
    
    fn sbc_immediate(&mut self){
        //A - M - C̅ -> A
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;

        let carry_in = if self.registers.sr.c { 0 } else { 1 };
        let acc = self.registers.ac;

        let result = (acc as i16) - (value as i16) - (carry_in as i16);
        let result_byte = result as u8;

        self.registers.sr.c = result >= 0;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ value) & 0x80 != 0) && ((acc ^ result_byte) & 0x80 != 0);

        self.registers.ac = result_byte;
        self.cycle_count += 2;        
    }
    fn sbc_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1; 
        
        let value = self.memory.read_byte(address as u32);
        let carry_in = if self.registers.sr.c { 0 } else { 1 };
        let acc = self.registers.ac;

        let result = (acc as i16) - (value as i16) - (carry_in as i16);
        let result_byte = result as u8;

        self.registers.sr.c = result >= 0;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ value) & 0x80 != 0) && ((acc ^ result_byte) & 0x80 != 0);

        self.registers.ac = result_byte;
        self.cycle_count += 3;        
    }
    fn sbc_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        let carry_in = if self.registers.sr.c { 0 } else { 1 };
        let acc = self.registers.ac;

        let result = (acc as i16) - (value as i16) - (carry_in as i16);
        let result_byte = result as u8;

        self.registers.sr.c = result >= 0;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ value) & 0x80 != 0) && ((acc ^ result_byte) & 0x80 != 0);

        self.registers.ac = result_byte;
        self.cycle_count += 4;    
    }
    fn sbc_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        let value = self.memory.read_byte(address as u32);
        let carry_in = if self.registers.sr.c { 0 } else { 1 };
        let acc = self.registers.ac;

        let result = (acc as i16) - (value as i16) - (carry_in as i16);
        let result_byte = result as u8;

        self.registers.sr.c = result >= 0;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ value) & 0x80 != 0) && ((acc ^ result_byte) & 0x80 != 0);

        self.registers.ac = result_byte;
        self.cycle_count += 4; 
    }
    fn sbc_absolute_x(&mut self){
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
        
        let x = self.registers.x;
        
        if self.page_boundary_cross(base_address, x as u8) {
            self.cycle_count += 5; // Page boundary crossed
        } else {
            self.cycle_count += 4; // No crossing
        }
        let effective_address = base_address.wrapping_add(x as u16);


        let value = self.memory.read_byte(effective_address as u32);
        let carry_in = if self.registers.sr.c { 0 } else { 1 };
        let acc = self.registers.ac;

        let result = (acc as i16) - (value as i16) - (carry_in as i16);
        let result_byte = result as u8;

        self.registers.sr.c = result >= 0;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ value) & 0x80 != 0) && ((acc ^ result_byte) & 0x80 != 0);

        self.registers.ac = result_byte;
    }
    fn sbc_absolute_y(&mut self){
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;
        
        let y = self.registers.y;
        
        if self.page_boundary_cross(base_address, y as u8) {
            self.cycle_count += 5; // Page boundary crossed
        } else {
            self.cycle_count += 4; // No crossing
        }
        let effective_address = base_address.wrapping_add(y as u16);

        
        let value = self.memory.read_byte(effective_address as u32);
        let carry_in = if self.registers.sr.c { 0 } else { 1 };
        let acc = self.registers.ac;

        let result = (acc as i16) - (value as i16) - (carry_in as i16);
        let result_byte = result as u8;

        self.registers.sr.c = result >= 0;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ value) & 0x80 != 0) && ((acc ^ result_byte) & 0x80 != 0);

        self.registers.ac = result_byte;
    }
    fn sbc_indirect_x(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        


        let pointer_address = zero_page_operand.wrapping_add(self.registers.x as u8);
        let effective_address = self.read_u16_zero_page(pointer_address as u8);
        let value = self.memory.read_byte(effective_address as u32);
        let carry_in = if self.registers.sr.c { 0 } else { 1 };
        let acc = self.registers.ac;

        let result = (acc as i16) - (value as i16) - (carry_in as i16);
        let result_byte = result as u8;

        self.registers.sr.c = result >= 0;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ value) & 0x80 != 0) && ((acc ^ result_byte) & 0x80 != 0);

        self.registers.ac = result_byte;

        self.cycle_count += 6;
    }

    fn sbc_indirect_y(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        let y = self.registers.y;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        let effective_address = base_address.wrapping_add(y as u16);
        println!("base {}", base_address);
        println!("y {}", y);
        println!("effective_address {}", effective_address);
        if self.page_boundary_cross(base_address, y as u8) {
            self.cycle_count += 6; // Page boundary crossed
        } else {
            self.cycle_count += 5; // No crossing
        }

        let value = self.memory.read_byte(effective_address as u32);

        let acc = self.registers.ac;
        let carry_in = if self.registers.sr.c { 0 } else { 1 };
        
        let result = (acc as i16) - (value as i16) - (carry_in as i16);
        let result_byte = result as u8;

        self.registers.sr.c = result >= 0;
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.sr.v = ((acc ^ value) & 0x80 != 0) && ((acc ^ result_byte) & 0x80 != 0);

        self.registers.ac = result_byte;
    }
    
    fn sec(&mut self){
        self.registers.sr.c = true;
        self.cycle_count += 2;
    }
    
    fn sed(&mut self){
        self.registers.sr.d = true;
        self.cycle_count += 2;
    }
    
    fn sei(&mut self){
        self.registers.sr.i = true;
        self.cycle_count += 2;
        
    }
    
    
    fn sta_zero_page(&mut self){
        println!("sta_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1; 
        
        self.memory.write_byte(address as u32, self.registers.ac);
        
        self.cycle_count += 3;
        
        println!("STA: mem[{:#X}] = {:#X}", address, self.registers.ac);

    }
    fn sta_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;
        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        self.memory.write_byte(address as u32, self.registers.ac);
        
        self.cycle_count += 4;
    }
    fn sta_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        self.memory.write_byte(address as u32, self.registers.ac);
        
        self.cycle_count += 4;
    }
    fn sta_absolute_x(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2;
        let x = self.registers.x;
        let effective_address = address.wrapping_add(x as u16);

        self.memory.write_byte(effective_address as u32, self.registers.ac);

        self.cycle_count += 5;
    }
    fn sta_absolute_y(&mut self){
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc += 2;
        let y = self.registers.y;
        let effective_address = address.wrapping_add(y as u16);

        self.memory.write_byte(effective_address as u32, self.registers.ac);

        self.cycle_count += 5;
    }
    fn sta_indirect_x(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        let pointer_address = zero_page_operand.wrapping_add(self.registers.x as u8);
        let effective_address = self.read_u16_zero_page(pointer_address as u8);

        self.memory.write_byte(effective_address as u32, self.registers.ac);

        self.cycle_count += 6;
    }
    fn sta_indirect_y(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let y = self.registers.y;
        let effective_address = base_address.wrapping_add(y as u16);

        self.memory.write_byte(effective_address as u32, self.registers.ac);

        self.cycle_count += 6;
    }
    
    
    fn stx_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1; 
        
        self.memory.write_byte(address as u32, self.registers.x);
        
        self.cycle_count += 3;
    }
    fn stx_zero_page_y(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;
        let address = base_address.wrapping_add(self.registers.y as u8);

        self.memory.write_byte(address as u32, self.registers.x);
        
        self.cycle_count += 4;
    }
    fn stx_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        self.memory.write_byte(address as u32, self.registers.x);
        
        self.cycle_count += 4;
    }
    
    
    fn sty_zero_page(&mut self){
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc += 1; 
        
        self.memory.write_byte(address as u32, self.registers.y);
        
        self.cycle_count += 3;
    }
    fn sty_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc += 1;
        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        self.memory.write_byte(address as u32, self.registers.y);
        
        self.cycle_count += 4;
    }
    fn sty_absolute(&mut self){
        let address = self.read_u16(self.registers.pc);
        self.registers.pc += 2;

        self.memory.write_byte(address as u32, self.registers.y);
        
        self.cycle_count += 4;
    }
    
    fn tax(&mut self){
        self.registers.x = self.registers.ac;

        self.registers.sr.z = self.registers.x == 0;
        self.registers.sr.n = (self.registers.x as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    
    fn tay(&mut self){
        self.registers.y = self.registers.ac;

        self.registers.sr.z = self.registers.y == 0;
        self.registers.sr.n = (self.registers.y as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    
    fn tsx(&mut self){
        self.registers.x = self.registers.sp;

        self.registers.sr.z = self.registers.x == 0;
        self.registers.sr.n = (self.registers.x as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    
    fn txa(&mut self){
        self.registers.ac = self.registers.x;

        self.registers.sr.z = self.registers.ac == 0;
        self.registers.sr.n = (self.registers.ac as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    
    fn txs(&mut self){
        self.registers.sp = self.registers.x as u8;
        self.cycle_count += 2;
    }
    
    fn tya(&mut self){
        self.registers.ac = self.registers.y;

        self.registers.sr.z = self.registers.ac == 0;
        self.registers.sr.n = (self.registers.ac as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
}



































fn main(){
    let mut cpu = Cpu6502::new();

    // Set reset vector to $8000
    cpu.memory.write_byte(0xFFFC, 0x00);
    cpu.memory.write_byte(0xFFFD, 0x80);

    cpu.reset();
}










































#[cfg(test)]
mod tests {
    use super::*; 
    
    
    fn setup_cpu_with_program(program: &[u8]) -> Cpu6502 {
        let mut cpu = Cpu6502::new();

        // Set the reset vector to 0x8000
        cpu.memory.write_byte(0xFFFC, 0x00);
        cpu.memory.write_byte(0xFFFD, 0x80);
        cpu.reset();

        // Load the program into memory at 0x8000
        for (i, byte) in program.iter().enumerate() {
            cpu.memory.write_byte(0x8000 + i as u32, *byte);
        }
        cpu
    }
    
    #[test]
    fn test_lda_immediate() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdaImmediate as u8, 0x42 // LDA #$42
        ]);
        cpu.step();
        assert_eq!(cpu.registers.ac, 0x42);
        assert!(!cpu.registers.sr.z); // not zero
        assert!(!cpu.registers.sr.n); // not negative
        assert_eq!(cpu.cycle_count, 2);
    }
    
    #[test]
    fn test_lda_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdaZeropage as u8, 0x10 // LDA $10
        ]);
        cpu.memory.write_byte(0x0010, 0x84);
        cpu.step();
        assert_eq!(cpu.registers.ac, 0x84);
        assert!(!cpu.registers.sr.z);
        assert!(cpu.registers.sr.n); // should be negative
        assert_eq!(cpu.cycle_count, 3);
    }
    
    #[test]
    fn test_lda_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdaZeropageX as u8, 0x10 // LDA $10,X
        ]);
        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x0015, 0x12);
        cpu.step();
        assert_eq!(cpu.registers.ac, 0x12);
        assert_eq!(cpu.cycle_count, 4);
    }
    
    #[test]
    fn test_lda_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdaAbsolute as u8, 0x00, 0x20 // LDA $2000
        ]);
        cpu.memory.write_byte(0x2000, 0xAA);
        cpu.step();
        assert_eq!(cpu.registers.ac, 0xAA);
        assert_eq!(cpu.cycle_count, 4);
    }
    
    #[test]
    fn test_lda_absolute_x_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdaAbsoluteX as u8, 0x10, 0x20 // LDA $2010,X
        ]);
        cpu.registers.x = 0x0F; // $2010 + $0F = $201F (no page cross)
        cpu.memory.write_byte(0x201F, 0x42);
        cpu.step();
    
        assert_eq!(cpu.registers.ac, 0x42);
        assert!(!cpu.registers.sr.z);
        assert!(!cpu.registers.sr.n);
        assert_eq!(cpu.cycle_count, 4);
    }
    
    #[test]
    fn test_lda_absolute_x_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdaAbsoluteX as u8, 0xFF, 0x20 // LDA $20FF,X
        ]);
        cpu.registers.x = 0x01; // $20FF + $01 = $2100 (page crossed)
        cpu.memory.write_byte(0x2100, 0x99);
        cpu.step();
    
        assert_eq!(cpu.registers.ac, 0x99);
        assert!(!cpu.registers.sr.z);
        assert!(cpu.registers.sr.n); 
        assert_eq!(cpu.cycle_count, 5);
    }
    
    #[test]
    fn test_lda_indirect_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdaIndirectX as u8, 0x10 // LDA ($10,X)
        ]);
    
        cpu.registers.x = 0x04;
    
        // $10 + $04 = $14
        cpu.memory.write_byte(0x0014, 0x00); // low byte
        cpu.memory.write_byte(0x0015, 0x90); // high byte
    
        cpu.memory.write_byte(0x9000, 0xA5); // value to load
    
        cpu.step();
        assert_eq!(cpu.registers.ac, 0xA5);
        assert!(!cpu.registers.sr.z);
        assert!(cpu.registers.sr.n); // 0xA5 = negative
        
        assert_eq!(cpu.cycle_count, 6);
    }
    
    #[test]
    fn test_lda_indirect_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdaIndirectY as u8, 0x10 // LDA ($10),Y
        ]);
    
        cpu.memory.write_byte(0x0010, 0x00); // low byte
        cpu.memory.write_byte(0x0011, 0x90); // high 
    
        cpu.registers.y = 0x0A; // $9000 + $0A = $900A
    
        cpu.memory.write_byte(0x900A, 0x3C); // value to load
    
        let cycles = cpu.step();
        assert_eq!(cpu.registers.ac, 0x3C);
        assert_eq!(cycles, 5);
    }
    
    #[test]
    fn test_lda_indirect_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdaIndirectY as u8, 0x10 // LDA ($10),Y
        ]);

        cpu.memory.write_byte(0x0010, 0xFF);
        cpu.memory.write_byte(0x0011, 0x20); // base address = $20FF
    
        cpu.registers.y = 0x01; // $20FF + 1 = $2100
    
        cpu.memory.write_byte(0x2100, 0x77);
    
        let cycles = cpu.step();
        assert_eq!(cpu.registers.ac, 0x77);
        assert_eq!(cycles, 6);
    }
    
    
    
    
    
    
    #[test]
    fn test_adc_immediate() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AdcImmediate as u8, 0x55
        ]);
        
        cpu.registers.ac = 0x10;
        cpu.registers.sr.c = false;
        
        let cycles = cpu.step();
        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.ac, 0x65);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
    }
    
    #[test]
    fn test_adc_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AdcZeropage as u8, 0xAF
        ]);
        
        // write 0xAE to 0x00AF
        cpu.memory.write_byte(0x00AF, 0xAE); 
        
        // give an inital value to accumulator
        cpu.registers.ac = 0xC0;
        
        let cycles = cpu.step();
        
        assert_eq!(cycles, 3); // this instruction takes 3 cycles
        assert_eq!(cpu.registers.ac, 0x6E); // 0xC0 + 0xAE = 0x16E
        assert_eq!(cpu.registers.sr.v, true); // there was an overflow
        assert_eq!(cpu.registers.sr.c, true); // there is also a carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        assert_eq!(cpu.registers.sr.n, false); // not negative
    }
    
    #[test]
    fn test_adc_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AdcZeropageX as u8, 0x03
        ]);
        
        cpu.registers.x = 0x05;
        
        // write 0x0C to 0x0008
        cpu.memory.write_byte(0x0008, 0x0C); 
        
        // give an inital value to accumulator
        cpu.registers.ac = 0xA0;
        cpu.registers.sr.c = true;
        
        
        let cycles = cpu.step();
        assert_eq!(cycles, 4); // this instruction takes 4 cycles
        assert_eq!(cpu.registers.ac, 0xAD); // 0x0C + 0xA0 + 0x01 = 0xAD
        assert_eq!(cpu.registers.sr.v, false); // no overflow
        assert_eq!(cpu.registers.sr.c, false); // no carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        
        //0xAD = 	1010_1101
        assert_eq!(cpu.registers.sr.n, true); // negative 
    }
     
    #[test]
    fn test_adc_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AdcAbsolute as u8, 0xA4, 0x14
        ]);
        
        // write 0xB2 to 0x14A4
        cpu.memory.write_byte(0x14A4, 0xB2); 
        
        // give an inital value to accumulator
        cpu.registers.ac = 0x34;
        
        let cycles = cpu.step();
        assert_eq!(cycles, 4);
        
        assert_eq!(cpu.registers.ac, 0xE6); // 0xB2 + 0x34 + 0x00 = 0xE6
        
        //0xE6 = 		1110 0110
        assert_eq!(cpu.registers.sr.v, false); // no overflow
        assert_eq!(cpu.registers.sr.c, false); // no carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        assert_eq!(cpu.registers.sr.n, true); // negative 
    }
     
    #[test]
    fn test_adc_absolute_x_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AdcAbsoluteX as u8, 0x10, 0x20
        ]);
        
        cpu.registers.x = 0x0F;
        cpu.registers.ac = 0x30;
        cpu.registers.sr.c = false;
        
        // write 0x40 to 0x201F
        cpu.memory.write_byte(0x201F, 0x40);
        
        //0x2010 + 0x10 + 0x0F = 0x201F
        //0x30 + 0x40 = 0x70 = 0111 0000
        
        let cycles = cpu.step();
        
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.ac, 0x70);
        assert_eq!(cpu.registers.sr.v, false); // no overflow
        assert_eq!(cpu.registers.sr.c, false); // no carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        assert_eq!(cpu.registers.sr.n, false); // not negative 
    }
    
    #[test]
    fn test_adc_absolute_x_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AdcAbsoluteX as u8, 0x10, 0x20
        ]);
        
        cpu.registers.x = 0xF1;
        cpu.registers.ac = 0x52;

         // write 0x74 to 0x2101, This is a page jump as the high byte changed value from 20 to 21
        cpu.memory.write_byte(0x2101, 0x74);
        
        //0x74 + 0x52 = 0xC6 = 1100 0110
        let cycles = cpu.step();
        assert_eq!(cycles, 5);
        assert_eq!(cpu.registers.ac, 0xC6);
        assert_eq!(cpu.registers.sr.v, true); // overflow (went from positive to negative)
        assert_eq!(cpu.registers.sr.c, false); // no carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        assert_eq!(cpu.registers.sr.n, true); // negative 
    }
    
    #[test]
    fn test_adc_absolute_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AdcAbsoluteY as u8, 0x12, 0x14
        ]);
        
        cpu.registers.y = 0x05;
        cpu.registers.ac = 0x41;
        cpu.registers.sr.c = true;
        
        // write 0x32 to 0x1417
        cpu.memory.write_byte(0x1417, 0x32);
        
        //0x41 + 0x32 + 1 = 0x74 = 0111 0100
        
        let cycles = cpu.step();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.ac, 0x74);
        assert_eq!(cpu.registers.sr.v, false); // no overflow
        assert_eq!(cpu.registers.sr.c, false); // no carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        assert_eq!(cpu.registers.sr.n, false); // not negative 
    }
    
    #[test]
    fn test_adc_absolute_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AdcAbsoluteY as u8, 0x78, 0x14
        ]);
        
        cpu.registers.y = 0xFF;
        cpu.registers.ac = 0x04;
        
        // write 0xD1 to 0x1577
        cpu.memory.write_byte(0x1577, 0xD1);
        
        let cycles = cpu.step();
        assert_eq!(cycles, 5);
        assert_eq!(cpu.registers.ac, 0xD5);
        assert_eq!(cpu.registers.sr.v, false); // no overflow
        assert_eq!(cpu.registers.sr.c, false); // no carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        assert_eq!(cpu.registers.sr.n, true); // negative 
    }
    
    #[test]
    fn test_adc_indirect_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AdcIndirectX as u8, 0x35
        ]);
        
        cpu.registers.ac = 0x08;
        cpu.registers.x = 0x94;
        
        // 0x35 + 0x94 = C9
        //point to 0x00D1
        cpu.memory.write_byte(0x00C9, 0xD1);
        cpu.memory.write_byte(0x00CA, 0x00);
        
        //value at 0x00D1
        cpu.memory.write_byte(0x00D1, 0x67);
    
        //0x67 + 0x08 = 0x6F
        
        let cycles = cpu.step();
        
        assert_eq!(cycles, 6);
        assert_eq!(cpu.registers.ac, 0x6F);
        assert_eq!(cpu.registers.sr.v, false); // no overflow
        assert_eq!(cpu.registers.sr.c, false); // no carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        assert_eq!(cpu.registers.sr.n, false); //not negative 
    }
    
    #[test]
    fn test_adc_indirect_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AdcIndirectY as u8, 0x52
        ]);
        
        cpu.registers.ac = 0x43;
        cpu.registers.y = 0x12;
        
         // 0x52 + 0x12 = 64
        //point to 0x00A1
        cpu.memory.write_byte(0x0052, 0xA1);
        cpu.memory.write_byte(0x0053, 0x00);
        
        //value at 0x00A1 + 0x12 = 0x00B3
        cpu.memory.write_byte(0x00B3, 0x35);
        
        //0x35 + 0x43 = 0x78
        
        let cycles = cpu.step();
        
        assert_eq!(cycles, 5);
        assert_eq!(cpu.registers.ac, 0x78);
        assert_eq!(cpu.registers.sr.v, false); // no overflow
        assert_eq!(cpu.registers.sr.c, false); // no carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        assert_eq!(cpu.registers.sr.n, false); //not negative 
    }
    
    #[test]
    fn test_adc_indirect_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AdcIndirectY as u8, 0x52
        ]);
        
        cpu.registers.ac = 0x43;
        cpu.registers.y = 0xDE;
        
         // 0x52 + 0x12 = 64
        //point to 0x84A1
        cpu.memory.write_byte(0x0052, 0xA1);
        cpu.memory.write_byte(0x0053, 0x84);
        
        //value at 0x84A1 + 0xDE = 0x857F
        cpu.memory.write_byte(0x857F, 0x35);
        
        //0x35 + 0x43 = 0x78
        
        let cycles = cpu.step();
        
        assert_eq!(cycles, 6);
        assert_eq!(cpu.registers.ac, 0x78);
        assert_eq!(cpu.registers.sr.v, false); // no overflow
        assert_eq!(cpu.registers.sr.c, false); // no carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        assert_eq!(cpu.registers.sr.n, false); //not negative 
    }
    
    
    

    #[test]
    fn test_and_immediate() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AndImmediate as u8, 0xFF
        ]);
        cpu.registers.ac = 0xFE;
         
        let cycles = cpu.step();
        // 1111 1111
        // 1111 1110
        // 1111 1110 = FE
        
        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.ac, 0xFE);
        assert_eq!(cpu.registers.sr.v, false); // no overflow
        assert_eq!(cpu.registers.sr.c, false); // no carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        assert_eq!(cpu.registers.sr.n, true); //negative 
    }
     
    #[test]
    fn test_and_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AndZeropage as u8, 0x52
        ]);
        
        // write 0xAE to 0x0052
        cpu.memory.write_byte(0x0052, 0xAE); 
        
        // give an inital value to accumulator
        cpu.registers.ac = 0xC0;
        
        let cycles = cpu.step();
        
        //1010 1110
        //1100 0000 = 1000 0000 = 0x80
        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.ac, 0x80);
        assert_eq!(cpu.registers.sr.v, false); // no overflow
        assert_eq!(cpu.registers.sr.c, false); // no carry out
        assert_eq!(cpu.registers.sr.z, false); // not zero
        assert_eq!(cpu.registers.sr.n, true); //negative 
    }
    
    #[test]
    fn test_and_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AndZeropageX as u8
        ]);
        let cycles = cpu.step();
        assert_eq!(cycles, 4);
    }
    
    #[test]
    fn test_and_absolute() {
        let mut cpu = setup_cpu_with_program(&[
        Opcodes::AndZeropageX as u8, 0x10
        ]);
        
        cpu.registers.x = 0x05;
        cpu.registers.ac = 0x0F;
        
        // write 0xF0 to 0x0015 (0x10 + 0x05)
        cpu.memory.write_byte(0x0015, 0xF0);
        
        let cycles = cpu.step();
        
        // 1111 0000
        // 0000 1111 = 0000 0000 = 0x00
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.ac, 0x00);
        assert_eq!(cpu.registers.sr.z, true);  // zero
    }
    
    #[test]
    fn test_and_absolute_x_npc() {
        let mut cpu = setup_cpu_with_program(&[
        Opcodes::AndAbsoluteX as u8, 0x00, 0x20
        ]);
    
        cpu.registers.ac = 0xF0;  
        cpu.registers.x = 0x05;
    
        // 0x2000 + 0x05 = 0x2005
        cpu.memory.write_byte(0x2005, 0xAA);
    
        let cycles = cpu.step();
    
        // 0xF0 & 0xAA = 0xA0
        assert_eq!(cpu.registers.ac, 0xA0);
    
        assert_eq!(cpu.registers.sr.z, false); 
        assert_eq!(cpu.registers.sr.n, true); 
        assert_eq!(cpu.registers.sr.c, false);      
        assert_eq!(cpu.registers.sr.v, false);        
        assert_eq!(cycles, 4);
    }
    
    #[test]
    fn test_and_absolute_x_pc() {
         let mut cpu = setup_cpu_with_program(&[
        Opcodes::AndAbsoluteX as u8, 0xFF, 0x20  
        ]);
    
        cpu.registers.ac = 0xFF;  
        cpu.registers.x = 0x01;  
    
        // write 0x0F to 0x2100
        cpu.memory.write_byte(0x2100, 0x0F);
    
        let cycles = cpu.step();
    
        // 0xFF & 0x0F = 0x0F
        assert_eq!(cpu.registers.ac, 0x0F);
    
        assert_eq!(cpu.registers.sr.z, false);  
        assert_eq!(cpu.registers.sr.n, false);  
        assert_eq!(cpu.registers.sr.c, false);  
        assert_eq!(cpu.registers.sr.v, false);  
    
        assert_eq!(cycles, 5);
    }
    
    #[test]
    fn test_and_absolute_y_npc() {
         let mut cpu = setup_cpu_with_program(&[
        Opcodes::AndAbsoluteY as u8, 0x10, 0x20  // AND $2010,Y
        ]);
    
        cpu.registers.ac = 0xFF;  
        cpu.registers.y = 0x05;   
    
        // write 0x0F to memory at 0x2015
        cpu.memory.write_byte(0x2015, 0x0F);
    
        let cycles = cpu.step();
    
        // 0xFF & 0x0F = 0x0F
        assert_eq!(cpu.registers.ac, 0x0F);
    
        assert_eq!(cpu.registers.sr.z, false);  
        assert_eq!(cpu.registers.sr.n, false);  
        assert_eq!(cpu.registers.sr.c, false);  
        assert_eq!(cpu.registers.sr.v, false);
    
        assert_eq!(cycles, 4);
    }
    
    #[test]
    fn test_and_absolute_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AndAbsoluteY as u8, 0xFF, 0x20
        ]);
    
        cpu.registers.ac = 0xFF;
        cpu.registers.y = 0x01;
    
        // Write 0x0F to memory at 0x2100
        cpu.memory.write_byte(0x2100, 0x0F);
    
        let cycles = cpu.step();
    
        // 0xFF & 0x0F = 0x0F
        assert_eq!(cpu.registers.ac, 0x0F);
    
        assert_eq!(cpu.registers.sr.z, false);  
        assert_eq!(cpu.registers.sr.n, false); 
        assert_eq!(cpu.registers.sr.c, false); 
        assert_eq!(cpu.registers.sr.v, false);
    
        assert_eq!(cycles, 5);
    }
    
    #[test]
    fn test_and_indirect_x() {
        let mut cpu = setup_cpu_with_program(&[
        Opcodes::AndIndirectX as u8, 0x40
        ]);
    
        cpu.registers.ac = 0xFF;  
        cpu.registers.x = 0x04;  
    
        // 0x40 + 0x04 = $44 points to 0x1234
        cpu.memory.write_byte(0x0044, 0x34);
        cpu.memory.write_byte(0x0045, 0x12);
    

        cpu.memory.write_byte(0x1234, 0x0F);
    
        let cycles = cpu.step();
    
        // 0xFF & 0x0F = 0x0F
        assert_eq!(cpu.registers.ac, 0x0F);
    
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.v, false);
    
        assert_eq!(cycles, 6);
    }
    
    #[test]
    fn test_and_indirect_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AndIndirectY as u8, 0x40 
        ]);
    
        cpu.registers.ac = 0xFF;
        cpu.registers.y = 0x05;
    
        cpu.memory.write_byte(0x0040, 0x00);
        cpu.memory.write_byte(0x0041, 0x10);
    
        cpu.memory.write_byte(0x1005, 0x0F);
    
        let cycles = cpu.step();
    
        assert_eq!(cpu.registers.ac, 0x0F);
    
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.v, false);
    
        assert_eq!(cycles, 5);
    }
    
    #[test]
    fn test_and_indirect_y_pc() {
         let mut cpu = setup_cpu_with_program(&[
            Opcodes::AndIndirectY as u8, 0x40 
        ]);
    
        cpu.registers.ac = 0xFF; 
        cpu.registers.y = 0xFF;  
    
        cpu.memory.write_byte(0x0040, 0xFF);
        cpu.memory.write_byte(0x0041, 0x10);
    
    
        cpu.memory.write_byte(0x11FE, 0x0F);
    
        let cycles = cpu.step();
    
        assert_eq!(cpu.registers.ac, 0x0F);
    
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.v, false);
    
        assert_eq!(cycles, 6);
    }


    
    
    
    #[test]
    fn test_asl_accumulator() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AslAccumulator as u8
        ]);

        cpu.registers.ac = 0x41;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x82);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false); 
        assert_eq!(cpu.registers.sr.n, true);  
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_asl_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AslZeropage as u8, 0x10  // ASL $10
        ]);

        cpu.memory.write_byte(0x0010, 0x41);  

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x0010);

        assert_eq!(result, 0x82);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true); 
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_asl_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AslZeropageX as u8, 0x10 // ASL $10,X
        ]);

        cpu.registers.x = 0x05;

        cpu.memory.write_byte(0x0015, 0x41);

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x0015);


        assert_eq!(result, 0x82);
        assert_eq!(cpu.registers.sr.c, false); 
        assert_eq!(cpu.registers.sr.z, false); 
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 6);
    }

        
    #[test]
    fn test_asl_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AslAbsolute as u8, 0x34, 0x12  // ASL $1234
        ]);


        cpu.memory.write_byte(0x1234, 0x41);

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x1234);

        assert_eq!(result, 0x82);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_asl_absolute_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::AslAbsoluteX as u8, 0x00, 0x20  // ASL $2000,X
        ]);

        cpu.registers.x = 0x05;

        cpu.memory.write_byte(0x2005, 0x41);
        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x2005);

        assert_eq!(result, 0x82);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 7);
    }

    
    
    
    
    
    #[test]
    fn test_bcc_not_taken() {
        let mut cpu = setup_cpu_with_program(&[
        Opcodes::BccRelative as u8, 0x05
        ]);
    
        cpu.registers.sr.c = true;
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 2); 
        assert_eq!(cpu.registers.pc, 0x8002);
    }
    
    #[test]
    fn test_bcc_taken_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BccRelative as u8, 0x02,
            Opcodes::Nop as u8,
            Opcodes::BRK as u8
        ]);
    
        cpu.registers.sr.c = false;

        let cycles = cpu.step();
    
        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.pc, 0x8004); 
    }
    
    
    #[test]
    fn test_bcc_taken_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BccRelative as u8, 0x05,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::BRK as u8
        ]);
    
        cpu.registers.pc = 0x80FD;
        cpu.memory.write_byte(0x80FD, Opcodes::BccRelative as u8);
        cpu.memory.write_byte(0x80FE, 0x05);
        cpu.registers.sr.c = false;
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 4); 
        assert_eq!(cpu.registers.pc, 0x8104);
    }
    
    
    
    
    
    
    #[test]
    fn test_bcs_not_taken() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BcsRelative as u8, 0x05,   // BCS +5
            Opcodes::LdaImmediate as u8, 0x42 // LDA #$42
        ]);

        cpu.registers.sr.c = false;

        let cycles = cpu.step(); 

        
        assert_eq!(cpu.registers.pc, 0x8002);
        assert_eq!(cycles, 2);

        cpu.step(); // Execute LDA #$42
        assert_eq!(cpu.registers.ac, 0x42);
    }




    
    #[test]
    fn test_bcs_taken_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BcsRelative as u8, 0x05,   // BCS +5 
            0x00, 0x00, 0x00, 0x00, 0x00, 
            Opcodes::LdaImmediate as u8, 0x42 // LDA #$42
        ]);

        cpu.registers.sr.c = true; 
        let cycles = cpu.step(); 

        assert_eq!(cycles, 3);  
        assert_eq!(cpu.registers.pc, 0x8007); 

        cpu.step();
        assert_eq!(cpu.registers.ac, 0x42);
    }

    
    
    #[test]
    fn test_bcs_taken_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BcsRelative as u8, 0x05,         // BCS +5
            0x00, 0x00, 0x00, 0x00, 0x00,   
            Opcodes::BRK as u8
        ]);

        cpu.registers.sr.c = true;
        let cycles = cpu.step();

        assert_eq!(cpu.registers.pc, 0x8007);
        assert_eq!(cycles, 3);
    }

    
    
    
    
    
    #[test]
    fn test_beq_not_taken() {
            let mut cpu = setup_cpu_with_program(&[
            Opcodes::BeqRelative as u8, 0x05,  
            Opcodes::BRK as u8               
        ]);
    
        cpu.registers.sr.z = false;
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.pc, 0x8002);
    }
    
    #[test]
    fn test_beq_taken_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BeqRelative as u8, 0x02, 
            Opcodes::Nop as u8,                
            Opcodes::BRK as u8                  
        ]);
    
        cpu.registers.sr.z = true; 
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 3);
    
        assert_eq!(cpu.registers.pc, 0x8004);

    }
    
    
    #[test]
    fn test_beq_taken_pc() {
        let mut cpu = setup_cpu_with_program(&[
        Opcodes::BeqRelative as u8, 0x05,  
        Opcodes::Nop as u8,
        Opcodes::Nop as u8,
        Opcodes::Nop as u8,
        Opcodes::Nop as u8,
        Opcodes::BRK as u8
    ]);

 
    cpu.registers.pc = 0x80FD;
    cpu.memory.write_byte(0x80FD, Opcodes::BeqRelative as u8);
    cpu.memory.write_byte(0x80FE, 0x05);
    cpu.registers.sr.z = true; 

    let cycles = cpu.step();
    assert_eq!(cycles, 4);

 
    assert_eq!(cpu.registers.pc, 0x8104);
}
    
    
    
    
    #[test]
    fn test_bit_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BitZeropage as u8, 0x10,
            Opcodes::BRK as u8   
        ]);
    
        
        cpu.memory.write_byte(0x0010, 0xC1);
        cpu.registers.ac = 0x81;
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cpu.registers.sr.v, true);
    }
    
    #[test]
    fn test_bit_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BitAbsolute as u8, 0x00, 0x20,
            Opcodes::BRK as u8                 
        ]);
    

        cpu.memory.write_byte(0x2000, 0xC1);
        cpu.registers.ac = 0x81;
    
        let cycles = cpu.step(); 
    
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cpu.registers.sr.v, true);
    }
    
    
    
    
    #[test]
    fn test_bmi_not_taken() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BmiRelative as u8, 0x02, 
            Opcodes::Nop as u8,               
            Opcodes::BRK as u8                
        ]);
    
        cpu.registers.sr.n = false; 
    
        let cycles = cpu.step(); 
    
        assert_eq!(cycles, 2); 
        assert_eq!(cpu.registers.pc, 0x8002);
    }
    
    #[test]
    fn test_bmi_taken_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BmiRelative as u8, 0x02, 
            Opcodes::Nop as u8,                 
            Opcodes::BRK as u8                  
        ]);
    
        cpu.registers.sr.n = true;
    
        let cycles = cpu.step(); 
    
        assert_eq!(cycles, 3); 
        assert_eq!(cpu.registers.pc, 0x8004);
    }
    
    
    #[test]
    fn test_bmi_taken_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BmiRelative as u8, 0x05, 
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::BRK as u8
        ]);

        cpu.registers.pc = 0x80FD; 
        cpu.memory.write_byte(0x80FD, Opcodes::BmiRelative as u8);
        cpu.memory.write_byte(0x80FE, 0x05);

        cpu.registers.sr.n = true;

        let cycles = cpu.step();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.pc, 0x8104);
    }
    
    
    
    
    #[test]
    fn test_bne_not_taken() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BneRelative as u8, 0x06,  
            Opcodes::Nop as u8,          
            Opcodes::BRK as u8               
        ]);
    
        cpu.registers.sr.z = true; 
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.pc, 0x8002);
    }
    
    #[test]
    fn test_bne_taken_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BneRelative as u8, 0x02,  
            Opcodes::Nop as u8,                 
            Opcodes::BRK as u8                  
        ]);
    
        cpu.registers.sr.z = false; 
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 3);       
        assert_eq!(cpu.registers.pc, 0x8004);
    }
    
    
    #[test]
    fn test_bne_taken_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BneRelative as u8, 0x04,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::BRK as u8,
        ]);

        cpu.registers.pc = 0x80FD;
        cpu.memory.write_byte(0x80FD, Opcodes::BneRelative as u8);
        cpu.memory.write_byte(0x80FE, 0x04);

        cpu.registers.sr.z = false; 

        let cycles = cpu.step();

        assert_eq!(cycles, 4); 
        assert_eq!(cpu.registers.pc, 0x8103); 
    }
    
    
    
    
    
    #[test]
    fn test_bpl_not_taken() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BplRelative as u8, 0x05,
            Opcodes::BRK as u8
        ]);
    
        cpu.registers.pc = 0x8000;
        cpu.memory.write_byte(0x8000, Opcodes::BplRelative as u8);
        cpu.memory.write_byte(0x8001, 0x05);
    
        cpu.registers.sr.n = true;
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 2); 
        assert_eq!(cpu.registers.pc, 0x8002);
    }
    
    #[test]
    fn test_bpl_taken_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BplRelative as u8, 0x03,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::BRK as u8,
        ]);
    
        cpu.registers.pc = 0x8000;
        cpu.memory.write_byte(0x8000, Opcodes::BplRelative as u8);
        cpu.memory.write_byte(0x8001, 0x03);
    
        cpu.registers.sr.n = false;
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 3);    
        assert_eq!(cpu.registers.pc, 0x8005);
    }
    
    
    #[test]
    fn test_bpl_taken_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BplRelative as u8, 0x04,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::BRK as u8,
        ]);
    
        cpu.registers.pc = 0x80FD;
        cpu.memory.write_byte(0x80FD, Opcodes::BplRelative as u8);
        cpu.memory.write_byte(0x80FE, 0x04);
    
        cpu.registers.sr.n = false;
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.pc, 0x8103);
    }
    
    
    
    #[test]
    fn test_brk() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BRK as u8
        ]);
        let cycles = cpu.step();
        assert_eq!(cycles, -1);
        assert!(cpu.registers.sr.i);
        assert_eq!(cpu.cycle_count, 7);
    }
    
    
    
    
    
    
    #[test]
    fn test_bvc_not_taken() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BvcRelative as u8, 0x05,           // BVC +5
            Opcodes::LdaImmediate as u8, 0x42   // LDA #$42 (
        ]);

        cpu.registers.sr.v = true; 

        let cycles = cpu.step();

        assert_eq!(cpu.registers.pc, 0x8002); 
        assert_eq!(cycles, 2);

        cpu.step();
        assert_eq!(cpu.registers.ac, 0x42);
    }

    
    #[test]
    fn test_bvc_taken_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BvcRelative as u8, 0x05,         // BVC +5
            0x00, 0x00, 0x00, 0x00, 0x00,
            Opcodes::LdaImmediate as u8, 0x42 
        ]);

        cpu.registers.sr.v = false;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.pc, 0x8007);
        assert_eq!(cycles, 3);

        cpu.step(); // execute LDA #$42
        assert_eq!(cpu.registers.ac, 0x42);
    }

    

    #[test]
    fn test_bvc_taken_pc_page_cross() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BvcRelative as u8, 0x04,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::BRK as u8,
        ]);
    
        cpu.registers.pc = 0x80FD;
        cpu.memory.write_byte(0x80FD, Opcodes::BplRelative as u8);
        cpu.memory.write_byte(0x80FE, 0x04);
    
        cpu.registers.sr.v = false;
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.pc, 0x8103);
    }




    
    
    
    
    
    #[test]
    fn test_bvs_not_taken() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BvsRelative as u8, 0x05,           // BVC +5
            Opcodes::LdaImmediate as u8, 0x42   // LDA #$42 (
        ]);

        cpu.registers.sr.v = false; 

        let cycles = cpu.step();

        assert_eq!(cpu.registers.pc, 0x8002); 
        assert_eq!(cycles, 2);

        cpu.step();
        assert_eq!(cpu.registers.ac, 0x42);
    }
    
    #[test]
    fn test_bvs_taken_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BvsRelative as u8, 0x05,         // BVC +5
            0x00, 0x00, 0x00, 0x00, 0x00,
            Opcodes::LdaImmediate as u8, 0x42 
        ]);

        cpu.registers.sr.v = true;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.pc, 0x8007);
        assert_eq!(cycles, 3);

        cpu.step(); // execute LDA #$42
        assert_eq!(cpu.registers.ac, 0x42);
    }
    
    
    #[test]
    fn test_bvs_taken_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::BvsRelative as u8, 0x04,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::Nop as u8,
            Opcodes::BRK as u8,
        ]);
    
        cpu.registers.pc = 0x80FD;
        cpu.memory.write_byte(0x80FD, Opcodes::BplRelative as u8);
        cpu.memory.write_byte(0x80FE, 0x04);
    
        cpu.registers.sr.v = true;
    
        let cycles = cpu.step();
    
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.pc, 0x8103);
    }
    
    

    #[test]
    fn test_clc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Clc as u8
        ]);
        cpu.registers.sr.c = true;
        let cycles = cpu.step();
        assert_eq!(cycles, 2);
        assert!(!cpu.registers.sr.c);
    }
    
    #[test]
    fn test_cld() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Cld as u8
        ]);
        cpu.registers.sr.d = true;
        let cycles = cpu.step();
        assert_eq!(cycles, 2);
        assert!(!cpu.registers.sr.d);
    }
    
    #[test]
    fn test_cli() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Cli as u8
        ]);
        cpu.registers.sr.i = true;
        let cycles = cpu.step();
        assert_eq!(cycles, 2);
        assert!(!cpu.registers.sr.i);
    }
    
    #[test]
    fn test_clv() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Clv as u8
        ]);
        cpu.registers.sr.v = true;
        let cycles = cpu.step();
        assert_eq!(cycles, 2);
        assert!(!cpu.registers.sr.v);
    }
    
    
    #[test]
    fn test_cmp_immediate() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CmpImmediate as u8, 0x42
        ]);

        cpu.registers.ac = 0x50;
        let cycles = cpu.step();

        

        assert_eq!(cpu.registers.sr.z, false); 
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_cmp_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CmpZeropage as u8, 0x42 // CMP $42
        ]);

        cpu.registers.ac = 0x50;
        cpu.memory.write_byte(0x0042, 0x30); 

        let cycles = cpu.step();


        assert_eq!(cpu.registers.sr.c, true);   
        assert_eq!(cpu.registers.sr.z, false);  
        assert_eq!(cpu.registers.sr.n, false); 
        assert_eq!(cycles, 3);
    }

    
    #[test]
    fn test_cmp_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CmpZeropageX as u8, 0x10 // CMP $10,X
        ]);

        cpu.registers.ac = 0x80; 
        cpu.registers.x = 0x05;  


        cpu.memory.write_byte(0x0015, 0x80);

        let cycles = cpu.step();


        assert_eq!(cpu.registers.sr.c, true);   
        assert_eq!(cpu.registers.sr.z, true);   
        assert_eq!(cpu.registers.sr.n, false);  
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_cmp_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CmpAbsolute as u8, 0x34, 0x12  // CMP $1234
        ]);

        cpu.registers.ac = 0x50;
        cpu.memory.write_byte(0x1234, 0x40);
        let cycles = cpu.step();


        assert_eq!(cpu.registers.sr.c, true);  
        assert_eq!(cpu.registers.sr.z, false);  
        assert_eq!(cpu.registers.sr.n, false); 
        assert_eq!(cycles, 4);        
    }

    
    #[test]
    fn test_cmp_absolute_x_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CmpAbsoluteX as u8, 0x00, 0x20 // CMP $2000,X
        ]);

        cpu.registers.ac = 0x90;  
        cpu.registers.x = 0x10;  

        cpu.memory.write_byte(0x2010, 0x80);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, true);  
        assert_eq!(cpu.registers.sr.z, false);  
        assert_eq!(cpu.registers.sr.n, false);  
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_cmp_absolute_x_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CmpAbsoluteX as u8, 0xF0, 0x20 // CMP $20F0,X
        ]);

        cpu.registers.ac = 0xAA;
        cpu.registers.x = 0x20;

        cpu.memory.write_byte(0x2110, 0x90);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, true); 
        assert_eq!(cpu.registers.sr.z, false); 
        assert_eq!(cpu.registers.sr.n, false);  
        assert_eq!(cycles, 5);      
    }

    
    #[test]
    fn test_cmp_absolute_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CmpAbsoluteY as u8, 0x00, 0x30 // CMP $3000,Y
        ]);

        cpu.registers.ac = 0x70;
        cpu.registers.y = 0x10;

        cpu.memory.write_byte(0x3010, 0x60);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_cmp_absolute_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CmpAbsoluteY as u8, 0xF0, 0x30 // CMP $30F0,Y
        ]);

        cpu.registers.ac = 0xA0;
        cpu.registers.y = 0x20;

        cpu.memory.write_byte(0x3110, 0x90);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_cmp_indirect_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CmpIndirectX as u8, 0x10 // CMP ($10,X)
        ]);

        cpu.registers.ac = 0x70; 
        cpu.registers.x = 0x04;   


        cpu.memory.write_byte(0x0014, 0x00);
        cpu.memory.write_byte(0x0015, 0x40);

        cpu.memory.write_byte(0x4000, 0x60);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_cmp_indirect_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CmpIndirectY as u8, 0x20 // CMP ($20),Y
        ]);

        cpu.registers.ac = 0x99;
        cpu.registers.y = 0x10;

        cpu.memory.write_byte(0x0020, 0x00);
        cpu.memory.write_byte(0x0021, 0x30);

        cpu.memory.write_byte(0x3010, 0x80);

        let cycles = cpu.step();


        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_cmp_indirect_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CmpIndirectY as u8, 0x20 // CMP ($20),Y
        ]);

        cpu.registers.ac = 0x55;
        cpu.registers.y = 0x20;

        cpu.memory.write_byte(0x0020, 0xF0);
        cpu.memory.write_byte(0x0021, 0x30);

        cpu.memory.write_byte(0x3110, 0x60);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 6);  
    }

    
    
    
    
    #[test]
    fn test_cpx_immediate() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CpxImmediate as u8, 0x10 // CPX #$10
        ]);

        cpu.registers.x = 0x12;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_cpx_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CpxZeropage as u8, 0x40 // CPX $40
        ]);

        cpu.registers.x = 0x50;
        cpu.memory.write_byte(0x0040, 0x45);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 3);
    }

    
    #[test]
    fn test_cpx_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CpxAbsolute as u8, 0x00, 0x20 // CPX $2000
        ]);

        cpu.registers.x = 0x30;
        cpu.memory.write_byte(0x2000, 0x25);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 4);
    }

    
    
    
    
    
    #[test]
    fn test_cpy_immediate() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CpyImmediate as u8, 0x20 // CPY #$20
        ]);

        cpu.registers.y = 0x25;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_cpy_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CpyZeropage as u8, 0x30 // CPY $30
        ]);

        cpu.registers.y = 0x40;
        cpu.memory.write_byte(0x0030, 0x35);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 3);
    }

    
    #[test]
    fn test_cpy_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::CpyAbsolute as u8, 0x00, 0x20 // CPY $2000 (little endian)
        ]);

        cpu.registers.y = 0x50;
        cpu.memory.write_byte(0x2000, 0x45);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 4);
    }

    
    
    
    
    
    
    #[test]
    fn test_dec_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::DecZeropage as u8, 0x40 // DEC $40
        ]);

        cpu.memory.write_byte(0x0040, 0x03); 

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x0040);
        assert_eq!(result, 0x02);

        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);

        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_dec_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::DecZeropageX as u8, 0x20 // DEC $20,X
        ]);

        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x0025, 0x04);

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x0025);
        assert_eq!(result, 0x03);

        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);

        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_dec_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::DecAbsolute as u8, 0x00, 0x20 // DEC $2000
        ]);

        cpu.memory.write_byte(0x2000, 0x10);

        let cycles = cpu.step();


        let result = cpu.memory.read_byte(0x2000);
        assert_eq!(result, 0x0F);

        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);

        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_dec_absolute_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::DecAbsoluteX as u8, 0x10, 0x20 // DEC $2010,X
        ]);

        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x2015, 0x20);

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x2015);
        assert_eq!(result, 0x1F);

        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);

        assert_eq!(cycles, 7);
    }

    
    
    #[test]
    fn test_dex() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Dex as u8
        ]);

        cpu.registers.x = 0x01;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.x, 0x00);
        assert_eq!(cpu.registers.sr.z, true);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_dey() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Dey as u8 // DEY opcode
        ]);

        cpu.registers.y = 0x01;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.y, 0x00);
        assert_eq!(cpu.registers.sr.z, true);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }

    
    
    
    
    #[test]
    fn test_eor_immediate() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::EorImmediate as u8, 0x0F // EOR #$0F
        ]);

        cpu.registers.ac = 0xF0;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);  
        assert_eq!(cpu.registers.sr.n, true);  
        assert_eq!(cycles, 2); 
    }

    
    #[test]
    fn test_eor_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::EorZeropage as u8, 0x10 // EOR $10
        ]);

        cpu.registers.ac = 0xAA;        
        cpu.memory.write_byte(0x0010, 0xCC);

        let cycles = cpu.step();

   
        assert_eq!(cpu.registers.ac, 0x66);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 3);
    }

    
    #[test]
    fn test_eor_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::EorZeropageX as u8, 0x10 // EOR $10,X
        ]);

        cpu.registers.ac = 0xF0;
        cpu.registers.x = 0x05;

        cpu.memory.write_byte(0x0015, 0x0F);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_eor_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::EorAbsolute as u8, 0x00, 0x20 // EOR $2000
        ]);

        cpu.registers.ac = 0xC3;
        cpu.memory.write_byte(0x2000, 0xAA);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x69);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_eor_absolute_x_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::EorAbsoluteX as u8, 0x00, 0x20 // EOR $2000,X
        ]);

        cpu.registers.ac = 0xAA;
        cpu.registers.x = 0x05;

        cpu.memory.write_byte(0x2005, 0xCC);

        let cycles = cpu.step();

        // 0xAA ^ 0xCC = 0x66
        assert_eq!(cpu.registers.ac, 0x66);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_eor_absolute_x_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::EorAbsoluteX as u8, 0xFF, 0x20 // EOR $20FF,X
        ]);

        cpu.registers.ac = 0xF0;
        cpu.registers.x = 0x01;

        cpu.memory.write_byte(0x2100, 0x0F); 

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_eor_absolute_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::EorAbsoluteY as u8, 0x00, 0x30 // EOR $3000,Y
        ]);

        cpu.registers.ac = 0xAA;
        cpu.registers.y = 0x0F;

        cpu.memory.write_byte(0x300F, 0xCC);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x66);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_eor_absolute_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::EorAbsoluteY as u8, 0xFF, 0x30 // EOR $30FF,Y
        ]);

        cpu.registers.ac = 0xF0;
        cpu.registers.y = 0x01; 

        cpu.memory.write_byte(0x3100, 0x0F);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_eor_indirect_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::EorIndirectX as u8, 0x10 // EOR ($10,X)
        ]);

        cpu.registers.ac = 0xF0;
        cpu.registers.x = 0x04;

        cpu.memory.write_byte(0x0014, 0x00);
        cpu.memory.write_byte(0x0015, 0x30);

        cpu.memory.write_byte(0x3000, 0x0F);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_eor_indirect_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::EorIndirectY as u8, 0x10 // EOR ($10),Y
        ]);

        cpu.registers.ac = 0xAA;
        cpu.registers.y = 0x05;

        cpu.memory.write_byte(0x0010, 0x00);
        cpu.memory.write_byte(0x0011, 0x30);

        cpu.memory.write_byte(0x3005, 0xCC);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x66);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_eor_indirect_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::EorIndirectY as u8, 0x10 // EOR ($10),Y
        ]);

        cpu.registers.ac = 0xAA;
        cpu.registers.y = 0xFF;

        cpu.memory.write_byte(0x0010, 0xFF);
        cpu.memory.write_byte(0x0011, 0x30);

        cpu.memory.write_byte(0x31FE, 0xCC); 

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x66);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 6);
    }

    
    
    
    
    #[test]
    fn test_inc_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::IncZeropage as u8, 0x10 // INC $10
        ]);

        cpu.memory.write_byte(0x0010, 0x42);

        let cycles = cpu.step();

        assert_eq!(cpu.memory.read_byte(0x0010), 0x43);        
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_inc_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::IncZeropageX as u8, 0x10 // INC $10,X
        ]);


        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x0015, 0x7F);

        let cycles = cpu.step();

        assert_eq!(cpu.memory.read_byte(0x0015), 0x80);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_inc_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::IncAbsolute as u8, 0x00, 0x20  // INC $2000
        ]);

        cpu.memory.write_byte(0x2000, 0xFF);

        let cycles = cpu.step();

        assert_eq!(cpu.memory.read_byte(0x2000), 0x00);
        assert_eq!(cpu.registers.sr.z, true);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_inc_absolute_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::IncAbsoluteX as u8, 0x00, 0x20  // INC $2000,X
        ]);

        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x2005, 0x7F);

        let cycles = cpu.step();

        assert_eq!(cpu.memory.read_byte(0x2005), 0x80);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cycles, 7);
    }

    
    
    

    #[test]
    fn test_inx() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Inx as u8,
            Opcodes::BRK as u8,
        ]);
        cpu.registers.x = 0x7F;
        
        let cycles = cpu.step();
        
        assert_eq!(cpu.registers.x, 0x80);  
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_iny() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Iny as u8,
            Opcodes::BRK as u8,
        ]);
        cpu.registers.y = 0xFF;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.y, 0x00);
        assert_eq!(cpu.registers.sr.z, true);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }

    
    
    
    #[test]
    fn test_jmp_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::JmpAbsolute as u8, 0x34, 0x12,
        ]);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.pc, 0x1234);
        assert_eq!(cycles, 3);
    }

    
    
    #[test]
    fn test_jmp_indirect() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::JmpIndirect as u8, 0x00, 0x10, // JMP ($1000)
        ]);

        cpu.memory.write_byte(0x1000, 0x34);
        cpu.memory.write_byte(0x1001, 0x12);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.pc, 0x1234);
        assert_eq!(cycles, 5);
    }

    
    
    
    
    
    
    #[test]
    fn test_jsr() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::JsrAbsolute as u8, 0x34, 0x12, // JSR $1234
            Opcodes::BRK as u8,
        ]);

        let starting_sp = cpu.registers.sp;
        let cycles = cpu.step();

        let return_hi = cpu.memory.read_byte(0x0100 + ((starting_sp - 0) as u16) as u32);
        let return_lo = cpu.memory.read_byte(0x0100 + ((starting_sp - 1) as u16) as u32);
        let return_address = (return_hi as u16) << 8 | return_lo as u16;

        assert_eq!(return_address, 0x8002);
        assert_eq!(cpu.registers.pc, 0x1234);
        assert_eq!(cpu.registers.sp, starting_sp.wrapping_sub(2));
        assert_eq!(cycles, 6);
    }
    
    
    
    #[test]
    fn test_ldx_immediate() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdxImmediate as u8, 0x42, // LDX #$42
        ]);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.x, 0x42);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_ldx_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdxZeropage as u8, 0x10, // LDX $10
        ]);

        cpu.memory.write_byte(0x0010, 0x37);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.x, 0x37);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 3);
    }

    
    #[test]
    fn test_ldx_zeropage_y() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdxZeropageY as u8, 0x10, // LDX $10,Y
        ]);

        cpu.registers.y = 0x05;
        cpu.memory.write_byte(0x0015, 0x99);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.x, 0x99);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_ldx_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdxAbsolute as u8, 0x00, 0x90, // LDX $8000
        ]);

        cpu.memory.write_byte(0x9000, 0x42);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.x, 0x42);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_ldx_absolute_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdxAbsoluteY as u8, 0xFF, 0x00, // LDX $00FF,Y
        ]);

        cpu.registers.y = 0x01;

        cpu.memory.write_byte(0x0100, 0x37);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.x, 0x37);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 5);
    }


    #[test]
    fn test_ldx_absolute_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdxAbsoluteY as u8, 0x10, 0x00, // LDX $0010,Y
        ]);

        cpu.registers.y = 0x0F;

        cpu.memory.write_byte(0x001F, 0x42);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.x, 0x42);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 4);
    }

    
    

    #[test]
    fn test_ldy_immediate() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdyImmediate as u8, 0x42,  // LDY #$42
            Opcodes::BRK as u8,
        ]);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.y, 0x42);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_ldy_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdyZeropage as u8, 0x10, // LDY $10
            Opcodes::BRK as u8,
        ]);

        cpu.memory.write_byte(0x0010, 0x42);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.y, 0x42);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 3);
    }

    
    #[test]
    fn test_ldy_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdyZeropageX as u8, 0x10, // LDY $10,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x0015, 0x37);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.y, 0x37);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_ldy_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdyAbsolute as u8, 0x55, 0x80, // LDY $8055
            Opcodes::BRK as u8,
        ]);

        cpu.memory.write_byte(0x8055, 0x55);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.y, 0x55);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_ldy_absolute_x_npc() {
        
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdyAbsoluteX as u8, 0x55, 0x80, // LDY $8055,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x01;
        cpu.memory.write_byte(0x8056, 0x42);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.y, 0x42);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_ldy_absolute_x_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LdyAbsoluteX as u8, 0xFF, 0x00, // LDX $00FF,Y
        ]);

        cpu.registers.x = 0x01;

        cpu.memory.write_byte(0x0100, 0x37);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.y, 0x37);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 5);
    } 
    
    
    
    
    
    #[test]
    fn test_lsr_accumulator() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LsrAccumulator as u8, // LSR A
            Opcodes::BRK as u8,           
        ]);

        cpu.registers.ac = 0x03;
        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x01);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }
    
    #[test]
    fn test_lsr_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LsrZeropage as u8, 0x10, // LSR $10
            Opcodes::BRK as u8,
        ]);

        cpu.memory.write_byte(0x0010, 0x02);
        let cycles = cpu.step();

        assert_eq!(cpu.memory.read_byte(0x0010), 0x01);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_lsr_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LsrZeropageX as u8, 0x10, // LSR $10,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x0015, 0x04);
        let cycles = cpu.step();

        assert_eq!(cpu.memory.read_byte(0x0015), 0x02);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_lsr_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LsrAbsolute as u8, 0x00, 0x20, // LSR $2000
            Opcodes::BRK as u8,
        ]);

        cpu.memory.write_byte(0x2000, 0x08);
        let cycles = cpu.step();

        assert_eq!(cpu.memory.read_byte(0x2000), 0x04);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_lsr_absolute_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::LsrAbsoluteX as u8, 0x00, 0x20, // LSR $2000,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x2005, 0x02);
        let cycles = cpu.step();

        assert_eq!(cpu.memory.read_byte(0x2005), 0x01);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 7);
    }

    
    
    #[test]
    fn test_nop() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Nop as u8
        ]);
        cpu.registers.x = 0x0A;
        cpu.registers.y = 0x0B;
        cpu.registers.ac = 0x0C;
        
        cpu.registers.sr.v = false;
        cpu.registers.sr.c = true;
        cpu.registers.sr.z = false;
        cpu.registers.sr.n = true;
        cpu.registers.sr.d = false;
        cpu.registers.sr.i = true;
        
        let cycles = cpu.step();
        assert_eq!(cycles, 2);
        
        assert_eq!(cpu.registers.x, 0x0A);
        assert_eq!(cpu.registers.y, 0x0B);
        assert_eq!(cpu.registers.ac, 0x0C);
        assert_eq!(cpu.registers.sr.v, false); 
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false); 
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cpu.registers.sr.d, false);
        assert_eq!(cpu.registers.sr.i, true);  
    }
    
    
 
    
    #[test]
    fn test_ora_immediate() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::OraImmediate as u8, 0x0F, // ORA #$0F
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0xF0;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_ora_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::OraZeropage as u8, 0x10, // ORA $10
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x0F;
        cpu.memory.write_byte(0x0010, 0xF0);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 3);
    }

    
   #[test]
    fn test_ora_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::OraZeropageX as u8, 0x10, // ORA $10,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x0F;
        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x0015, 0xF0);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_ora_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::OraAbsolute as u8, 0x00, 0x20, // ORA $2000
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x0F;
        cpu.memory.write_byte(0x2000, 0xF0);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_ora_absolute_x_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::OraAbsoluteX as u8, 0x00, 0x20, // ORA $2000,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x0F;
        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x2005, 0xF0);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_ora_absolute_x_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::OraAbsoluteX as u8, 0xFF, 0x20, // ORA $20FF,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x0F;
        cpu.registers.x = 0x01;
        cpu.memory.write_byte(0x2100, 0xF0);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_ora_absolute_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::OraAbsoluteY as u8, 0x00, 0x20, // ORA $2000,Y
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x0F;
        cpu.registers.y = 0x05;
        cpu.memory.write_byte(0x2005, 0xF0);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_ora_absolute_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::OraAbsoluteY as u8, 0xFF, 0x20, // ORA $20FF,Y
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x0F;
        cpu.registers.y = 0x01;
        cpu.memory.write_byte(0x2100, 0xF0);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_ora_indirect_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::OraIndirectX as u8, 0x10, // ORA ($10,X)
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x0F;
        cpu.registers.x = 0x04;

        cpu.memory.write_byte(0x0014, 0x00);
        cpu.memory.write_byte(0x0015, 0x30);

        cpu.memory.write_byte(0x3000, 0xF0);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_ora_indirect_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::OraIndirectY as u8, 0x10, // ORA ($10),Y
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x0F;
        cpu.registers.y = 0x05;


        cpu.memory.write_byte(0x0010, 0x00);
        cpu.memory.write_byte(0x0011, 0x20);

        cpu.memory.write_byte(0x2005, 0xF0);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_ora_indirect_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::OraIndirectY as u8, 0x10, // ORA ($10),Y
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x0F;
        cpu.registers.y = 0xFF;

        cpu.memory.write_byte(0x0010, 0xFF);
        cpu.memory.write_byte(0x0011, 0x20);

        cpu.memory.write_byte(0x21FE, 0xF0);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xFF);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 6);
    }

    
    
    
    
    #[test]
    fn test_pha() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Pha as u8,  // PHA
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0xAB;
        let sp_before = cpu.registers.sp;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sp, sp_before.wrapping_sub(1));

        let stack_addr = 0x0100 + (cpu.registers.sp.wrapping_add(1) as u16);
        assert_eq!(cpu.memory.read_byte(stack_addr.into()), 0xAB);

        assert_eq!(cycles, 3);
    }

    
   #[test]
    fn test_php() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Php as u8,  // PHP
            Opcodes::BRK as u8,
        ]);

        cpu.registers.sr.c = true;
        cpu.registers.sr.z = false;
        cpu.registers.sr.i = true;
        cpu.registers.sr.d = false;
        cpu.registers.sr.b = true;
        cpu.registers.sr.v = true;
        cpu.registers.sr.n = false;

        let sp_before = cpu.registers.sp;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sp, sp_before.wrapping_sub(1));

        let stack_addr = 0x0100 + (cpu.registers.sp.wrapping_add(1) as u16);
        let pushed_value = cpu.memory.read_byte(stack_addr as u32);


        let mut expected = 0u8;
        expected |= (cpu.registers.sr.n as u8) << 7;
        expected |= (cpu.registers.sr.v as u8) << 6;
        expected |= 1 << 5;
        expected |= 1 << 4;
        expected |= (cpu.registers.sr.d as u8) << 3;
        expected |= (cpu.registers.sr.i as u8) << 2;
        expected |= (cpu.registers.sr.z as u8) << 1;
        expected |= (cpu.registers.sr.c as u8) << 0;

        assert_eq!(pushed_value, expected);
        assert_eq!(cycles, 3);
    }

    
    #[test]
    fn test_pla() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Pla as u8,  // PLA
            Opcodes::BRK as u8,
        ]);

        cpu.registers.sp = 0xFD;

        let stack_addr = 0x0100 + (cpu.registers.sp.wrapping_add(1) as u16);
        let value_to_pull = 0x42;
        cpu.memory.write_byte(stack_addr.into(), value_to_pull);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, value_to_pull);
        assert_eq!(cpu.registers.sp, 0xFE);
        assert_eq!(cpu.registers.sr.z, value_to_pull == 0);
        assert_eq!(cpu.registers.sr.n, (value_to_pull & 0x80) != 0);
        assert_eq!(cycles, 4);
    }


    
    #[test]
    fn test_plp() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Plp as u8,  // PLP
            Opcodes::BRK as u8,
        ]);

        let sp_before = cpu.registers.sp;
        let status_byte = 0xB5;
  
        let stack_addr = 0x0100 + (sp_before as u16);
        cpu.memory.write_byte(stack_addr as u32, status_byte);

        cpu.registers.sp = sp_before.wrapping_sub(1);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sp, sp_before);
        assert_eq!(cpu.registers.sr.n, (status_byte & 0x80) != 0);
        assert_eq!(cpu.registers.sr.v, (status_byte & 0x40) != 0);
        assert_eq!(cpu.registers.sr.b, (status_byte & 0x10) != 0);
        assert_eq!(cpu.registers.sr.d, (status_byte & 0x08) != 0);
        assert_eq!(cpu.registers.sr.i, (status_byte & 0x04) != 0);
        assert_eq!(cpu.registers.sr.z, (status_byte & 0x02) != 0);
        assert_eq!(cpu.registers.sr.c, (status_byte & 0x01) != 0);
        assert_eq!(cycles, 4);
    }

    
    
    
    
    #[test]
    fn test_rol_accumulator() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::RolAccumulator as u8, // ROL A
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x5A;
        cpu.registers.sr.c = false;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0xB4);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_rol_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::RolZeropage as u8, 0x10, // ROL $10          
            Opcodes::BRK as u8,
        ]);

        cpu.memory.write_byte(0x0010, 0x5A);
        cpu.registers.sr.c = false;

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x0010);

        assert_eq!(result, 0xB4);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_rol_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::RolZeropageX as u8, 0x10, // ROL $10,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x0015, 0xC3);
        cpu.registers.sr.c = true;
        
        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x0015);

        assert_eq!(result, 0x87);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_rol_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::RolAbsolute as u8, 0x34, 0x12, // ROL $1234
            Opcodes::BRK as u8,
        ]);

        cpu.memory.write_byte(0x1234, 0x6A);
        cpu.registers.sr.c = true;

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x1234);

        assert_eq!(result, 0xD5);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_rol_absolute_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::RolAbsoluteX as u8, 0x00, 0x20,// ROL $2000,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x2005, 0xCA);
        cpu.registers.sr.c = true;

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x2005);

        assert_eq!(result, 0x95);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 7);
    }

    
    
    
    
    #[test]
    fn test_ror_accumulator() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::RorAccumulator as u8, // ROR A
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x03;
        cpu.registers.sr.c = true;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x81);
        assert_eq!(cpu.registers.sr.c, (0x03 & 0x01) != 0);
        assert_eq!(cpu.registers.sr.z, cpu.registers.ac == 0);
        assert_eq!(cpu.registers.sr.n, (cpu.registers.ac & 0x80) != 0);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_ror_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::RorZeropage as u8, 0x10, // ROR $10
            Opcodes::BRK as u8,
        ]);

        cpu.memory.write_byte(0x0010, 0x03); 
        cpu.registers.sr.c = true;

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x0010);

        assert_eq!(result, 0x81);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_ror_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::RorZeropageX as u8, 0x10, // ROR $10,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x0015, 0x02);
        cpu.registers.sr.c = true;

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x0015);

        assert_eq!(result, 0x81);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_ror_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::RorAbsolute as u8, 0x00, 0x20, // ROR $2000
            Opcodes::BRK as u8,
        ]);

        cpu.memory.write_byte(0x2000, 0x02);
        cpu.registers.sr.c = true;

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x2000);

        assert_eq!(result, 0x81);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_ror_absolute_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::RorAbsoluteX as u8, 0x00, 0x20, // ROR $2000,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x05;
        cpu.memory.write_byte(0x2005, 0x02);
        cpu.registers.sr.c = true;

        let cycles = cpu.step();

        let result = cpu.memory.read_byte(0x2005);

        assert_eq!(result, 0x81);
        assert_eq!(cpu.registers.sr.c, false);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 7);
    }

    
    
    
    
    #[test]
    fn test_rti() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Rti as u8,
            Opcodes::BRK as u8,
        ]);


        cpu.registers.sp = 0xFA;


        cpu.memory.write_byte(0x0100 + 0xFB, 0xFF);
        cpu.memory.write_byte(0x0100 + 0xFC, 0x34);
        cpu.memory.write_byte(0x0100 + 0xFD, 0x12);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.pc, 0x1234);
        assert_eq!(cycles, 6);
    }

    
    #[test]
    fn test_rts() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Rts as u8,
            Opcodes::BRK as u8,
        ]);

        cpu.registers.sp = 0xFC;

        cpu.memory.write_byte(0x0100 + 0xFD, 0x34);
        cpu.memory.write_byte(0x0100 + 0xFE, 0x12);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.pc, 0x1235);
        assert_eq!(cycles, 6);
    }

    
    
    
    
    
    
    #[test]
    fn test_sbc_immediate() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::SbcImmediate as u8, 0x10, // SBC #$10
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x20;
        cpu.registers.sr.c = true;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x10);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 2);
    }

    
    #[test]
    fn test_sbc_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::SbcZeropage as u8, 0x10, // SBC $10
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x50;
        cpu.registers.sr.c = true;
        cpu.memory.write_byte(0x0010, 0x20);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x30);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 3);
    }

    
    #[test]
    fn test_sbc_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::SbcZeropageX as u8, 0x0A, // SBC $0A,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x60;
        cpu.registers.x = 0x05;
        cpu.registers.sr.c = true;
        cpu.memory.write_byte(0x000F, 0x20);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x40);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_sbc_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::SbcAbsolute as u8, 0x00, 0x20, // SBC $2000
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x50;
        cpu.registers.sr.c = true;
        cpu.memory.write_byte(0x2000, 0x10);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x40);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_sbc_absolute_x_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::SbcAbsoluteX as u8, 0x00, 0x20, // SBC $2000,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x50;
        cpu.registers.x = 0x05;
        cpu.registers.sr.c = true;
        cpu.memory.write_byte(0x2005, 0x10);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x40);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_sbc_absolute_x_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::SbcAbsoluteX as u8, 0xFF, 0x20, // SBC $20FF,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x50;
        cpu.registers.x = 0x02;
        cpu.registers.sr.c = true;
        cpu.memory.write_byte(0x2101, 0x10);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x40);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_sbc_absolute_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::SbcAbsoluteY as u8, 0x10, 0x20, // SBC $2010,Y
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x50;
        cpu.registers.y = 0x05;
        cpu.registers.sr.c = true;
        cpu.memory.write_byte(0x2015, 0x20);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x30);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_sbc_absolute_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::SbcAbsoluteY as u8, 0xFF, 0x20, // SBC $20FF,Y
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x50;
        cpu.registers.y = 0x02;
        cpu.registers.sr.c = true;
        cpu.memory.write_byte(0x2101, 0x20);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x30);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_sbc_indirect_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::SbcIndirectX as u8, 0x10, // SBC ($10,X)
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x50;
        cpu.registers.x = 0x04;
        cpu.registers.sr.c = true;
        cpu.memory.write_byte(0x14, 0x55);
        cpu.memory.write_byte(0x15, 0x80);

        cpu.memory.write_byte(0x8055, 0x20);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x30);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_sbc_indirect_y_npc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::SbcIndirectY as u8, 0x10, // SBC ($10),Y
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x50;
        cpu.registers.y = 0x03;
        cpu.registers.sr.c = true;
        cpu.memory.write_byte(0x10, 0x00);
        cpu.memory.write_byte(0x11, 0x20);
        cpu.memory.write_byte(0x2003, 0x20);

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x30);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_sbc_indirect_y_pc() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::SbcIndirectY as u8, 0x10, // SBC ($10),Y
            Opcodes::BRK as u8,
        ]);
    
        cpu.registers.ac = 0x50;
        cpu.registers.y = 0x01;
        cpu.registers.sr.c = true;
        cpu.memory.write_byte(0x10, 0xFF);
        cpu.memory.write_byte(0x11, 0x20);
        cpu.memory.write_byte(0x2100, 0x20);
    
        let cycles = cpu.step();
    
        assert_eq!(cpu.registers.ac, 0x30);
        assert_eq!(cpu.registers.sr.c, true);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cpu.registers.sr.v, false);
        assert_eq!(cycles, 6);
    }



    
    
    #[test]
    fn test_sec() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Sec as u8
        ]);
        cpu.registers.sr.c = false;
        let cycles = cpu.step();
        assert!(cpu.registers.sr.c);
        assert_eq!(cycles, 2);
    }
    
    
    #[test]
    fn test_sed() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Sed as u8
        ]);
        cpu.registers.sr.d = false;
        let cycles = cpu.step();
        assert!(cpu.registers.sr.d);
        assert_eq!(cycles, 2);
    }
    
    
    #[test]
    fn test_sei() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Sei as u8
        ]);
        cpu.registers.sr.i = false;
        let cycles = cpu.step();
        assert!(cpu.registers.sr.i);
        assert_eq!(cycles, 2);
    }
    
    
    
    
    #[test]
    fn test_sta_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StaZeropage as u8, 0x10,  // STA $10
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x42;

        let cycles = cpu.step();

        let stored = cpu.memory.read_byte(0x10);
        assert_eq!(stored, 0x42);
        assert_eq!(cycles, 3);
    }

    
    #[test]
    fn test_sta_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StaZeropageX as u8, 0x10, // STA $10,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x55;
        cpu.registers.x = 0x05;

        let cycles = cpu.step();

        let stored = cpu.memory.read_byte(0x15);
        assert_eq!(stored, 0x55);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_sta_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StaAbsolute as u8, 0x00, 0x20, // STA $2000
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x55;

        let cycles = cpu.step();

        let stored = cpu.memory.read_byte(0x2000);
        assert_eq!(stored, 0x55);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_sta_absolute_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StaAbsoluteX as u8, 0x00, 0x20, // STA $2000,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x42;
        cpu.registers.x = 0x05;

        let cycles = cpu.step();

        let stored = cpu.memory.read_byte(0x2005);
        assert_eq!(stored, 0x42);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_sta_absolute_y() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StaAbsoluteY as u8, 0x00, 0x20, // STA $2000,Y
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x37;
        cpu.registers.y = 0x05;

        let cycles = cpu.step();

        let stored = cpu.memory.read_byte(0x2005);
        assert_eq!(stored, 0x37);
        assert_eq!(cycles, 5);
    }

    
    #[test]
    fn test_sta_indirect_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StaIndirectX as u8, 0x10, // STA ($10,X)
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x42;
        cpu.registers.x = 0x04;
        cpu.memory.write_byte(0x14, 0x00);
        cpu.memory.write_byte(0x15, 0x20);

        let cycles = cpu.step();

        let stored = cpu.memory.read_byte(0x2000);
        assert_eq!(stored, 0x42);
        assert_eq!(cycles, 6);
    }

    #[test]
    fn test_sta_indirect_y() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StaIndirectY as u8, 0x10, // STA ($10),Y
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x42;
        cpu.registers.y = 0x04;
        cpu.memory.write_byte(0x10, 0x00);
        cpu.memory.write_byte(0x11, 0x20);

        let cycles = cpu.step();

        let stored = cpu.memory.read_byte(0x2000 + 0x04);
        assert_eq!(stored, 0x42);
        assert_eq!(cycles, 6);
    }

    
    
    
    #[test]
    fn test_stx_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StxZeropage as u8, 0x10, // STX $10
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x37;

        let cycles = cpu.step();

        let stored = cpu.memory.read_byte(0x10);
        assert_eq!(stored, 0x37);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_stx_zeropage_y() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StxZeropageY as u8, 0x10, // STX $10,Y
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x42;
        cpu.registers.y = 0x05;

        let cycles = cpu.step();

        let address = 0x10u8.wrapping_add(cpu.registers.y);
        let stored = cpu.memory.read_byte(address as u32);
        assert_eq!(stored, 0x42);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_stx_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StxAbsolute as u8, 0x00, 0x20, // STX $2000
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x42;

        let cycles = cpu.step();

        let stored = cpu.memory.read_byte(0x2000);
        assert_eq!(stored, 0x42);
        assert_eq!(cycles, 4);
    }

    
    
    
    
    #[test]
    fn test_sty_zeropage() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StyZeropage as u8, 0x10, // STY $10
            Opcodes::BRK as u8,
        ]);

        cpu.registers.y = 0x37;

        let cycles = cpu.step();

        let stored = cpu.memory.read_byte(0x10);
        assert_eq!(stored, 0x37);
        assert_eq!(cycles, 3);
    }

    #[test]
    fn test_sty_zeropage_x() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StyZeropageX as u8, 0x10, // STY $10,X
            Opcodes::BRK as u8,
        ]);

        cpu.registers.y = 0x42;
        cpu.registers.x = 0x05;

        let cycles = cpu.step();

        let address = 0x10u8.wrapping_add(cpu.registers.x);
        let stored = cpu.memory.read_byte(address as u32);
        assert_eq!(stored, 0x42);
        assert_eq!(cycles, 4);
    }

    
    #[test]
    fn test_sty_absolute() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::StyAbsolute as u8, 0x00, 0x20, // STY $2000
            Opcodes::BRK as u8,
        ]);

        cpu.registers.y = 0x42;

        let cycles = cpu.step();

        let stored = cpu.memory.read_byte(0x2000);
        assert_eq!(stored, 0x42);
        assert_eq!(cycles, 4);
    }
    
    
    
    #[test]
    fn test_tax() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Tax as u8, // TAX
            Opcodes::BRK as u8,
        ]);

        cpu.registers.ac = 0x80;
        let cycles = cpu.step();

        assert_eq!(cpu.registers.x, 0x80);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, true);
        assert_eq!(cycles, 2);
    }

    
    
    #[test]
    fn test_tay() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Tay as u8, // TAY
            Opcodes::BRK as u8,
        ]);
        
        cpu.registers.y = 0x01;
        cpu.registers.ac = 0x00;
        let cycles = cpu.step();

        assert_eq!(cpu.registers.y, 0x00);
        assert_eq!(cpu.registers.sr.z, true);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }

    
    
    #[test]
    fn test_tsx() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Tsx as u8, // TSX
            Opcodes::BRK as u8,
        ]);

        cpu.registers.sp = 0x42;
        let cycles = cpu.step();

        assert_eq!(cpu.registers.x, 0x42);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }

    
    
    
    
    #[test]
    fn test_txa() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Txa as u8, // TXA
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x42;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x42);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }

    
    
    #[test]
    fn test_txs() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Txs as u8, // TXS
            Opcodes::BRK as u8,
        ]);

        cpu.registers.x = 0x42;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.sp, 0x42);
        assert_eq!(cycles, 2);
    }

    
    
    #[test]
    fn test_tya() {
        let mut cpu = setup_cpu_with_program(&[
            Opcodes::Tya as u8, // TYA
            Opcodes::BRK as u8,
        ]);

        cpu.registers.y = 0x42;

        let cycles = cpu.step();

        assert_eq!(cpu.registers.ac, 0x42);
        assert_eq!(cpu.registers.sr.z, false);
        assert_eq!(cpu.registers.sr.n, false);
        assert_eq!(cycles, 2);
    }

    #[test]
    fn test_fibonacci_sequence() {
        use Opcodes::*;

        let program = vec![
            LdaImmediate as u8, 0x00,// LDA #$00
            StaZeropage as u8, 0x00,// STA $00
            LdaImmediate as u8, 0x01,// LDA #$01
            StaZeropage as u8, 0x01,// STA $01
            LdyImmediate as u8, 0x0A,// LDY #$0A (10 iterations)

            LdaZeropage as u8, 0x00,// LDA $00
            Clc as u8,// CLC
            AdcZeropage as u8, 0x01,// ADC $01
            StaZeropage as u8, 0x02,// STA $02
            LdaZeropage as u8, 0x01,// LDA $01
            StaZeropage as u8, 0x00,// STA $00
            LdaZeropage as u8, 0x02,// LDA $02
            StaZeropage as u8, 0x01,// STA $01 
            Dey as u8,// DEY 1
            BneRelative as u8, 0xEE,// BNE
            BRK as u8,// BRK
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            cpu.step();
        
            if cpu.memory.read_byte(cpu.registers.pc as u32) == Opcodes::BRK as u8 {
                break;
            }
        }
        let fib12 = cpu.memory.read_byte(0x02);
        assert_eq!(fib12, 89);
    }
    
    #[test]
    fn test_count_up_loop() {
        use Opcodes::*;
    
        let program = vec![
            LdxImmediate as u8, 0x00,// LDX #$00
            StxZeropage as u8, 0x00,// STX $00

            LdxZeropage as u8, 0x00,// LDX $00
            Inx as u8,// INX
            StxZeropage as u8, 0x00,// STX $00
            CpxImmediate as u8, 0xFF,// CPX #$FF
            BneRelative as u8, 0xF7,// BNE loop
            BRK as u8,// BRK
        ];
    
        let mut cpu = setup_cpu_with_program(&program);
    
        loop {
            let pc_before = cpu.registers.pc;
            let opcode = cpu.memory.read_byte(pc_before as u32);
            cpu.step();
    
            if opcode == Opcodes::BRK as u8 {
                break;
            }
        }
    
        let final_value = cpu.memory.read_byte(0x00);
        assert_eq!(final_value, 0xFF);
    }
    
    
    #[test]
    fn test_multiplication_via_addition() {
        use Opcodes::*;
    
        let program = vec![
            LdxImmediate as u8, 0x05,// LDX #$05
            StyZeropage as u8, 0x01,// STY $01
            LdyImmediate as u8, 0x03,// LDY #$03
            StyZeropage as u8, 0x01,// STY $01
    
            LdaImmediate as u8, 0x00,// LDA #$00
            StaZeropage as u8, 0x02,// STA $02

            CpxImmediate as u8, 0x00,// CPX #$00
            BeqRelative as u8, 0x0B,// BEQ done
    
            Cld as u8,// CLD
            LdaZeropage as u8, 0x02,// LDA $02
            Clc as u8,// CLC
            AdcZeropage as u8, 0x01,// ADC $01
            StaZeropage as u8, 0x02,// STA $02
    
            Dex as u8,// DEX
            BneRelative as u8, 0xF4,// BNE

            BRK as u8,
        ];
    
        let mut cpu = setup_cpu_with_program(&program);
    
        loop {
            let pc_before = cpu.registers.pc;
            let opcode = cpu.memory.read_byte(pc_before as u32);
            cpu.step();
    
            if opcode == Opcodes::BRK as u8 {
                break;
            }
        }
    
        let result = cpu.memory.read_byte(0x02);
        assert_eq!(result, 15);
    }
}
