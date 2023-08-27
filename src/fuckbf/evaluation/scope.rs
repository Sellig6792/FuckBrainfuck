use crate::fuckbf::ast::{InstructionTrait, InstructionType};

use super::Cell;

#[derive(Clone)]
pub struct Scope<T: InstructionTrait<T>> {
    memory: Box<[Cell; 30000]>,
    function_memory: Box<[T; 30000]>,
}

impl<T: InstructionTrait<T>> Scope<T> {
    pub fn new() -> Scope<T> {
        let mut function_memory_vec = vec![];
        for _ in 0..30000 {
            function_memory_vec.push(T::new(InstructionType::Function, Some(vec![])));
        }

        Scope {
            memory: Box::new([Cell::new(0); 30000]),
            function_memory: match function_memory_vec.into_boxed_slice().try_into() {
                Ok(function_memory) => function_memory,
                Err(_) => panic!("Could not convert Vec to Box<[T; 30000]>"),
            },
        }
    }

    pub fn get_function(&self, index: usize) -> &T {
        &self.function_memory[index]
    }
}

pub struct Scopes<T>
where
    T: InstructionTrait<T>,
{
    index: usize,
    scopes: Vec<Scope<T>>,
    scope_index: usize,
}

impl<T> Scopes<T>
where
    T: InstructionTrait<T>,
{
    pub fn new() -> Scopes<T> {
        Scopes {
            index: 0,
            scopes: vec![Scope::new()],
            scope_index: 0,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_scope_index(&self) -> usize {
        self.scope_index
    }

    pub fn get_current_scope(&self) -> &Scope<T> {
        &self.scopes[self.scope_index]
    }

    pub fn get_current_scope_mut(&mut self) -> &mut Scope<T> {
        &mut self.scopes[self.scope_index]
    }

    pub fn get_scope_at(&self, index: usize) -> Option<&Scope<T>> {
        self.scopes.get(index)
    }

    pub fn move_right(&mut self, amount: usize) {
        let sum: usize = self.index + amount;

        self.index = if sum > 29999 { sum - 30000 } else { sum };
    }

    pub fn move_left(&mut self, amount: usize) {
        // If the index is less than 0, set the index to the last cell
        let sub: isize = self.index as isize - amount as isize;

        self.index = if sub < 0 {
            30000 - sub.unsigned_abs()
        } else {
            sub as usize
        }
    }

    pub fn move_right_scope(&mut self, amount: usize) {
        // If the index exceeds the number of scopes, set the index to the last scope
        let new_scope_pointer = self.scope_index + amount;

        self.scope_index = if new_scope_pointer >= self.scopes.len() {
            self.scopes.len() - 1
        } else {
            new_scope_pointer
        };
    }

    pub fn move_left_scope(&mut self, amount: usize) {
        // If the index is less than 0, set the index to the first scope
        let sub: isize = self.scope_index as isize - amount as isize;

        self.scope_index = if sub < 0 { 0 } else { sub as usize };
    }

    pub fn get_current_cell(&self) -> &Cell {
        self.get_cell_at(self.index).unwrap()
    }

    pub fn get_current_cell_mut(&mut self) -> &mut Cell {
        self.get_cell_at_mut(self.index).unwrap()
    }

    pub fn get_cell_at(&self, index: usize) -> Option<&Cell> {
        self.get_current_scope().memory.get(index)
    }

    pub fn get_cell_at_mut(&mut self, index: usize) -> Option<&mut Cell> {
        self.get_current_scope_mut().memory.get_mut(index)
    }

    pub fn get_current_function_mut(&mut self) -> &mut T {
        self.get_function_at_mut(self.index).unwrap()
    }

    pub fn get_function_at_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_current_scope_mut().function_memory.get_mut(index)
    }

    pub fn push(&mut self) {
        self.scopes.push(Scope::new());
        self.scope_index += 1;
    }

    pub fn pop(&mut self) {
        self.scopes.pop();
        if self.scope_index > 0 {
            self.scope_index -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fuckbf::ast::Instruction;

    #[test]
    fn test_move_right() {
        let mut scopes = Scopes::<Instruction>::new();

        scopes.move_right(1);
        assert_eq!(scopes.get_index(), 1);
    }

    #[test]
    fn test_move_left() {
        let mut scopes = Scopes::<Instruction>::new();

        scopes.index = 5;
        scopes.move_left(1);
        assert_eq!(scopes.get_index(), 4);
    }

    #[test]
    fn test_move_right_overflow() {
        let mut scopes = Scopes::<Instruction>::new();

        scopes.index = 29999;
        scopes.move_right(5);
        assert_eq!(scopes.get_index(), 4);
    }

    #[test]
    fn test_move_left_overflow() {
        let mut scopes = Scopes::<Instruction>::new();

        scopes.index = 5;
        scopes.move_left(10);
        assert_eq!(scopes.get_index(), 29995);
    }

    #[test]
    fn test_move_right_scope_if_only_one_scope() {
        let mut scopes = Scopes::<Instruction>::new();

        scopes.move_right_scope(1);
        assert_eq!(scopes.get_scope_index(), 0);
    }

    #[test]
    fn test_move_right_scope_if_multiple_scopes() {
        let mut scopes = Scopes::<Instruction>::new();

        scopes.push();
        scopes.move_left_scope(1); // Because .push() add 1 to the scope index

        scopes.move_right_scope(1);
        assert_eq!(scopes.get_scope_index(), 1);
    }

    #[test]
    fn test_move_right_scope_if_multiple_scopes_overflow() {
        let mut scopes = Scopes::<Instruction>::new();

        scopes.push();
        scopes.move_left_scope(1); // Because .push() add 1 to the scope index

        scopes.move_right_scope(2);
        assert_eq!(scopes.get_scope_index(), 1);
    }

    #[test]
    fn test_move_left_scope_if_at_first_scope() {
        let mut scopes = Scopes::<Instruction>::new();

        scopes.move_left_scope(1);
        assert_eq!(scopes.get_scope_index(), 0);
    }

    #[test]
    fn test_move_left_scope_if_multiple_scopes() {
        let mut scopes = Scopes::<Instruction>::new();

        scopes.push();
        scopes.move_left_scope(1); // Because .push() add 1 to the scope index

        scopes.move_left_scope(8);
        assert_eq!(scopes.get_scope_index(), 0);
    }
}
