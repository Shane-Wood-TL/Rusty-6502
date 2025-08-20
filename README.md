# Rusty-6502
Emulating a 6502 in Rust

All Original commands of the 6502 have been implemented. <br>
Test cases for all instructions also exist, while they do not test all possible outcomes, there is a at least one version for every instruction of every addressing type with pc (page crossed) and npc (no page crossed) detection.

## Format
<img width="1031" height="531" alt="6502 drawio" src="https://github.com/user-attachments/assets/fee5cf07-067c-49d9-845f-1223ddcd2d80" />

A page cross is when the upper byte of the memory address have changed for instance:
- 0x8000 to 0x8079 does not cross a page
- 0x8079 to 0x8101 does cross a page, and given some instructions, this will take an extra cycle

## References
[Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html) <br>
[Dave Poo's Series]()
