# Rusty-6502
Emulating a 6502 in Rust

All Original commands of the 6502 have been implemented. <br>
Test cases for all instructions also exist, while they do not test all possible outcomes, there is a at least one version for every instruction of every addressing type with pc (page crossed) and npc (no page crossed) detection.

## Format
<img width="1031" height="531" alt="6502 drawio" src="https://github.com/user-attachments/assets/fee5cf07-067c-49d9-845f-1223ddcd2d80" />

A page cross is when the upper byte of the memory address have changed for instance:
- 0x8000 to 0x8079 does not cross a page
- 0x8079 to 0x8101 does cross a page, and given some instructions, this will take an extra cycle

## Adding Programs
Below is the general outline for adding additional assembly programs to this project
```Rust
#[test] // I recommend adding additional programs as additional tests
    fn Program() {
        use Opcodes::*; //allows for the opcode name to just be typed
    
        let program = vec![ //this is where the opcodes + data go, 
            Opcode as u8, X, Y, //just write the opcode + any parameters if there are any,
            //some examples are shown below

            //remeber the 6502 is little endian, meaning that the low byte of a 16 bit value comes first in memory
            //opcode 0x00, 0x80 looks at memory address 0x8000

            AdcZeropageX as u8, 0x10,
            Inx as u8,
            BneRelative as u8, 0xFA,       
            BRK as u8,
        ];
    
        let mut cpu = setup_cpu_with_program(&program); //loads the program at pc = 0x8000, 
        //this can be changed however by modifying the reset vector
        
        loop {
            let opcode = cpu.memory.read_byte(cpu.registers.pc as u32);  //can be useful when
            //debugging to see the currnet opcode, mainly if using branching or jumps

            cpu.step(); //performs one entire opcode
            if opcode == Opcodes::BRK as u8 { break; } //break is treated as the "end program" command
        }
    
        assert_eq!(A, B); //here is where you can check to see if your program is working as expected.
        //If you just want to see the printed output from this,
        //put a statement here that will always be false, otherwise it will be hidden
    }
```
## References
[Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html) <br>
[Dave Poo's Series]()
