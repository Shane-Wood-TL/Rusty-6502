

#[cfg(test)]
mod tests {
    use crate::Opcodes;
    use crate::tests::test_utils::setup_cpu_with_program;


    #[test]
    fn test_fibonacci_sequence() {

        let program = vec![
            Opcodes::LdaImmediate as u8, 0x00,// LDA #$00
            Opcodes::StaZeropage as u8, 0x00,// STA $00
            Opcodes::LdaImmediate as u8, 0x01,// LDA #$01
            Opcodes::StaZeropage as u8, 0x01,// STA $01
            Opcodes::LdyImmediate as u8, 0x0A,// LDY #$0A (10 iterations)

            Opcodes::LdaZeropage as u8, 0x00,// LDA $00
            Opcodes::Clc as u8,// CLC
            Opcodes::AdcZeropage as u8, 0x01,// ADC $01
            Opcodes::StaZeropage as u8, 0x02,// STA $02
            Opcodes::LdaZeropage as u8, 0x01,// LDA $01
            Opcodes::StaZeropage as u8, 0x00,// STA $00
            Opcodes::LdaZeropage as u8, 0x02,// LDA $02
            Opcodes::StaZeropage as u8, 0x01,// STA $01 
            Opcodes::Dey as u8,// DEY 1
            Opcodes::BneRelative as u8, 0xEE,// BNE
            Opcodes::BRK as u8,// BRK
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

        let program = vec![
            Opcodes::LdxImmediate as u8, 0x00,// LDX #$00
            Opcodes::StxZeropage as u8, 0x00,// STX $00

            Opcodes::LdxZeropage as u8, 0x00,// LDX $00
            Opcodes::Inx as u8,// INX
            Opcodes::StxZeropage as u8, 0x00,// STX $00
            Opcodes::CpxImmediate as u8, 0xFF,// CPX #$FF
            Opcodes::BneRelative as u8, 0xF7,// BNE loop
            Opcodes::BRK as u8,// BRK
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

        let program = vec![
            Opcodes::LdxImmediate as u8, 0x05,// LDX #$05
            Opcodes::StyZeropage as u8, 0x01,// STY $01
            Opcodes::LdyImmediate as u8, 0x03,// LDY #$03
            Opcodes::StyZeropage as u8, 0x01,// STY $01

            Opcodes::LdaImmediate as u8, 0x00,// LDA #$00
            Opcodes::StaZeropage as u8, 0x02,// STA $02

            Opcodes::CpxImmediate as u8, 0x00,// CPX #$00
            Opcodes::BeqRelative as u8, 0x0B,// BEQ done

            Opcodes::Cld as u8,// CLD
            Opcodes::LdaZeropage as u8, 0x02,// LDA $02
            Opcodes::Clc as u8,// CLC
            Opcodes::AdcZeropage as u8, 0x01,// ADC $01
            Opcodes::StaZeropage as u8, 0x02,// STA $02

            Opcodes::Dex as u8,// DEX
            Opcodes::BneRelative as u8, 0xF4,// BNE

            Opcodes::BRK as u8,
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


    #[test]
    fn test_hello_sequence() {

        let hello = [b'H', b'E', b'L', b'L', b'O'];

        let program = vec![
            Opcodes::LdxImmediate as u8, 0x00,// LDX #$00
            // a
            Opcodes::LdaZeropageX as u8, 0x10, // LDA $10,X
            Opcodes::StaZeropage as u8, 0x00, // STA $00
            Opcodes::Inx as u8, // INX
            Opcodes::CpxImmediate as u8, hello.len() as u8, // CPX #5
            Opcodes::BneRelative as u8, 0xF7, // BNE a
            Opcodes::BRK as u8,
        ];

        let mut cpu = setup_cpu_with_program(&program);

        for (i, &ch) in hello.iter().enumerate() {
            cpu.memory.write_byte(0x10 + i as u32, ch);
        }

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();

            if opcode == Opcodes::BRK as u8 {
                break;
            }
        }

        let output = cpu.memory.read_byte(0x00);
        assert_eq!(output, b'O');
    }


    #[test]
    fn test_countdown_loop() {
        
        let program = vec![
            Opcodes::LdxImmediate as u8, 0x05,// LDX #$05
            Opcodes::StaZeropage as u8, 0x00,// STX $00
            Opcodes::Dex as u8,// DEX
            Opcodes::StaZeropage as u8, 0x00,// STX $00
            Opcodes::BneRelative as u8, 0xFB,// BNE
            Opcodes::BRK as u8,// BRK
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

        let final_count = cpu.memory.read_byte(0x00);
        assert_eq!(final_count, 0);
    }

    #[test]
    fn test_and_ora() {

        let program = vec![
            Opcodes::LdaImmediate as u8, 0b10101010,// LDA #$AA
            Opcodes::AndImmediate as u8, 0b11001100,//AND #$CC => 0b10001000 (0x88)
            Opcodes::OraImmediate as u8, 0b00001111,// ORA #$0F => 0b10001111 (0x8F)
            Opcodes::StaZeropage as u8, 0x10,// STA $10
            Opcodes::BRK as u8,
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        let result = cpu.memory.read_byte(0x10);
        assert_eq!(result, 0x8F);
    }

    #[test]
    fn test_stack_push_pull() {

        let program = vec![
            Opcodes::LdaImmediate as u8, 0x77,   // LDA #$77
            Opcodes::Pha as u8,                  // PHA
            Opcodes::LdaImmediate as u8, 0x00,   // LDA #$00
            Opcodes::Pla as u8,                  // PLA
            Opcodes::StaZeropage as u8, 0x10,    // STA $10
            Opcodes::BRK as u8,                  // BRK
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        let val = cpu.memory.read_byte(0x10);
        assert_eq!(val, 0x77);
    }


    #[test]
    fn test_rotate() {

        let program = vec![
            Opcodes::LdaImmediate as u8, 0b10000000,  // LDA #$80
            Opcodes::RolAccumulator as u8,             // ROL A => should become 0 (carry set)
            Opcodes::StaZeropage as u8, 0x20,         // STA $20
            Opcodes::Clc as u8,                       // Clear carry flag before ROR
            Opcodes::LdaImmediate as u8, 0b00000001,  // LDA #$01
            Opcodes::RorAccumulator as u8,             // ROR A => should become 0 (carry cleared)
            Opcodes::StaZeropage as u8, 0x21,         // STA $21
            Opcodes::BRK as u8,
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        assert_eq!(cpu.memory.read_byte(0x20), 0x00);
        assert_eq!(cpu.memory.read_byte(0x21), 0x00);
    }

    #[test]
    fn test_compare_branch() {

        let program = vec![
            Opcodes::LdxImmediate as u8, 0x05,// LDX #$05
            Opcodes::CpxImmediate as u8, 0x06,// CPX #$05
            Opcodes::BneRelative as u8, 0x02,// BNE +2
            Opcodes::LdxImmediate as u8, 0x01,// LDX #$00
            Opcodes::BRK as u8,// BRK
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        assert_eq!(cpu.registers.x, 0x05);
    }

    #[test]
    fn test_beq_branch_taken() {

        let program = vec![
            Opcodes::LdxImmediate as u8, 0x03,// LDX #$03
            Opcodes::CpxImmediate as u8, 0x03,// CPX #$03
            Opcodes::BeqRelative as u8, 0x02,// BEQ
            Opcodes::LdxImmediate as u8, 0x00,// LDX #$00
            Opcodes::BRK as u8,
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        assert_eq!(cpu.registers.x, 0x03);
    }

    #[test]
    fn test_bne_branch_taken() {

        let program = vec![
            Opcodes::LdxImmediate as u8, 0x04,// LDX #$04
            Opcodes::CpxImmediate as u8, 0x05,// CPX #$05
            Opcodes::BneRelative as u8, 0x02,// BNE +2
            Opcodes::LdxImmediate as u8, 0x00,// LDX #$00
            Opcodes::BRK as u8,
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        assert_eq!(cpu.registers.x, 4);
    }

    #[test]
    fn test_adc_branch() {

        let program = vec![
            Opcodes::Cld as u8,
            Opcodes::Clc as u8,
            Opcodes::LdaImmediate as u8, 0x01,
            Opcodes::AdcImmediate as u8, 0xFE,
            Opcodes::BeqRelative as u8, 0x02,
            Opcodes::BRK as u8,// BRK
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            println!("z: {}",cpu.registers.sr.z);
            if opcode == Opcodes::BRK as u8 { break; }
        }

        assert_eq!(cpu.registers.ac, 255);
        assert!(!cpu.registers.sr.z);
    }

    #[test]
    fn test_combined_arithmetic_branch() {

        let program = vec![
            Opcodes::LdxImmediate as u8, 0x05,
            Opcodes::LdyImmediate as u8, 0x03,
            Opcodes::LdaImmediate as u8, 0x00,
            Opcodes::CpyImmediate as u8, 0x03,
            Opcodes::BeqRelative as u8, 0x04,
            Opcodes::LdaImmediate as u8, 0xFF,
            Opcodes::StaZeropage as u8, 0x20,
            Opcodes::LdaImmediate as u8, 0xAA,
            Opcodes::StaZeropage as u8, 0x21,
            Opcodes::BRK as u8,
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        assert_eq!(cpu.memory.read_byte(0x20), 0);
        assert_eq!(cpu.memory.read_byte(0x21), 0xAA);
    }

    #[test]
    fn test_conditional_swap() {

        let program = vec![
            Opcodes::LdaZeropage as u8, 0x10,
            Opcodes::CmpZeropage as u8, 0x11,
            Opcodes::BccRelative as u8, 0x06,


            Opcodes::LdxZeropage as u8, 0x11,
            Opcodes::StaZeropage as u8, 0x11,
            Opcodes::StxZeropage as u8, 0x10,

            Opcodes::BRK as u8,
        ];

        let mut cpu = setup_cpu_with_program(&program);
        cpu.memory.write_byte(0x10, 0x42);
        cpu.memory.write_byte(0x11, 0x21);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        assert_eq!(cpu.memory.read_byte(0x10), 0x21);
        assert_eq!(cpu.memory.read_byte(0x11), 0x42);
    }

    #[test]
    fn test_nested_loops() {

        let program = vec![
            Opcodes::LdxImmediate as u8, 0x02, 

            // outer
            Opcodes::LdyImmediate as u8, 0x03, 

            // inner
            Opcodes::Dey as u8,                       
            Opcodes::BneRelative as u8, 0xFD,         // go to inner

            Opcodes::Dex as u8,                       
            Opcodes::BneRelative as u8, 0xF8,         //go to outer
            Opcodes::BRK as u8,
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        assert_eq!(cpu.registers.x, 0);
        assert_eq!(cpu.registers.y, 0);
    }

    #[test]
    fn test_branch_backwards() {

        let program = vec![
            Opcodes::LdxImmediate as u8, 0x03,

            // loop
            Opcodes::Dex as u8,
            Opcodes::CpxImmediate as u8, 0x00,
            Opcodes::BneRelative as u8, 0xFB, // to loop
            Opcodes::BRK as u8,
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        assert_eq!(cpu.registers.x, 0);
    }

    #[test]
    fn test_bitwise_logic() {

        let program = vec![
            Opcodes::LdaImmediate as u8, 0b11001100,
            Opcodes::AndImmediate as u8, 0b11110000, 
            Opcodes::OraImmediate as u8, 0b00001111,
            Opcodes::EorImmediate as u8, 0b11111111,
            Opcodes::StaZeropage as u8, 0x20,
            Opcodes::BRK as u8,
        ];

        let mut cpu = setup_cpu_with_program(&program);

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        assert_eq!(cpu.memory.read_byte(0x20), 0x30);
    }

    #[test]
    fn test_sum_array() {

        let program = vec![
            Opcodes::LdxImmediate as u8, 0x00,
            Opcodes::LdyImmediate as u8, 0x05,
            Opcodes::LdaImmediate as u8, 0x00,

            // loop
            Opcodes::AdcZeropageX as u8, 0x10,
            Opcodes::Inx as u8,
            Opcodes::Dey as u8,
            Opcodes::BneRelative as u8, 0xFA,// to loop
            
            Opcodes::StaZeropage as u8, 0x20,         
            Opcodes::BRK as u8,
        ];

        let mut cpu = setup_cpu_with_program(&program);
        for i in 0..5 {
            cpu.memory.write_byte(0x10 + i, (i + 1) as u8);
        }

        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);
            cpu.step();
            if opcode == Opcodes::BRK as u8 { break; }
        }

        assert_eq!(cpu.memory.read_byte(0x20), 15);
    }
}