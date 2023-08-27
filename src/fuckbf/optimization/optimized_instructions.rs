use crate::fuckbf::ast::{InstructionTrait, InstructionType};

#[derive(Clone)]
pub struct OptimizedInstruction {
    pub instruction_type: InstructionType,
    content: Option<Vec<OptimizedInstruction>>,
    pub amount: u32,
}

impl OptimizedInstruction {
    pub fn new(
        instruction_type: InstructionType,
        content: Option<Vec<OptimizedInstruction>>,
    ) -> OptimizedInstruction {
        OptimizedInstruction {
            instruction_type,
            content,
            amount: 1,
        }
    }

    pub fn add(&mut self, amount: u32) {
        self.amount += amount
    }

    pub fn sub(&mut self, amount: u32) {
        self.amount -= amount
    }

    pub fn set_amount(&mut self, amount: u32) {
        self.amount = amount
    }

    pub fn is_opposed(&self, other: &OptimizedInstruction) -> bool {
        match self.get_instruction_type() {
            InstructionType::Increment => {
                matches!(other.get_instruction_type(), InstructionType::Decrement)
            }
            InstructionType::Decrement => {
                matches!(other.get_instruction_type(), InstructionType::Increment)
            }
            InstructionType::MoveLeft => {
                matches!(other.get_instruction_type(), InstructionType::MoveRight)
            }
            InstructionType::MoveRight => {
                matches!(other.get_instruction_type(), InstructionType::MoveLeft)
            }
            InstructionType::MoveLeftScope => matches!(
                other.get_instruction_type(),
                InstructionType::MoveRightScope
            ),
            InstructionType::MoveRightScope => {
                matches!(other.get_instruction_type(), InstructionType::MoveLeftScope)
            }
            _ => false,
        }
    }
}

impl InstructionTrait<OptimizedInstruction> for OptimizedInstruction {
    fn new(
        instruction_type: InstructionType,
        content: Option<Vec<OptimizedInstruction>>,
    ) -> OptimizedInstruction {
        OptimizedInstruction {
            instruction_type,
            content,
            amount: 1,
        }
    }
    fn get_instruction_type(&self) -> InstructionType {
        self.instruction_type.clone()
    }

    fn get_content(&self) -> Vec<OptimizedInstruction> {
        match &self.content {
            Some(content) => content.clone(),
            None => panic!("Instruction has no content"),
        }
    }
    fn get_content_ref(&self) -> &Vec<OptimizedInstruction> {
        match &self.content {
            Some(content) => content,
            None => panic!("Instruction has no content"),
        }
    }
    fn get_content_mut(&mut self) -> &mut Vec<OptimizedInstruction> {
        match &mut self.content {
            Some(content) => content,
            None => panic!("Instruction has no content"),
        }
    }

    fn get_amount(&self) -> u32 {
        self.amount
    }
}

impl std::fmt::Debug for OptimizedInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.amount {
            1 => write!(f, "{:?}", self.instruction_type),
            _ => write!(f, "{:?}({})", self.instruction_type, self.amount),
        }
    }
}
