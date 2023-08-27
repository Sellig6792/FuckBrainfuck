use super::{InstructionTrait, Pattern, PatternType};
use crate::fuckbf::ast::InstructionType;

#[derive(Clone, Debug, PartialEq)]
pub struct SetToZero {}

impl<T> Pattern<T> for SetToZero
// This pattern is Loop<Decrement(2n+1) or Loop<Increment(2n+1)
where
    T: InstructionTrait<T> + Clone,
{
    fn match_pattern(&self, instructions: &[T]) -> bool {
        if instructions.len() != 1 {
            return false;
        }

        match instructions[0].get_instruction_type() {
            InstructionType::Loop => {
                let content = instructions[0].get_content_ref();
                if content.len() != 1 {
                    return false;
                }
                match content[0].get_instruction_type() {
                    InstructionType::Decrement | InstructionType::Increment => {
                        let amount = content[0].get_amount();
                        if amount % 2 != 1 {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
            _ => return false,
        }

        true
    }

    fn replace(&self, mut instructions: Vec<T>) -> Vec<T> {
        // Find in instructions somewhere where the pattern matches
        // Replace the pattern with a SetToZero instruction
        // Remove the instructions that were replaced
        for instruction in &mut instructions {
            if self.match_pattern(&[instruction.clone()]) {
                let new_instruction = T::new(
                    InstructionType::Pattern(PatternType::SetToZero(SetToZero {})),
                    None,
                );
                *instruction = new_instruction;
            }
        }

        instructions
    }
}
