use std::{error::Error, collections::LinkedList};

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    PutChar,
    GetChar,
    Comment,
    JumpForward(usize),
    JumpForwardPlaceholder,
    JumpBackward(usize),
}

pub fn parse(input: &str) -> Result<Vec<Instruction>, Box<dyn Error + Send>> {
    let mut open_brackets: LinkedList<usize> = LinkedList::new();
    let mut remaining_fits = LinkedList::new();
    let mut ret: Vec<Instruction> = input.char_indices().map(|e| {
        let num = e.0;
        let character = e.1;
        let instruction = match character {
            '>' => Instruction::IncrementPointer,
            '<' => Instruction::DecrementPointer,
            '+' => Instruction::IncrementValue,
            '-' => Instruction::DecrementValue,
            '.' => Instruction::PutChar,
            ',' => Instruction::GetChar,
            '[' => {
                open_brackets.push_back(num);
                Instruction::JumpForwardPlaceholder
            },
            ']' => {
                let fitting_bracket = open_brackets.pop_back().expect("Instructions are invalid");
                remaining_fits.push_back((fitting_bracket, num));
                Instruction::JumpBackward(fitting_bracket)
            },
            _ => Instruction::Comment,
        };
        instruction
    }).collect();

    remaining_fits.iter().for_each(|e| {
        let fitting_bracket_position = e.0;
        let fitting_bracket_content = e.1;
        ret[fitting_bracket_position] = Instruction::JumpForward(fitting_bracket_content);
    });
 
    Ok(ret)
}
