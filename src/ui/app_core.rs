use crate::sudoku::Sudoku;
use crate::ui::types::{AppInput, RammapMessage};

pub struct RammapApp {
    pub(super) sudoku: Sudoku,
    pub(super) pencil_mode: bool,
}

impl RammapApp {
    pub(super) fn new(_input: &AppInput) -> Self {
        Self {
            sudoku: Sudoku::new(),
            pencil_mode: false,
        }
    }

    pub(super) fn update_message(&mut self, message: RammapMessage) {
        match message {
            RammapMessage::SelectCell(row, col) => self.sudoku.select(row, col),
            RammapMessage::EnterNumber(value) => {
                if self.pencil_mode && value != 0 {
                    self.sudoku.toggle_pencil_mark(value);
                } else if value == 0 {
                    self.sudoku.clear();
                } else {
                    self.sudoku.input(value);
                }
            }
            RammapMessage::TogglePencil => self.pencil_mode = !self.pencil_mode,
            RammapMessage::SetDifficulty(difficulty) => {
                self.sudoku = Sudoku::new_with_difficulty(difficulty);
                self.pencil_mode = false;
            }
            RammapMessage::NewGame => {
                self.sudoku.new_game();
                self.pencil_mode = false;
            }
        }
    }
}
