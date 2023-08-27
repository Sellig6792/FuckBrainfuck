use rand::Rng;

use crate::fuckbf::ast::{InstructionTrait, InstructionType, PatternType};

use super::{Cell, Scopes};

pub struct Evaluator<T: InstructionTrait<T>>
where
    T: Clone,
{
    program: Vec<T>,

    scopes: Scopes<T>,

    input: Vec<u8>,
    output_buffer: Vec<u8>,
}

impl<T: InstructionTrait<T> + 'static> Evaluator<T>
where
    T: Clone,
{
    pub fn new(instructions: Vec<T>) -> Evaluator<T> {
        Evaluator {
            program: instructions,

            scopes: Scopes::new(),

            input: vec![],
            output_buffer: vec![],
        }
    }

    pub fn evaluate(&mut self, container_to_execute: Option<T>, show_output: Option<bool>) {
        let instructions = match container_to_execute {
            Some(container) => container.get_content(),
            None => self.program.clone(),
        };

        for instruction in instructions.iter() {
            match &instruction.get_instruction_type() {
                InstructionType::Increment => {
                    self.scopes
                        .get_current_cell_mut()
                        .add(instruction.get_amount());
                }
                InstructionType::Decrement => {
                    self.scopes
                        .get_current_cell_mut()
                        .sub(instruction.get_amount());
                }

                InstructionType::MoveLeft => {
                    self.scopes.move_left(instruction.get_amount() as usize);
                }
                InstructionType::MoveRight => {
                    self.scopes.move_right(instruction.get_amount() as usize);
                }

                InstructionType::Input => {
                    for _ in 0..instruction.get_amount() {
                        if self.input.is_empty() {
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            // Convert the input to a vector of u8
                            self.input = input.trim().bytes().collect();
                        }

                        if self.input.is_empty() {
                            self.scopes.get_current_cell_mut().set_value(0);
                        } else {
                            self.scopes
                                .get_current_cell_mut()
                                .set_value(self.input.remove(0));
                        }
                    }
                }
                InstructionType::Output => {
                    for _ in 0..instruction.get_amount() {
                        self.output_buffer
                            .push(self.scopes.get_current_cell().get_value());
                    }
                }

                InstructionType::Loop => {
                    while *self.scopes.get_current_cell() != 0 {
                        self.evaluate(Some(instruction.clone()), Some(false));
                    }
                }
                InstructionType::Function => {
                    *self.scopes.get_current_function_mut() =
                        T::new(InstructionType::Function, Some(instruction.get_content()));
                }

                InstructionType::CallFunction => {
                    for _ in 0..instruction.get_amount() {
                        self.scopes.push();
                        self.evaluate(
                            Some(
                                self.scopes
                                    .get_scope_at(self.scopes.get_scope_index() - 1)
                                    .unwrap()
                                    .get_function(self.scopes.get_index())
                                    .clone(),
                            ),
                            Some(false),
                        );
                        self.scopes.pop();
                    }
                }

                InstructionType::MoveLeftScope => {
                    self.scopes
                        .move_left_scope(instruction.get_amount() as usize);
                }
                InstructionType::MoveRightScope => {
                    self.scopes
                        .move_right_scope(instruction.get_amount() as usize);
                }

                InstructionType::Random => {
                    /*
                    Generate a random number between the left cell's value and the right cell's value (including both)

                    If the left cell's value is greater than the right cell's value,
                    generate a random number between the left cell's value and 255 and the right cell's value and 0
                     */
                    let left: &Cell = self
                        .scopes
                        .get_cell_at(self.scopes.get_index() - 1)
                        .unwrap();
                    let right: &Cell = self
                        .scopes
                        .get_cell_at(self.scopes.get_index() + 1)
                        .unwrap();

                    if right > left {
                        let r = rand::thread_rng().gen_range(left.get_value()..=right.get_value());
                        self.scopes.get_current_cell_mut().set_value(r);
                    } else {
                        let left_to_255: u8 = rand::thread_rng().gen_range(left.get_value()..=255);
                        let _0_to_right: u8 = rand::thread_rng().gen_range(0..=right.get_value());

                        let left_to_255_or_right_to_0: u8 = rand::thread_rng().gen_range(0..=1);

                        if left_to_255_or_right_to_0 == 0 {
                            self.scopes.get_current_cell_mut().set_value(left_to_255);
                        } else {
                            self.scopes.get_current_cell_mut().set_value(_0_to_right);
                        }
                    }
                }

                InstructionType::Pattern(pattern_type) => match pattern_type {
                    PatternType::SetToZero(_) => {
                        self.scopes.get_current_cell_mut().set_value(0);
                    }
                },
            }
        }

        match show_output {
            None | Some(true) => {
                println!("{}", String::from_utf8(self.output_buffer.clone()).unwrap())
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fuckbf::{ast, optimization};

    #[test]
    fn test_hello_world() {
        let program = String::from("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
        let mut parser = ast::Parser::new(program);
        let instructions = parser.parse();
        let mut optimizer = optimization::Optimizer::new(instructions);
        let optimized_instructions = optimizer.optimize();
        let mut brainfuck = Evaluator::new(optimized_instructions);
        brainfuck.evaluate(None, Some(false));

        assert_eq!(
            String::from_utf8(brainfuck.output_buffer).unwrap(),
            "Hello World!\n"
        );
    }

    #[test]
    fn test_override_function() {
        let program = String::from("{++++[>++++++++++++<-]>.}{++++[>++++++++++++<-]>+.}=");
        let mut parser = ast::Parser::new(program);
        let instructions = parser.parse();
        let mut optimizer = optimization::Optimizer::new(instructions);
        let optimized_instructions = optimizer.optimize();
        let mut brainfuck = Evaluator::new(optimized_instructions);
        brainfuck.evaluate(None, Some(false));
        assert_eq!(String::from_utf8(brainfuck.output_buffer).unwrap(), "1");
    }

    #[test]
    fn test_call_multiple_time_functions() {
        let program = String::from("{+++++[>+++++++++++++<-]>.<}==");
        let mut parser = ast::Parser::new(program);
        let instructions = parser.parse();
        let mut optimizer = optimization::Optimizer::new(instructions);
        let optimized_instructions = optimizer.optimize();
        let mut brainfuck = Evaluator::new(optimized_instructions);
        brainfuck.evaluate(None, Some(false));
        assert_eq!(String::from_utf8(brainfuck.output_buffer).unwrap(), "AA");
    }

    #[test]
    fn test_overflow_scope_pop() {
        let program = String::from("{´}=");
        let mut parser = ast::Parser::new(program);
        let instructions = parser.parse();
        let mut evaluator = Evaluator::new(instructions);
        evaluator.evaluate(None, Some(false));
    }

    #[test]
    fn test_comments() {
        let program = String::from("++++++[>++++++++<-]>#+#.");
        let mut parser = ast::Parser::new(program);
        let instructions = parser.parse();
        let mut evaluator = Evaluator::new(instructions);
        evaluator.evaluate(None, Some(false));
        assert_eq!(String::from_utf8(evaluator.output_buffer).unwrap(), "0");
    }
}
