use crate::fuckbf::ast::InstructionType;

pub trait InstructionTrait<T> {
    fn new(instruction_type: InstructionType, content: Option<Vec<T>>) -> Self;
    fn get_instruction_type(&self) -> InstructionType;
    fn get_content(&self) -> Vec<T>;
    fn get_content_ref(&self) -> &Vec<T>;
    fn get_content_mut(&mut self) -> &mut Vec<T>;
    fn get_amount(&self) -> u32;
}

#[derive(Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    content: Option<Vec<Instruction>>,
}

impl Instruction {
    pub fn new(
        instruction_type: InstructionType,
        content: Option<Vec<Instruction>>,
    ) -> Instruction {
        Instruction {
            instruction_type,
            content,
        }
    }
}

impl InstructionTrait<Instruction> for Instruction {
    fn new(instruction_type: InstructionType, content: Option<Vec<Instruction>>) -> Instruction {
        Instruction {
            instruction_type,
            content,
        }
    }
    fn get_instruction_type(&self) -> InstructionType {
        self.instruction_type.clone()
    }

    fn get_content(&self) -> Vec<Instruction> {
        match &self.content {
            Some(content) => content.clone(),
            None => panic!("Instruction has no content"),
        }
    }
    fn get_content_ref(&self) -> &Vec<Instruction> {
        match &self.content {
            Some(content) => content,
            None => panic!("Instruction has no content"),
        }
    }
    fn get_content_mut(&mut self) -> &mut Vec<Instruction> {
        match &mut self.content {
            Some(content) => content,
            None => panic!("Instruction has no content"),
        }
    }
    fn get_amount(&self) -> u32 {
        1
    }
}
