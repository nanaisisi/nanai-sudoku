use crate::sudoku::Sudoku;
use crate::ui::types::{AppInput, Message};
use windows_reactor::{ElementRef, TextBox};

pub struct App {
    pub(super) sudoku: Sudoku,
    pub(super) keyboard_ref: ElementRef<TextBox>,
    pub(super) pencil_mode: bool,
    pub(super) continuous_pencil: bool,
    pub(super) pencil_value: Option<u8>,
    pub(super) clear_all_dialog_open: bool,
}

impl App {
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

    pub(super) fn update_message(&mut self, message: Message) {
        match message {
            Message::SelectCell(row, col) => {
                self.sudoku.select(row, col);
                if self.continuous_pencil
                    && let Some(value) = self.pencil_value
                {
                    self.sudoku.toggle_pencil_mark(value);
                }
            }
            Message::EnterNumber(value) => {
                if self.pencil_mode && value != 0 {
                    if self.continuous_pencil {
                        self.pencil_value = Some(value);
                    } else {
                        self.sudoku.toggle_pencil_mark(value);
                    }
                } else if value == 0 {
                    if !self.pencil_mode {
                        self.sudoku.clear();
                    }
                } else {
                    if !self.pencil_mode {
                        self.sudoku.input(value);
                    }
                }
            }
            Message::TogglePencil => {
                self.pencil_mode = !self.pencil_mode;
                if !self.pencil_mode {
                    self.continuous_pencil = false;
                    self.pencil_value = None;
                }
            }
            Message::ToggleContinuousPencil => {
                if self.pencil_mode {
                    self.continuous_pencil = !self.continuous_pencil;
                    if !self.continuous_pencil {
                        self.pencil_value = None;
                    }
                }
            }
            Message::CheckAnswers => self.sudoku.check_answers(),
            Message::ShowClearAllDialog => {
                self.clear_all_dialog_open = true;
            }
            Message::ConfirmClearAll => {
                self.sudoku.clear_all();
                self.clear_all_dialog_open = false;
            }
            Message::CancelClearAll => {
                self.clear_all_dialog_open = false;
            }
            Message::SetDifficulty(difficulty) => {
                self.sudoku = Sudoku::new_with_difficulty(difficulty);
                self.pencil_mode = false;
                self.continuous_pencil = false;
                self.pencil_value = None;
                self.clear_all_dialog_open = false;
            }
            Message::NewGame => {
                self.sudoku.new_game();
                self.pencil_mode = false;
                self.continuous_pencil = false;
                self.pencil_value = None;
                self.clear_all_dialog_open = false;
            }
        }
    }
}
