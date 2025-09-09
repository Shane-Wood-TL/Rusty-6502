use crate::Memory;
use crate::StatusRegisterValues;
use crate::Registers6502;
use crate::Opcodes;

pub struct Cpu6502{
    pub registers: Registers6502,
    pub memory: Memory,
    pub cycle_count: u32
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
        
        self.registers.pc = self.registers.pc.wrapping_add(1); //move to next byte
        
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
        println!("lda_immediate");
        self.registers.pc = self.registers.pc.wrapping_add(1); //LDA immediate takes 2 bytes,
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
        self.registers.pc = self.registers.pc.wrapping_add(1); //LDA zero page takes 2 bytes, one was already done in step
        
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
        self.registers.pc = self.registers.pc.wrapping_add(1); //LDA zero page takes 2 bytes, one was already done in step
        
        let address = base_address.wrapping_add(self.registers.x as u8);
        
        let value = self.memory.read_byte(address as u32);
        self.registers.ac = value as u8;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn lda_absolute(&mut self){
        println!("lda_absolute");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2); //LDA absolute takes 3 bytes, one was already done in step
        
        self.cycle_count += 4;
        
        let value = self.memory.read_byte(address as u32);
        
        self.registers.ac = value as u8;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn lda_absolute_x(&mut self){
        println!("lda_absolute_x");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2); //lda_absolute_x absolute takes 3 bytes, one was already done in step
        
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
        
        self.registers.pc = self.registers.pc.wrapping_add(2); //lda_absolute_x absolute takes 3 bytes, one was already done in step
        
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
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("adc_immediate");
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let acc = self.registers.ac as u8;
        let carry_in = if self.registers.sr.c { 1 } else { 0 };

        let result = acc as u16 + value as u16 + carry_in as u16;
        let result_byte = result as u8;
    
         println!("ADC: A={}, value={}, carry_in={}, result={}", acc, value, carry_in, result_byte);
         
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
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("adc_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("adc_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

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
        println!("adc_absolute_x");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
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
        println!("adc_absolute_y");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2); 
        
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
        println!("adc_indirect_x");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("adc_indirect_y");    
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("and_immediate");
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
   
        let result_byte = value & self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 2;
    }
    fn and_zero_page(&mut self){
        println!("and_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 3;
        self.registers.pc = self.registers.pc.wrapping_add(1); 
        
        let value = self.memory.read_byte(address as u32);
        
        let result_byte = value & self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn and_zero_page_x(&mut self){
        println!("and_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let result_byte = value & self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn and_absolute(&mut self){
        println!("and_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        let value = self.memory.read_byte(address as u32);

        let result_byte = value & (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn and_absolute_x(&mut self){
        println!("and_absolute_x");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
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
        println!("and_absolute_y");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2); 
        
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
        println!("and_indirect_x");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("and_indirect_y");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("asl_accumulator");
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
        println!("asl_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1); 
        
        let value = self.memory.read_byte(address as u32);
        
        self.registers.sr.c = (value & 0x80) != 0;
        let result_byte = value << 1;
        
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.memory.write_byte(address as u32, result_byte);

        self.cycle_count += 5;
    }
    fn asl_zero_page_x(&mut self){
        println!("asl_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("asl_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        let value = self.memory.read_byte(address as u32);

        self.registers.sr.c = (value & 0x80) != 0;
        let result_byte = value << 1;
        
        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.memory.write_byte(address as u32, result_byte);

        self.cycle_count += 6;
    }
    fn asl_absolute_x(&mut self){
        println!("asl_absolute_x");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
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
        println!("bcc_relative");
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("bcs_relative");
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("beq_relative");
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("bit_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1); 
        
        let value = self.memory.read_byte(address as u32);
        
        let and_result = (self.registers.ac as u8) & value;

        self.registers.sr.z = and_result == 0;
        self.registers.sr.n = (value & 0x80) != 0;
        self.registers.sr.v = (value & 0x40) != 0;

        self.cycle_count += 3; 
    }

    fn bit_absolute(&mut self){
        println!("bit_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        let value = self.memory.read_byte(address as u32);
        
        let and_result = (self.registers.ac as u8) & value;

        
        self.registers.sr.z = and_result == 0;
        self.registers.sr.n = (value & 0x80) != 0;
        self.registers.sr.v = (value & 0x40) != 0;

        self.cycle_count += 4;
    }
    
    
    fn bmi_relative(&mut self){ //branch on n = 1
        println!("bmi_relative");
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc = self.registers.pc.wrapping_add(1);

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

        println!("Zero flag {}", self.registers.sr.z);
        if !self.registers.sr.z {
            let old_pc = self.registers.pc;
            let new_pc = ((self.registers.pc as i32) + (offset as i32)) as u16;

            self.cycle_count += 1;  // +1 cycle for taken branch
            if self.page_boundary_cross_signed(old_pc, offset) {
                // Page crossed
                self.cycle_count += 1;  //+2 total increase
            }

            self.registers.pc = new_pc;
        } else {
            println!("Branch not taken");
        }
        self.cycle_count += 2;  //cycle count for not take
        
    }
    
    fn bpl_relative(&mut self){ //branch on n = 0
        println!("bpl_relative");
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
        println!("bvc_relative");
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("bvs_relative");
        let offset = self.memory.read_byte(self.registers.pc as u32) as i8;
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("cmp_immediate");
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let acc = self.registers.ac;
        let result = acc.wrapping_sub(value);

        self.registers.sr.c = acc >= value;
        self.registers.sr.z = acc == value;
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    fn cmp_zero_page(&mut self){
        println!("cmp_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let acc = self.registers.ac;
        let result = acc.wrapping_sub(value);

        self.registers.sr.c = acc >= value;
        self.registers.sr.z = acc == value;
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 3;
    }
    fn cmp_zero_page_x(&mut self){
        println!("cmp_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("cmp_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        let value = self.memory.read_byte(address as u32);

        let acc = self.registers.ac;
        let result = acc.wrapping_sub(value);

        self.registers.sr.c = acc >= (value);
        self.registers.sr.z = acc == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 4;
    }
    fn cmp_absolute_x(&mut self){
        println!("cmp_absolute_x");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
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
        println!("cmp_absolute_y");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
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
        println!("cmp_indirect_x");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("cmp_indirect_y");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("cpx_immediate");
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let x = self.registers.x;
        let result = x.wrapping_sub(value);

        self.registers.sr.c = x >= (value);
        self.registers.sr.z = x == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    fn cpx_zero_page(&mut self){
        println!("cpx_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let x = self.registers.x;
        let result = x.wrapping_sub(value);

        self.registers.sr.c = x >= (value);
        self.registers.sr.z = x == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 3;
    }
    fn cpx_absolute(&mut self){
        println!("cpx_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        let value = self.memory.read_byte(address as u32);

        let x = self.registers.x;
        let result = x.wrapping_sub(value);

        self.registers.sr.c = x >= (value);
        self.registers.sr.z = x == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 4;
    }
    
    
    fn cpy_immediate(&mut self){
        println!("cpy_immediate");
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let y = self.registers.y;
        let result = y.wrapping_sub(value);

        self.registers.sr.c = y >= (value);
        self.registers.sr.z = y == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    fn cpy_zero_page(&mut self){
        println!("cpy_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let y = self.registers.y;
        let result = y.wrapping_sub(value);

        self.registers.sr.c = y >= (value);
        self.registers.sr.z = y == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 3;
    }
    fn cpy_absolute(&mut self){
        println!("cpy_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        let value = self.memory.read_byte(address as u32);

        let y = self.registers.y;
        let result = y.wrapping_sub(value);

        self.registers.sr.c = y >= (value);
        self.registers.sr.z = y == (value);
        self.registers.sr.n = (result as u8 & 0x80) != 0;

        self.cycle_count += 4;
    }
    
    
    fn dec_zero_page(&mut self){
        println!("dec_zero_page");
        let base_address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let mut value = self.memory.read_byte(base_address as u32);
        value = value.wrapping_sub(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value as u8 & 0x80) != 0;

        self.memory.write_byte(base_address as u32, value as u8);

        self.cycle_count += 5;
    }
    fn dec_zero_page_x(&mut self){
        println!("dec_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let mut value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        value = value.wrapping_sub(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value as u8 & 0x80) != 0;

        self.memory.write_byte(address as u32, value as u8);

        self.cycle_count += 6;
    }
    fn dec_absolute(&mut self){
        println!("dec_absolute");
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
 
        let mut value = self.memory.read_byte(base_address as u32);
        value = value.wrapping_sub(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;

        self.memory.write_byte(base_address as u32, value);

        self.cycle_count += 6;
    }
    fn dec_absolute_x(&mut self){
        println!("dec_absolute_x");
        let zero_page_operand = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
        let effective_address = zero_page_operand.wrapping_add(self.registers.x as u16);

        let mut value = self.memory.read_byte(effective_address as u32);
        value = value.wrapping_sub(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;

        self.memory.write_byte(effective_address as u32, value);

        self.cycle_count += 7;
    }
    
    
    fn dex(&mut self){
        println!("dex");
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.registers.sr.z  = self.registers.x == 0;
        self.registers.sr.n = (self.registers.x & 0x80) != 0;
        self.cycle_count += 2;
    }
    
    fn dey(&mut self){
        println!("dey");
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.registers.sr.z  = self.registers.y == 0;
        self.registers.sr.n = (self.registers.y & 0x80) != 0;
        self.cycle_count += 2;
    }
    
    
    fn eor_immediate(&mut self){
        println!("eor_immediate");
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
   
        let result_byte = value ^ self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 2;
    }
    fn eor_zero_page(&mut self){
        println!("eor_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 3;
        self.registers.pc = self.registers.pc.wrapping_add(1); 
        
        let value = self.memory.read_byte(address as u32);
        
        let result_byte = value ^ self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn eor_zero_page_x(&mut self){
        println!("eor_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let result_byte = value ^ self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn eor_absolute(&mut self){
        println!("eor_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        let value = self.memory.read_byte(address as u32);

        let result_byte = value ^ (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn eor_absolute_x(&mut self){
        println!("eor_absolute_x");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
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
        println!("eor_absolute_y");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2); 
        
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
        println!("eor_indirect_x");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("eor_indirect_y");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("inc_zero_page");
        let base_address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let mut value = self.memory.read_byte(base_address as u32);
        value = value.wrapping_add(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value as u8 & 0x80) != 0;

        self.memory.write_byte(base_address as u32, value as u8);

        self.cycle_count += 5;
    }
    fn inc_zero_page_x(&mut self){
        println!("inc_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let mut value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        value = value.wrapping_add(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value as u8 & 0x80) != 0;

        self.memory.write_byte(address as u32, value as u8);

        self.cycle_count += 6;
    }
    fn inc_absolute(&mut self){
        println!("inc_absolute");
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
 
        let mut value = self.memory.read_byte(base_address as u32);
        value = value.wrapping_add(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;

        self.memory.write_byte(base_address as u32, value);

        self.cycle_count += 6;
    }
    fn inc_absolute_x(&mut self){
        println!("inc_absolute_x");
        let zero_page_operand = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
        let pointer_address = zero_page_operand.wrapping_add(self.registers.x as u16);

        let mut value = self.memory.read_byte(pointer_address as u32);
        value = value.wrapping_add(1);

        self.registers.sr.z  = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;

        self.memory.write_byte(pointer_address as u32, value);

        self.cycle_count += 7;
    }
    
    
    fn inx(&mut self){
        println!("inx");
        self.registers.x = self.registers.x.wrapping_add(1);
        self.registers.sr.z  = self.registers.x == 0;
        self.registers.sr.n = (self.registers.x & 0x80) != 0;
        self.cycle_count += 2;
    }
    
    fn iny(&mut self){
        println!("iny");
        self.registers.y = self.registers.y.wrapping_add(1);
        self.registers.sr.z  = self.registers.y == 0;
        self.registers.sr.n = (self.registers.y & 0x80) != 0;
        self.cycle_count += 2;
    }
    
    
    fn jmp_absolute(&mut self){
        println!("jmp_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        self.registers.pc = address;
        self.cycle_count += 3;
    }

    fn jmp_indirect(&mut self) {
        println!("jmp_indirect");
        let addr_ptr = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);


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
        println!("jsr_absolute");
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
        println!("ldx_immediate");
        let value = self.memory.read_byte(self.registers.pc as u32);
        println!("LDX I");
        self.registers.pc = self.registers.pc.wrapping_add(1); //LDX immediate takes 2 bytes,
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
        self.registers.pc = self.registers.pc.wrapping_add(1); //LDA zero page takes 2 bytes, one was already done in step
        
        let value = self.memory.read_byte(address as u32);
        
        self.registers.x = value;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn ldx_zero_page_y(&mut self){
        println!("ldx_zero_page_y");
        let base_address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 4;
        self.registers.pc = self.registers.pc.wrapping_add(1); //LDA zero page takes 2 bytes, one was already done in step
        
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
        println!("ldy_immediate");
        self.registers.pc = self.registers.pc.wrapping_add(1); //LDY immediate takes 2 bytes,
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
        self.registers.pc = self.registers.pc.wrapping_add(1); //LDA zero page takes 2 bytes, one was already done in step
        
        let value = self.memory.read_byte(address as u32);
        
        self.registers.y = value;
        
        self.registers.sr.z = value == 0;
        self.registers.sr.n = (value & 0x80) != 0;
    }
    fn ldy_zero_page_x(&mut self){
        println!("ldy_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 4;
        self.registers.pc = self.registers.pc.wrapping_add(1); //LDA zero page takes 2 bytes, one was already done in step
        
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
        println!("lsr_accumulator");
    
        let acc = self.registers.ac;

        self.registers.sr.c = (acc & 0x01) != 0;

        let value = acc >> 1;
        self.registers.ac = value;

        self.registers.sr.z = value == 0;
        self.registers.sr.n = false;

        self.cycle_count += 2;
    }
    fn lsr_zero_page(&mut self){
        println!("lsr_zero_page");    
        let base_address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let mut value = self.memory.read_byte(base_address as u32);

        self.registers.sr.c = (value & 0x01) != 0;

        value = value >> 1;

        self.registers.sr.z = value == 0;
        self.registers.sr.n = false;

        self.memory.write_byte(base_address as u32, value as u8);

        self.cycle_count += 5;
    }
    fn lsr_zero_page_x(&mut self){
        println!("lsr_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("lsr_absolute");
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
 
        let mut value = self.memory.read_byte(base_address as u32);

        self.registers.sr.c = (value & 0x01) != 0;

        value = value >> 1;

        self.registers.sr.z = value == 0;
        self.registers.sr.n = false;

        self.memory.write_byte(base_address as u32, value as u8);

        self.cycle_count += 6;
    }
    fn lsr_absolute_x(&mut self){
        println!("lsr_absolute_x");
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

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
        println!("ora_immediate");
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
   
        let result_byte = value | self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 2;
    }
    fn ora_zero_page(&mut self){
        println!("ora_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.cycle_count += 3;
        self.registers.pc = self.registers.pc.wrapping_add(1); 
        
        let value = self.memory.read_byte(address as u32);
        
        let result_byte = value | self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;
    }
    fn ora_zero_page_x(&mut self){
        println!("ora_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);

        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        let value = self.memory.read_byte(address as u32); //takes an extra cycle since it also has to load this from memory
        
        let result_byte = value | self.registers.ac as u8;

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn ora_absolute(&mut self){
        println!("ora_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        let value = self.memory.read_byte(address as u32);

        let result_byte = value | (self.registers.ac as u8);

        self.registers.sr.z = result_byte == 0;
        self.registers.sr.n = (result_byte & 0x80) != 0;
        self.registers.ac = result_byte as u8;

        self.cycle_count += 4;
    }
    fn ora_absolute_x(&mut self){
        println!("ora_absolute_x");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
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
        println!("ora_absolute_y");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2); 
        
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
        println!("ora_indirect_x");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("ora_indirect_y");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("pha");
        self.memory.stack_push(&mut self.registers.sp, self.registers.ac as u8);
        self.cycle_count += 3;
    }
    
    fn php(&mut self){
        println!("php");
        self.memory.stack_push(&mut self.registers.sp, self.registers.sr.to_byte() as u8);
        self.cycle_count += 3;
    }
    
    fn pla(&mut self){
        println!("pla");
        self.registers.ac = self.memory.stack_pop(&mut self.registers.sp);

        self.registers.sr.z = self.registers.ac == 0;
        self.registers.sr.n = (self.registers.ac & 0x80) != 0;

        self.cycle_count += 4;
    }
    
    fn plp(&mut self){
        println!("plp");
        let new_status = self.memory.stack_pop(&mut self.registers.sp);
        println!("PLP read status byte: 0x{:02X}", new_status);
        self.registers.sr.from_byte(new_status);
        self.cycle_count += 4;
    }
    
    
    
    fn rol_accumulator(&mut self){
        println!("rol_accumulator");
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
        println!("rol_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1); 
        
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
        println!("rol_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);
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
        println!("rol_absolute");
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
 
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
        println!("rol_absolute_x");
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
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
        println!("ror_accumulator");
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
        println!("ror_zero_page");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);
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
        println!("ror_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);
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
        println!("ror_absolute");
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
 
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
        println!("ror_absolute_x");
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
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
        println!("rti");
        //The status register is pulled with the break flag
        //and bit 5 ignored. Then PC is pulled from the stack.
        let status = self.memory.stack_pop(&mut self.registers.sp);

        let status_byte = status & 0xEF; // Clear b and not used
    
        self.registers.sr.from_byte(status_byte);

        let pcl = self.memory.stack_pop(&mut self.registers.sp);
        let pch = self.memory.stack_pop(&mut self.registers.sp);

        self.registers.pc = ((pch as u16) << 8) | (pcl as u16);

        self.cycle_count += 6;
    }
    
    fn rts(&mut self){
        println!("rts");
        //pull PC, PC+1 -> P
        let pcl = self.memory.stack_pop(&mut self.registers.sp);
        let pch = self.memory.stack_pop(&mut self.registers.sp);
        
        self.registers.pc = ((pch as u16) << 8) | (pcl as u16);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
        self.cycle_count += 6;
    }
    
    
    fn sbc_immediate(&mut self){
        println!("sbc_immediate");
        //A - M - C̅ -> A
        let value = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("sbc_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1); 
        
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
        println!("sbc_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);

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
        println!("sbc_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

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
        println!("sbc_absolute_x");
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
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
        println!("sbc_absolute_y");
        let base_address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        
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
        println!("sbc_indirect_x");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        


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
        println!("sbc_indirect_y");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
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
        println!("sec");
        self.registers.sr.c = true;
        self.cycle_count += 2;
    }
    
    fn sed(&mut self){
        println!("sed");
        self.registers.sr.d = true;
        self.cycle_count += 2;
    }
    
    fn sei(&mut self){
        println!("sei");
        self.registers.sr.i = true;
        self.cycle_count += 2;
        
    }
    
    
    fn sta_zero_page(&mut self){
        println!("sta_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1); 
        
        self.memory.write_byte(address as u32, self.registers.ac);
        
        self.cycle_count += 3;
        
        println!("STA: mem[{:#X}] = {:#X}", address, self.registers.ac);

    }
    fn sta_zero_page_x(&mut self){
        println!("sta_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);
        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        self.memory.write_byte(address as u32, self.registers.ac);
        
        self.cycle_count += 4;
    }
    fn sta_absolute(&mut self){
        println!("sta_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        self.memory.write_byte(address as u32, self.registers.ac);
        
        self.cycle_count += 4;
    }
    fn sta_absolute_x(&mut self){
        println!("sta_absolute_x");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2);
        let x = self.registers.x;
        let effective_address = address.wrapping_add(x as u16);

        self.memory.write_byte(effective_address as u32, self.registers.ac);

        self.cycle_count += 5;
    }
    fn sta_absolute_y(&mut self){
        println!("sta_absolute_y");
        let address = self.read_u16(self.registers.pc);
        
        self.registers.pc = self.registers.pc.wrapping_add(2);
        let y = self.registers.y;
        let effective_address = address.wrapping_add(y as u16);

        self.memory.write_byte(effective_address as u32, self.registers.ac);

        self.cycle_count += 5;
    }
    fn sta_indirect_x(&mut self){
        println!("sta_indirect_x");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
        let pointer_address = zero_page_operand.wrapping_add(self.registers.x as u8);
        let effective_address = self.read_u16_zero_page(pointer_address as u8);

        self.memory.write_byte(effective_address as u32, self.registers.ac);

        self.cycle_count += 6;
    }
    fn sta_indirect_y(&mut self){
        println!("sta_indirect_y");
        let zero_page_operand = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
        let base_address = self.read_u16_zero_page(zero_page_operand as u8);
        
        let y = self.registers.y;
        let effective_address = base_address.wrapping_add(y as u16);

        self.memory.write_byte(effective_address as u32, self.registers.ac);

        self.cycle_count += 6;
    }
    
    
    fn stx_zero_page(&mut self){
        println!("stx_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1); 
        
        self.memory.write_byte(address as u32, self.registers.x);
        
        self.cycle_count += 3;
    }
    fn stx_zero_page_y(&mut self){
        println!("stx_zero_page_y");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);
        let address = base_address.wrapping_add(self.registers.y as u8);

        self.memory.write_byte(address as u32, self.registers.x);
        
        self.cycle_count += 4;
    }
    fn stx_absolute(&mut self){
        println!("stx_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        self.memory.write_byte(address as u32, self.registers.x);
        
        self.cycle_count += 4;
    }
    
    
    fn sty_zero_page(&mut self){
        println!("sty_zero_page");
        let address = self.memory.read_byte(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1); 
        
        self.memory.write_byte(address as u32, self.registers.y);
        
        self.cycle_count += 3;
    }
    fn sty_zero_page_x(&mut self){
        println!("sty_zero_page_x");
        let base_address = self.memory.read_byte(self.registers.pc as u32); //takes an extra cycle since it also has to load this from memory
        self.registers.pc = self.registers.pc.wrapping_add(1);
        let address = base_address.wrapping_add(self.registers.x as u8) & 0xFF;

        self.memory.write_byte(address as u32, self.registers.y);
        
        self.cycle_count += 4;
    }
    fn sty_absolute(&mut self){
        println!("sty_absolute");
        let address = self.read_u16(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        self.memory.write_byte(address as u32, self.registers.y);
        
        self.cycle_count += 4;
    }
    
    fn tax(&mut self){
        println!("tax");
        self.registers.x = self.registers.ac;

        self.registers.sr.z = self.registers.x == 0;
        self.registers.sr.n = (self.registers.x as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    
    fn tay(&mut self){
        println!("tay");
        self.registers.y = self.registers.ac;

        self.registers.sr.z = self.registers.y == 0;
        self.registers.sr.n = (self.registers.y as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    
    fn tsx(&mut self){
        println!("tsx");
        self.registers.x = self.registers.sp;

        self.registers.sr.z = self.registers.x == 0;
        self.registers.sr.n = (self.registers.x as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    
    fn txa(&mut self){
        println!("txa");
        self.registers.ac = self.registers.x;

        self.registers.sr.z = self.registers.ac == 0;
        self.registers.sr.n = (self.registers.ac as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
    
    fn txs(&mut self){
        println!("txs");
        self.registers.sp = self.registers.x as u8;
        self.cycle_count += 2;
    }
    
    fn tya(&mut self){
        println!("tya");
        self.registers.ac = self.registers.y;

        self.registers.sr.z = self.registers.ac == 0;
        self.registers.sr.n = (self.registers.ac as u8 & 0x80) != 0;

        self.cycle_count += 2;
    }
}