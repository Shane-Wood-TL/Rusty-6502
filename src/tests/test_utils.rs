use crate::Cpu6502;
    


pub fn setup_cpu_with_program(program: &[u8]) -> Cpu6502 {
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
    