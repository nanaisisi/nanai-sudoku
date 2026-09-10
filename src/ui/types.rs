use crate::sudoku::Difficulty;

#[derive(Clone, PartialEq, Default)]
pub struct AppInput;

#[derive(Clone, Copy)]
pub enum Message {
    SelectCell(usize, usize),
    EnterNumber(u8),
    TogglePencil,
    ToggleContinuousPencil,
    CheckAnswers,
    ShowClearAllDialog,
    ConfirmClearAll,
    CancelClearAll,
    SetDifficulty(Difficulty),
    NewGame,
}
