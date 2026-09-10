use crate::sudoku::Sudoku;
use crate::ui::types::{AppInput, RammapMessage};

pub struct RammapApp {
    pub(super) sudoku: Sudoku,
    pub(super) pencil_mode: bool,
    pub(super) continuous_pencil: bool,
    pub(super) pencil_value: Option<u8>,
}

impl RammapApp {
    pub(super) fn new(_input: &AppInput) -> Self {
        Self {
            sudoku: Sudoku::new(),
            pencil_mode: false,
            continuous_pencil: false,
            pencil_value: None,
        }
    }

    pub(super) fn update_message(&mut self, message: RammapMessage) {
        match message {
            RammapMessage::SelectCell(row, col) => {
                self.sudoku.select(row, col);
                if self.continuous_pencil {
                    if let Some(value) = self.pencil_value {
                        self.sudoku.toggle_pencil_mark(value);
                    }
                }
            }
            RammapMessage::EnterNumber(value) => {
                if self.pencil_mode && value != 0 {
                    if self.continuous_pencil {
                        self.pencil_value = Some(value);
                    } else {
                        self.sudoku.toggle_pencil_mark(value);
                    }
                } else if value == 0 {
                    self.sudoku.clear();
                } else {
                    self.sudoku.input(value);
                }
            }
            RammapMessage::TogglePencil => {
                self.pencil_mode = !self.pencil_mode;
                if !self.pencil_mode {
                    self.continuous_pencil = false;
                    self.pencil_value = None;
                }
            }
            RammapMessage::ToggleContinuousPencil => {
                if self.pencil_mode {
                    self.continuous_pencil = !self.continuous_pencil;
                    if !self.continuous_pencil {
                        self.pencil_value = None;
                    }
                }
            }
            RammapMessage::CheckAnswers => self.sudoku.check_answers(),
            RammapMessage::SetDifficulty(difficulty) => {
                self.sudoku = Sudoku::new_with_difficulty(difficulty);
                self.pencil_mode = false;
                self.continuous_pencil = false;
                self.pencil_value = None;
            }
            RammapMessage::NewGame => {
                self.sudoku.new_game();
                self.pencil_mode = false;
                self.continuous_pencil = false;
                self.pencil_value = None;
            }
        }
    }
}
