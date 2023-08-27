use std::cmp::Ordering;

use crate::fuckbf::ast::{Instruction, InstructionTrait, InstructionType, PatternType};

use super::OptimizedInstruction;

pub struct Optimizer {
    instructions: Vec<Instruction>,
}

impl Optimizer {
    pub fn new(instructions: Vec<Instruction>) -> Optimizer {
        Optimizer { instructions }
    }

    pub fn optimize(&mut self) -> Vec<OptimizedInstruction> {
        let mut optimized_instructions = vec![];
        self.merge_instructions(&mut optimized_instructions);
        self.cancel_opposed_instructions(&mut optimized_instructions);
        self.recognize_patterns(&mut optimized_instructions);
        optimized_instructions
    }

    fn optimize_container(&self, content: Vec<Instruction>) -> Vec<OptimizedInstruction> {
        let mut optimizer = Optimizer::new(content);
        optimizer.optimize()
    }

    fn merge_instructions(&self, optimized_instructions: &mut Vec<OptimizedInstruction>) {
        for instruction in self.instructions.iter() {
            match optimized_instructions.last_mut() {
                Some(last_optimized_instruction) => {
                    if last_optimized_instruction.get_instruction_type()
                        == instruction.get_instruction_type()
                    {
                        match instruction.get_instruction_type() {
                            InstructionType::Function => {
                                optimized_instructions.pop();
                                optimized_instructions.push(OptimizedInstruction::new(
                                    instruction.get_instruction_type(),
                                    Some(self.optimize_container(instruction.get_content())),
                                ));
                            }
                            _ => last_optimized_instruction.add(1),
                        }
                    } else {
                        match instruction.get_instruction_type() {
                            InstructionType::Function | InstructionType::Loop => {
                                optimized_instructions.push(OptimizedInstruction::new(
                                    instruction.get_instruction_type(),
                                    Some(
                                        self.optimize_container(instruction.get_content().clone()),
                                    ),
                                ));
                            }
                            _ => {
                                optimized_instructions.push(OptimizedInstruction::new(
                                    instruction.get_instruction_type(),
                                    None,
                                ));
                            }
                        }
                    }
                }
                None => match instruction.instruction_type {
                    InstructionType::Function | InstructionType::Loop => {
                        optimized_instructions.push(OptimizedInstruction::new(
                            instruction.get_instruction_type(),
                            Some(self.optimize_container(instruction.get_content().clone())),
                        ));
                    }
                    _ => optimized_instructions.push(OptimizedInstruction::new(
                        instruction.get_instruction_type(),
                        None,
                    )),
                },
            }
        }
    }

    fn cancel_opposed_instructions(&self, optimized_instructions: &mut Vec<OptimizedInstruction>) {
        let mut new_optimized_instructions: Vec<OptimizedInstruction> = vec![];

        for optimized_instruction in optimized_instructions.iter() {
            match new_optimized_instructions.last() {
                Some(last_optimized_instruction) => {
                    if last_optimized_instruction.is_opposed(optimized_instruction) {
                        let last_amount = last_optimized_instruction.get_amount();
                        let current_amount = optimized_instruction.get_amount();

                        match last_amount.cmp(&current_amount) {
                            Ordering::Greater => {
                                new_optimized_instructions
                                    .last_mut()
                                    .expect("Error while getting last optimized instruction")
                                    .sub(current_amount);
                            }
                            Ordering::Less => {
                                new_optimized_instructions.pop();
                                new_optimized_instructions.push(OptimizedInstruction::new(
                                    optimized_instruction.get_instruction_type(),
                                    None,
                                ));
                                new_optimized_instructions
                                    .last_mut()
                                    .expect("Error while getting last optimized instruction")
                                    .set_amount(current_amount - last_amount);
                            }
                            Ordering::Equal => {
                                new_optimized_instructions.pop();
                            }
                        }
                    } else if last_optimized_instruction.get_instruction_type()
                        == optimized_instruction.get_instruction_type()
                    {
                        // If the current instruction is the same as the last one, we add the amount of the current instruction to the last one
                        new_optimized_instructions
                            .last_mut()
                            .expect(
                                "Error while getting last element of new_optimized_instructions",
                            )
                            .add(optimized_instruction.amount);
                    } else {
                        new_optimized_instructions.push(optimized_instruction.clone());
                    }
                }

                None => new_optimized_instructions.push(optimized_instruction.clone()),
            }
        }

        *optimized_instructions = new_optimized_instructions;
    }

    fn recognize_patterns(&self, optimized_instructions: &mut Vec<OptimizedInstruction>) {
        let mut new_optimized_instructions: Vec<OptimizedInstruction> =
            optimized_instructions.clone();

        let patterns = PatternType::iter();

        for pattern in patterns {
            new_optimized_instructions = pattern.replace(optimized_instructions.clone());
        }

        *optimized_instructions = new_optimized_instructions;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fuckbf::ast::{pattern_structs, InstructionType, PatternType};

    #[test]
    fn test_merge_instructions() {
        let instructions = vec![
            Instruction::new(InstructionType::Increment, None),
            Instruction::new(InstructionType::Increment, None),
            Instruction::new(InstructionType::Increment, None),
            Instruction::new(InstructionType::MoveLeft, None),
            Instruction::new(InstructionType::MoveLeft, None),
            Instruction::new(InstructionType::Decrement, None),
        ];
        let mut optimizer = Optimizer::new(instructions);
        let optimized_instructions = optimizer.optimize();

        assert_eq!(optimized_instructions.len(), 3);
        assert_eq!(optimized_instructions[0].get_amount(), 3);
        assert_eq!(optimized_instructions[1].get_amount(), 2);
        assert_eq!(optimized_instructions[2].get_amount(), 1);
    }

    #[test]
    fn test_cancel_opposed_instructions_same_amount() {
        let instructions = vec![
            Instruction::new(InstructionType::Increment, None),
            Instruction::new(InstructionType::Decrement, None),
        ];
        let mut optimizer = Optimizer::new(instructions);
        let optimized_instructions = optimizer.optimize();

        assert_eq!(optimized_instructions.len(), 0);
    }

    #[test]
    fn test_cancel_opposed_instructions_left_bigger() {
        let instructions = vec![
            Instruction::new(InstructionType::Increment, None),
            Instruction::new(InstructionType::Increment, None),
            Instruction::new(InstructionType::Decrement, None),
        ];
        let mut optimizer = Optimizer::new(instructions);
        let optimized_instructions = optimizer.optimize();

        assert_eq!(optimized_instructions.len(), 1);
        assert_eq!(
            optimized_instructions[0].get_instruction_type(),
            InstructionType::Increment
        );
        assert_eq!(optimized_instructions[0].get_amount(), 1);
    }

    #[test]
    fn test_cancel_opposed_instructions_right_bigger() {
        let instructions = vec![
            Instruction::new(InstructionType::Increment, None),
            Instruction::new(InstructionType::Decrement, None),
            Instruction::new(InstructionType::Decrement, None),
        ];
        let mut optimizer = Optimizer::new(instructions);
        let optimized_instructions = optimizer.optimize();

        assert_eq!(optimized_instructions.len(), 1);
        assert_eq!(
            optimized_instructions[0].get_instruction_type(),
            InstructionType::Decrement
        );
        assert_eq!(optimized_instructions[0].get_amount(), 1);
    }

    #[test]
    fn test_recognize_pattern_set_to_zero_with_decrement() {
        for i in (1..5).step_by(2) {
            let mut loop_content = vec![];
            for _ in 0..i {
                loop_content.push(Instruction::new(InstructionType::Decrement, None));
            }

            let instructions = vec![Instruction::new(InstructionType::Loop, Some(loop_content))];

            let mut optimizer = Optimizer::new(instructions);
            let optimized_instructions = optimizer.optimize();

            assert_eq!(optimized_instructions.len(), 1);
            assert_eq!(
                optimized_instructions[0].get_instruction_type(),
                InstructionType::Pattern(PatternType::SetToZero(pattern_structs::SetToZero {}))
            );
        }
    }

    #[test]
    fn test_recognize_pattern_set_to_zero_with_increment() {
        for i in (1..5).step_by(2) {
            let mut loop_content = vec![];
            for _ in 0..i {
                loop_content.push(Instruction::new(InstructionType::Increment, None));
            }

            let instructions = vec![Instruction::new(InstructionType::Loop, Some(loop_content))];

            let mut optimizer = Optimizer::new(instructions);
            let optimized_instructions = optimizer.optimize();

            assert_eq!(optimized_instructions.len(), 1);
            assert_eq!(
                optimized_instructions[0].get_instruction_type(),
                InstructionType::Pattern(PatternType::SetToZero(pattern_structs::SetToZero {}))
            );
        }
    }

    #[test]
    fn test_do_not_recognize_pattern_set_to_zero_with_decrement() {
        for i in (2..5).step_by(2) {
            let mut loop_content = vec![];
            for _ in 0..i {
                loop_content.push(Instruction::new(InstructionType::Decrement, None));
            }

            let instructions = vec![Instruction::new(InstructionType::Loop, Some(loop_content))];

            let mut optimizer = Optimizer::new(instructions);
            let optimized_instructions = optimizer.optimize();

            assert_eq!(optimized_instructions.len(), 1);
            assert_eq!(
                optimized_instructions[0].get_instruction_type(),
                InstructionType::Loop
            );
        }
    }

    #[test]
    fn test_do_not_recognize_pattern_set_to_zero_with_increment() {
        for i in (2..5).step_by(2) {
            let mut loop_content = vec![];
            for _ in 0..i {
                loop_content.push(Instruction::new(InstructionType::Increment, None));
            }

            let instructions = vec![Instruction::new(InstructionType::Loop, Some(loop_content))];

            let mut optimizer = Optimizer::new(instructions);
            let optimized_instructions = optimizer.optimize();

            assert_eq!(optimized_instructions.len(), 1);
            assert_eq!(
                optimized_instructions[0].get_instruction_type(),
                InstructionType::Loop
            );
        }
    }
}
