mod cpu;
use crate::cpu::opcodes::Opcodes;
use crate::cpu::memory::Memory;
use crate::cpu::status_registers::StatusRegisterValues;
use crate::cpu::registers_6502::Registers6502;
use crate::cpu::cpu_6502::Cpu6502;



fn main(){
    let mut cpu = Cpu6502::new();

    // Set reset vector to $8000
    cpu.memory.write_byte(0xFFFC, 0x00);
    cpu.memory.write_byte(0xFFFD, 0x80);

    cpu.reset();
}





#[cfg(test)]
mod tests;