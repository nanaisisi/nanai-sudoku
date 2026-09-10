use crate::sudoku::Difficulty;

#[derive(Clone, PartialEq, Default)]
pub struct AppInput;

#[derive(Clone, Copy)]
pub enum RammapMessage {
    SelectCell(usize, usize),
    EnterNumber(u8),
    TogglePencil,
    ToggleContinuousPencil,
    CheckAnswers,
    SetDifficulty(Difficulty),
    NewGame,
}
