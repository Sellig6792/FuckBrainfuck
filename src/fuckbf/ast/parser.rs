use crate::fuckbf::ast::{Instruction, InstructionType};

pub struct Parser {
    program: String,
}

impl Parser {
    pub fn new(program: String) -> Parser {
        let program = program.replace([' ', '\t', '\r', '\n'], "");

        Parser { program }
    }

    pub fn parse(&mut self) -> Vec<Instruction> {
        let (instructions, _) = self._parse(None, None);
        instructions
    }

    fn _parse(&self, index: Option<usize>, stop_char: Option<char>) -> (Vec<Instruction>, usize) {
        let mut index = index.unwrap_or(0);
        let mut instructions = vec![];
        let mut comment = false;

        while index < self.program.len() {
            let char = match self.program.chars().nth(index) {
                Some(char) => char,
                None => return (instructions, index),
            };
            if stop_char.is_some() && char == stop_char.unwrap() {
                return (instructions, index);
            }

            if char == '#' {
                comment = !comment;
            }

            if comment {
                index += 1;
                continue;
            }

            match char {
                '+' => instructions.push(Instruction::new(InstructionType::Increment, None)),
                '-' => instructions.push(Instruction::new(InstructionType::Decrement, None)),

                '<' => instructions.push(Instruction::new(InstructionType::MoveLeft, None)),
                '>' => instructions.push(Instruction::new(InstructionType::MoveRight, None)),

                '.' => instructions.push(Instruction::new(InstructionType::Output, None)),
                ',' => instructions.push(Instruction::new(InstructionType::Input, None)),

                '[' => {
                    let (loop_instructions, new_index) = self._parse(Some(index + 1), Some(']'));
                    instructions.push(Instruction::new(
                        InstructionType::Loop,
                        Some(loop_instructions),
                    ));
                    index = new_index;
                }
                '{' => {
                    let (function_instructions, new_index) =
                        self._parse(Some(index + 1), Some('}'));
                    instructions.push(Instruction::new(
                        InstructionType::Function,
                        Some(function_instructions),
                    ));
                    index = new_index;
                }

                '=' => instructions.push(Instruction::new(InstructionType::CallFunction, None)),

                '?' => instructions.push(Instruction::new(InstructionType::Random, None)),

                '´' => instructions.push(Instruction::new(InstructionType::MoveLeftScope, None)),
                '`' => instructions.push(Instruction::new(InstructionType::MoveRightScope, None)),

                ':' => {
                    // The end of the comment is ';'
                    // Jump to the end of the comment
                    while index < self.program.len() {
                        let char = match self.program.chars().nth(index) {
                            Some(char) => char,
                            None => return (instructions, index),
                        };
                        if char == ';' {
                            break;
                        }
                        index += 1;
                    }
                }

                _ => {}
            }

            index += 1;
        }

        (instructions, index)
    }
}
