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
        self.registors.pc = reset_vector;
        self.registors.sp = 0x0100;
        
        self.registors.sr = StatusRegisterValues::new();
        
        
        self.cycle_count = 0;
    }
    
    pub fn step(&mut self) -> i64{
        let pc = self.registors.pc;
        let opcode = self.memory.read_byte(pc as u32);
        
        self.registors.pc += 1; //move to next byte
        
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
            
            x if x == Opcodes::AslAccumulator as u8 => self.asl_accumulator_x(),
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
            
            x if x == Opcodes::BRK            as u8 => {println!("BRK"); return -1;}
            
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
        
        let x = self.registors.x;

        if self.page_boundary_cross(address, x as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(x as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registors.ac = value as i8;
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn lda_absolute_y(&mut self){
        println!("lda_absolute_y");
        let address = self.read_u16(self.registors.pc);
        
        self.registors.pc += 2; //lda_absolute_x absolute takes 3 bytes, one was already done in step
        
        let y = self.registors.y;
        
        if self.page_boundary_cross(address, y as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(y as u16);
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

        let effective_address = self.read_u16_zero_page(pointer_address as u8);
        
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registors.ac = value as i8;
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    fn lda_indirect_y(&mut self){
        println!("lda_indirect_y");
        
        let zero_page_operand = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 1;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let y = self.registors.y;

        if self.page_boundary_cross(base_address, y as u8) {
            self.cycle_count += 6; // Page boundary crossed
        } else {
            self.cycle_count += 5; // No crossing
        }
        
        let effective_address = base_address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registors.ac = value as i8;
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    
    
    
    fn adc_immediate(&mut self) {
        let value = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 1;

        let acc = self.registors.ac as u8;
        let carry_in = if self.registors.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registors.sr.c = result > 0xFF;
        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registors.ac = result_byte as i8;

        self.cycle_count += 2;
    }
    fn adc_zero_page(&mut self){
        let address = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 1;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let acc = self.registors.ac as u8;
        let carry_in = if self.registors.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registors.sr.c = result > 0xFF;
        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registors.ac = result_byte as i8;

        self.cycle_count += 3;
    }
    fn adc_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registors.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registors.pc += 1;

        let address = base_address.wrapping_add(self.registors.x as u8) & 0xFF;


        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let acc = self.registors.ac as u8;
        let carry_in = if self.registors.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registors.sr.c = result > 0xFF;
        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registors.ac = result_byte as i8;

        self.cycle_count += 4;
    }
    fn adc_absolute(&mut self){
        let address = self.read_u16(self.registors.pc);
        self.registors.pc += 2;

        let value = self.memory.read_byte(address as u32);

        let acc = self.registors.ac as u8;
        let carry_in = if self.registors.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registors.sr.c = result > 0xFF;
        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registors.ac = result_byte as i8;

        self.cycle_count += 4;
    }
    fn adc_absolute_x(&mut self){
        let address = self.read_u16(self.registors.pc);
        
        self.registors.pc += 2;
        
        let x = self.registors.x;

        if self.page_boundary_cross(address, x as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(x as u16);
        let value = self.memory.read_byte(effective_address as u32);

        let acc = self.registors.ac as u8;
        let carry_in = if self.registors.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registors.sr.c = result > 0xFF;
        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registors.ac = result_byte as i8;
    }
    fn adc_absolute_y(&mut self){
        let address = self.read_u16(self.registors.pc);
        
        self.registors.pc += 2; 
        
        let y = self.registors.y;
        
        if self.page_boundary_cross(address, y as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let acc = self.registors.ac as u8;
        let carry_in = if self.registors.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registors.sr.c = result > 0xFF;
        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registors.ac = result_byte as i8;
    }
    fn adc_indirect_x(&mut self){      
        let zero_page_operand = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 1;
        
        self.cycle_count += 6;

        let pointer_address = zero_page_operand.wrapping_add(self.registors.x as u8);

        let effective_address = self.read_u16_zero_page(pointer_address as u8);
        
        let value = self.memory.read_byte(effective_address as u32);
        
        let acc = self.registors.ac as u8;
        let carry_in = if self.registors.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registors.sr.c = result > 0xFF;
        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registors.ac = result_byte as i8;
    }
    fn adc_indirect_y(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 1;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let y = self.registors.y;

        if self.page_boundary_cross(base_address, y as u8) {
            self.cycle_count += 6; // Page boundary crossed
        } else {
            self.cycle_count += 5; // No crossing
        }
        
        let effective_address = base_address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let acc = self.registors.ac as u8;
        let carry_in = if self.registors.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;

        self.registors.sr.c = result > 0xFF;
        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.sr.v = ((acc ^ result_byte) & (value ^ result_byte) & 0x80) != 0;
        self.registors.ac = result_byte as i8;
    }


    fn and_immediate(&mut self){
        let value = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 1;
   
        let result_byte = value & self.registors.ac as u8;

        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.ac = result_byte as i8;

        self.cycle_count += 2;
    }
    fn and_zero_page(&mut self){
        let address = self.memory.read_byte(self.registors.pc as u32);
        self.cycle_count += 3;
        self.registors.pc += 1; 
        
        let value = self.memory.read_byte(address as u32);
        
        let result_byte = value & self.registors.ac as u8;

        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.ac = result_byte as i8;
    }
    fn and_zero_page_x(&mut self){
        let base_address = self.memory.read_byte(self.registors.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registors.pc += 1;

        let address = base_address.wrapping_add(self.registors.x as u8) & 0xFF;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let result_byte = value & self.registors.ac as u8;

        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.ac = result_byte as i8;

        self.cycle_count += 4;
    }
    fn and_absolute(&mut self){
        let address = self.read_u16(self.registors.pc);
        self.registors.pc += 2;

        let value = self.memory.read_byte(address as u32);

        let result_byte = value & (self.registors.ac as u8);

        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.ac = result_byte as i8;

        self.cycle_count += 4;
    }
    fn and_absolute_x(&mut self){
        let address = self.read_u16(self.registors.pc);
        
        self.registors.pc += 2;
        
        let x = self.registors.x;

        if self.page_boundary_cross(address, x as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(x as u16);
        let value = self.memory.read_byte(effective_address as u32);

        let result_byte = value & (self.registors.ac as u8);

        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.ac = result_byte as i8;
    }
    fn and_absolute_y(&mut self){
        let address = self.read_u16(self.registors.pc);
        
        self.registors.pc += 2; 
        
        let y = self.registors.y;
        
        if self.page_boundary_cross(address, y as u8) {
            self.cycle_count += 5; // page boundary crossed
        } else {
            self.cycle_count += 4; // no crossing
        }
        
        let effective_address = address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value & (self.registors.ac as u8);

        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.ac = result_byte as i8;
    }
    fn and_indirect_x(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 1;
        
        self.cycle_count += 6;

        let pointer_address = zero_page_operand.wrapping_add(self.registors.x as u8);

        let effective_address = self.read_u16_zero_page(pointer_address as u8);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value & (self.registors.ac as u8);

        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.ac = result_byte as i8;
    }
    fn and_indirect_y(&mut self){
        let zero_page_operand = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 1;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let y = self.registors.y;

        if self.page_boundary_cross(base_address, y as u8) {
            self.cycle_count += 6; // Page boundary crossed
        } else {
            self.cycle_count += 5; // No crossing
        }
        
        let effective_address = base_address.wrapping_add(y as u16);
        let value = self.memory.read_byte(effective_address as u32);
        
        let result_byte = value & (self.registors.ac as u8);

        self.registors.sr.z = result_byte == 0;
        self.registors.sr.n = (result_byte & 0x80) != 0;
        self.registors.ac = result_byte as i8;
    }


    fn asl_accumulator_x(&mut self){
        self.registors.pc += 0;
    }
    fn asl_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn asl_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn asl_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn asl_absolute_x(&mut self){
        self.registors.pc += 2;
    }
    
    
    fn bcc_relative(&mut self){
        self.registors.pc += 1;
    }
    
    fn bcs_relative(&mut self){
        self.registors.pc += 1;
    }
    fn beq_relative(&mut self){
        self.registors.pc += 1;
    }
    
    
    fn bit_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn bit_absolute(&mut self){
        self.registors.pc += 2;
    }
    
    
    fn bmi_relative(&mut self){
        self.registors.pc += 1;
    }
    
    fn bne_relative(&mut self){
        self.registors.pc += 1;
    }
    
    fn bpl_relative(&mut self){
        self.registors.pc += 1;
    }
    
    fn bvc_relative(&mut self){
        self.registors.pc += 1;   
    }
    
    fn  bvs_relative(&mut self){
        self.registors.pc += 1;
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
    
    
    fn cmp_immediate(&mut self){
        self.registors.pc += 1;
    }
    fn cmp_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn cmp_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn cmp_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn cmp_absolute_x(&mut self){
        self.registors.pc += 2;
    }
    fn cmp_absolute_y(&mut self){
        self.registors.pc += 2;
    }
    fn cmp_indirect_x(&mut self){
        self.registors.pc += 1;
    }
    fn cmp_indirect_y(&mut self){
        self.registors.pc += 1;
    }


    fn cpx_immediate(&mut self){
        self.registors.pc += 1;
    }
    fn cpx_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn cpx_absolute(&mut self){
        self.registors.pc += 2;
    }
    
    
    fn cpy_immediate(&mut self){
        self.registors.pc += 1;
    }
    fn cpy_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn cpy_absolute(&mut self){
        self.registors.pc += 2;
    }
    
    
    fn dec_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn dec_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn dec_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn dec_absolute_x(&mut self){
        self.registors.pc += 2;
    }
    
    
    fn dex(&mut self){
        
    }
    
    fn dey(&mut self){
        
    }
    
    
    fn eor_immediate(&mut self){
        self.registors.pc += 1;
    }
    fn eor_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn eor_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn eor_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn eor_absolute_x(&mut self){
        self.registors.pc += 2;
    }
    fn eor_absolute_y(&mut self){
        self.registors.pc += 2;
    }
    fn eor_indirect_x(&mut self){
        self.registors.pc += 1;
    }
    fn eor_indirect_y(&mut self){
        self.registors.pc += 1;
    }
    
    
    fn inc_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn inc_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn inc_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn inc_absolute_x(&mut self){
        self.registors.pc += 2;
    }
    
    
    fn inx(&mut self){
        
    }
    
    fn iny(&mut self){
        
    }
    
    
    fn jmp_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn jmp_indirect(&mut self){
        self.registors.pc += 2;
    }
    
    fn jsr_absolute(&mut self){
        self.registors.pc += 2;
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
        println!("ldx_absolute_y");
        
        let zero_page_operand = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 2;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let effective_address = base_address.wrapping_add(self.registors.y as u16);

        if self.page_boundary_cross(base_address, self.registors.y as u8) {
            self.cycle_count += 5;
        } else {
            self.cycle_count += 4;
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
    fn ldy_absolute_x(&mut self){
        println!("ldy_indirect_x");
        
        let zero_page_operand = self.memory.read_byte(self.registors.pc as u32);
        self.registors.pc += 2;
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let effective_address = base_address.wrapping_add(self.registors.x as u16);


        if self.page_boundary_cross(base_address, self.registors.x as u8) {
            self.cycle_count += 5;
        } else {
            self.cycle_count += 4;
        }
        
        let value = self.memory.read_byte(effective_address as u32);
        
        self.registors.y = value as i8;
        self.registors.sr.z = value == 0;
        self.registors.sr.n = (value & 0x80) != 0;
    }
    
    fn lsr_accumulator(&mut self){
        
    }
    fn lsr_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn lsr_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn lsr_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn lsr_absolute_x(&mut self){
        self.registors.pc += 2;
    }
    
    fn nop(&mut self){
        println!("nop");
        self.cycle_count += 2;
        //self.registors.pc += 0; //nop takes no operands
    }
    
    
    fn ora_immediate(&mut self){
        self.registors.pc += 1;
    }
    fn ora_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn ora_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn ora_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn ora_absolute_x(&mut self){
        self.registors.pc += 2;
    }
    fn ora_absolute_y(&mut self){
        self.registors.pc += 2;
    }
    fn ora_indirect_x(&mut self){
        self.registors.pc += 1;
    }
    fn ora_indirect_y(&mut self){
        self.registors.pc += 1;
    }
    
    fn pha(&mut self){
        
    }
    
    fn php(&mut self){
        
    }
    
    fn pla(&mut self){
        
    }
    
    fn plp(&mut self){
        
    }
    
    
    
    fn rol_accumulator(&mut self){
        
    }
    fn rol_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn rol_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn rol_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn rol_absolute_x(&mut self){
        self.registors.pc += 2;
    }
    
    fn ror_accumulator(&mut self){
        
    }
    fn ror_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn ror_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn ror_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn ror_absolute_x(&mut self){
        self.registors.pc += 2;
    }
    
    fn rti(&mut self){
        
    }
    
    fn rts(&mut self){
        
    }
    
    
    fn sbc_immediate(&mut self){
        self.registors.pc += 1;
    }
    fn sbc_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn sbc_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn sbc_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn sbc_absolute_x(&mut self){
        self.registors.pc += 2;
    }
    fn sbc_absolute_y(&mut self){
        self.registors.pc += 2;
    }
    fn sbc_indirect_x(&mut self){
        self.registors.pc += 1;
    }
    fn sbc_indirect_y(&mut self){
        self.registors.pc += 1;
    }
    
    fn sec(&mut self){
        
    }
    
    fn sed(&mut self){
        
    }
    
    fn sei(&mut self){
        
    }
    
    
    fn sta_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn sta_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn sta_absolute(&mut self){
        self.registors.pc += 2;
    }
    fn sta_absolute_x(&mut self){
        self.registors.pc += 2;
    }
    fn sta_absolute_y(&mut self){
        self.registors.pc += 2;
    }
    fn sta_indirect_x(&mut self){
        self.registors.pc += 1;
    }
    fn sta_indirect_y(&mut self){
        self.registors.pc += 1;
    }
    
    
    fn stx_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn stx_zero_page_y(&mut self){
        self.registors.pc += 1;
    }
    fn stx_absolute(&mut self){
        self.registors.pc += 2;
    }
    
    
    fn sty_zero_page(&mut self){
        self.registors.pc += 1;
    }
    fn sty_zero_page_x(&mut self){
        self.registors.pc += 1;
    }
    fn sty_absolute(&mut self){
        self.registors.pc += 2;
    }
    
    fn tax(&mut self){
        
    }
    
    fn tay(&mut self){
        
    }
    
    fn tsx(&mut self){
        
    }
    
    fn txa(&mut self){
        
    }
    
    fn txs(&mut self){
        
    }
    
    fn tya(&mut self){
        
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
        let base_pointer_address: u16 = 0x12FF;
        let data_value: u8 = 0x42;
    
        cpu.memory.write_byte(0x0020, (base_pointer_address & 0xFF) as u8); // Low byte
        cpu.memory.write_byte(0x0021, (base_pointer_address >> 8) as u8);   // High byte
        cpu.memory.write_byte(base_pointer_address as u32, data_value);     // Store value
    
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
