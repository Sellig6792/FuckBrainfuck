mod set_to_zero;

use super::InstructionTrait;
use crate::fuckbf::optimization::OptimizedInstruction;

pub mod pattern_structs {
    pub use super::set_to_zero::SetToZero;
}

pub trait Pattern<T>
where
    T: InstructionTrait<T>,
{
    fn match_pattern(&self, instructions: &[T]) -> bool;
    fn replace(&self, instructions: Vec<T>) -> Vec<T>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatternType {
    SetToZero(set_to_zero::SetToZero),
}

impl PatternType {
    pub fn iter() -> std::slice::Iter<'static, PatternType> {
        static PATTERNS: [PatternType; 1] = [PatternType::SetToZero(set_to_zero::SetToZero {})];
        PATTERNS.iter()
    }

    pub fn get_pattern(&self) -> Box<dyn Pattern<OptimizedInstruction>> {
        match self {
            PatternType::SetToZero(pattern) => Box::new(pattern.clone()),
        }
    }

    pub fn replace(&self, instructions: Vec<OptimizedInstruction>) -> Vec<OptimizedInstruction> {
        self.get_pattern().replace(instructions)
    }
}
