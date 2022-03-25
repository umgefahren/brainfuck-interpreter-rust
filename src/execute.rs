use std::io::{Read, Write};

use crate::parser::Instruction;

pub fn execute(instructions: Vec<Instruction>, input: Box<dyn std::io::Read>, output: Box<dyn std::io::Write>) {
    let mut stdin = input.bytes();
    // let mut stdout = std::io::stdout();
    let mut stdout = output;
    let mut memory: Vec<i128> = Vec::new();
    memory.push(0);
    let mut cursor = 0;
    let mut instruction_cursour = 0;
    while instruction_cursour < instructions.len() {
        let e = instructions[instruction_cursour];
        match e {
            Instruction::IncrementPointer => {
                cursor += 1;
                if memory.len() <= cursor {
                    memory.push(0);
                }
            },
            Instruction::DecrementPointer => {
                cursor -= 1;
            },
            Instruction::IncrementValue => {
                memory[cursor] += 1;
            },
            Instruction::DecrementValue => {
                memory[cursor] -= 1;
            },
            Instruction::Comment => {},
            Instruction::JumpForward(pos) => {
                if memory[cursor] == 0 {
                    instruction_cursour = pos;
                }
            },
            Instruction::JumpBackward(pos) => {
                if memory[cursor] != 0 {
                    instruction_cursour = pos;
                }
            },
            Instruction::GetChar => {
                let read_char = stdin.next().unwrap().unwrap() as i128;
                memory[cursor] = read_char;
            },
            Instruction::PutChar => {
                let write_u8 = memory[cursor] as u8;
                stdout.write(&[write_u8]).unwrap();
            },
            Instruction::JumpForwardPlaceholder => {
                panic!("This instruction shouldn't be here");
            }
        }
        instruction_cursour += 1;
    }
}
