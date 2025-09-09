


#[cfg(test)]
mod tests {
    use crate::Opcodes;
    use crate::tests::test_utils::setup_cpu_with_program;
    use crate::Memory;
    
    
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
        cpu.registers.x = 0x0F; // $2010 + F = $201F (no page cross)
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
            Opcodes::EorImmediate as u8, 0x0F // EOR #F
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
            Opcodes::OraImmediate as u8, 0x0F, // ORA #F
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
            Opcodes::SbcZeropageX as u8, 0x0A, // SBC A,X
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
    fn test_memory_reset() {
        let mut memory = Memory::new();


        memory.write_byte(0, 123);
        memory.write_byte(1023, 255);
        memory.write_byte(65535, 42);

        assert_eq!(memory.read_byte(0), 123);
        assert_eq!(memory.read_byte(1023), 255);
        assert_eq!(memory.read_byte(65535), 42);

        memory.reset();

        assert_eq!(memory.read_byte(0), 0);
        assert_eq!(memory.read_byte(1023), 0);
        assert_eq!(memory.read_byte(65535), 0);
        assert_eq!(memory.read_byte(500), 0);
        assert_eq!(memory.read_byte(32000), 0);
    }


    #[test]
    fn test_write_word() {
        let mut memory = Memory::new();
        let mut cycles = 10;

        let address = 0x1000;
        let word: u16 = 0xABCD;

        memory.write_word(address, word, &mut cycles);

        assert_eq!(memory.read_byte(address), 0xCD);
        assert_eq!(memory.read_byte(address + 1), 0xAB);
        assert_eq!(cycles, 8);
    }
}
