use super::PatternType;

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Increment,
    Decrement,

    MoveLeft,
    MoveRight,

    Input,
    Output,

    Loop,

    Function,
    CallFunction,

    MoveLeftScope,
    MoveRightScope,

    Random,

    Pattern(PatternType),
}
