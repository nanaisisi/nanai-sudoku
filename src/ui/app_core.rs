use crate::sudoku::Sudoku;
use crate::ui::types::{AppInput, RammapMessage};
use windows_reactor::{ElementRef, TextBox};

pub struct RammapApp {
    pub(super) sudoku: Sudoku,
    pub(super) keyboard_ref: ElementRef<TextBox>,
    pub(super) pencil_mode: bool,
    pub(super) continuous_pencil: bool,
    pub(super) pencil_value: Option<u8>,
    pub(super) clear_all_dialog_open: bool,
}

impl RammapApp {
    pub(super) fn new(_input: &AppInput) -> Self {
        Self {
            sudoku: Sudoku::new(),
            keyboard_ref: ElementRef::new(),
            pencil_mode: false,
            continuous_pencil: false,
            pencil_value: None,
            clear_all_dialog_open: false,
        }
    }

    pub(super) fn update_message(&mut self, message: RammapMessage) {
        match message {
            RammapMessage::SelectCell(row, col) => {
                self.sudoku.select(row, col);
                if self.continuous_pencil
                    && let Some(value) = self.pencil_value
                {
                    self.sudoku.toggle_pencil_mark(value);
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
                    if !self.pencil_mode
                        && matches!(self.sudoku.clear(), crate::sudoku::InputResult::Updated)
                    {
                        self.sudoku.select_next_editable();
                    }
                } else {
                    if !self.pencil_mode
                        && matches!(
                            self.sudoku.input(value),
                            crate::sudoku::InputResult::Updated
                        )
                    {
                        self.sudoku.select_next_editable();
                    }
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
            RammapMessage::ShowClearAllDialog => {
                self.clear_all_dialog_open = true;
            }
            RammapMessage::ConfirmClearAll => {
                self.sudoku.clear_all();
                self.clear_all_dialog_open = false;
            }
            RammapMessage::CancelClearAll => {
                self.clear_all_dialog_open = false;
            }
            RammapMessage::SetDifficulty(difficulty) => {
                self.sudoku = Sudoku::new_with_difficulty(difficulty);
                self.pencil_mode = false;
                self.continuous_pencil = false;
                self.pencil_value = None;
                self.clear_all_dialog_open = false;
            }
            RammapMessage::NewGame => {
                self.sudoku.new_game();
                self.pencil_mode = false;
                self.continuous_pencil = false;
                self.pencil_value = None;
                self.clear_all_dialog_open = false;
            }
        }
    }
}
