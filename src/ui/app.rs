use std::cell::RefCell;
use std::rc::Rc;

use crate::sudoku::Difficulty;
use crate::ui::types::{AppInput, RammapMessage};
use windows_reactor::*;

pub use crate::ui::app_core::RammapApp;

impl Component for RammapApp {
    type Input = AppInput;
    type Message = RammapMessage;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self::new(input)
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        self.update_message(message);
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let sender = context.sender();
        let send = {
            let sender = sender.clone();
            move |message| {
                sender.send(message);
            }
        };
        let keys = [
            (AcceleratorKey::NumberPad0, 0),
            (AcceleratorKey::NumberPad1, 1),
            (AcceleratorKey::NumberPad2, 2),
            (AcceleratorKey::NumberPad3, 3),
            (AcceleratorKey::NumberPad4, 4),
            (AcceleratorKey::NumberPad5, 5),
            (AcceleratorKey::NumberPad6, 6),
            (AcceleratorKey::NumberPad7, 7),
            (AcceleratorKey::NumberPad8, 8),
            (AcceleratorKey::NumberPad9, 9),
        ];
        let mut accelerators = keys
            .into_iter()
            .map(|(key, value)| {
                KeyAccelerator::new(
                    key,
                    AcceleratorModifiers::None,
                    context.message(RammapMessage::EnterNumber(value)),
                )
            })
            .collect::<Vec<_>>();
        accelerators.push(KeyAccelerator::new(
            AcceleratorKey::Subtract,
            AcceleratorModifiers::None,
            context.message(RammapMessage::EnterNumber(0)),
        ));
        accelerators.push(KeyAccelerator::new(
            AcceleratorKey::Decimal,
            AcceleratorModifiers::None,
            context.message(RammapMessage::EnterNumber(0)),
        ));
        self.sudoku_view(send, KeyAccelerators::new(accelerators))
    }
}

impl RammapApp {
    fn sudoku_view<S: Fn(RammapMessage) + Clone + 'static>(
        &self,
        send: S,
        accelerators: KeyAccelerators,
    ) -> View {
        let mut rows = Vec::new();
        for row in 0..9 {
            let mut cells = Vec::new();
            for col in 0..9 {
                cells.push(KeyedView::new(col, self.cell(row, col, send.clone())));
            }
            rows.push(KeyedView::new(
                row,
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(2.0)
                    .keyed_children(cells),
            ));
        }

        let mut keypad = Vec::new();
        for value in 1..=9 {
            let input_send = send.clone();
            let active = self.continuous_pencil && self.pencil_value == Some(value);
            keypad.push(KeyedView::new(
                value as usize,
                Button::new()
                    .width(42.0)
                    .height(42.0)
                    .on_click(move || input_send(RammapMessage::EnterNumber(value)))
                    .content(
                        TextBlock::new()
                            .text(if active {
                                format!("●{}", value)
                            } else {
                                value.to_string()
                            })
                            .font_size(18.0),
                    ),
            ));
        }
        let clear_send = send.clone();
        keypad.push(KeyedView::new(
            0usize,
            Button::new()
                .width(72.0)
                .height(42.0)
                .on_click(move || clear_send(RammapMessage::EnterNumber(0)))
                .content(TextBlock::new().text("消去")),
        ));

        let pencil_send = send.clone();
        keypad.push(KeyedView::new(
            10usize,
            Button::new()
                .width(92.0)
                .height(42.0)
                .on_click(move || pencil_send(RammapMessage::TogglePencil))
                .content(TextBlock::new().text(if self.pencil_mode {
                    "下書き ON"
                } else {
                    "下書き"
                })),
        ));

        let continuous_send = send.clone();
        keypad.push(KeyedView::new(
            12usize,
            Button::new()
                .width(116.0)
                .height(42.0)
                .on_click(move || continuous_send(RammapMessage::ToggleContinuousPencil))
                .content(TextBlock::new().text(if self.continuous_pencil {
                    "連続下書き ON"
                } else {
                    "連続下書き"
                })),
        ));

        let check_send = send.clone();
        keypad.push(KeyedView::new(
            11usize,
            Button::new()
                .width(104.0)
                .height(42.0)
                .on_click(move || check_send(RammapMessage::CheckAnswers))
                .content(TextBlock::new().text("答え合わせ")),
        ));

        let keyboard_send = send.clone();
        let previous_text = Rc::new(RefCell::new(String::new()));
        let previous_text_state = previous_text.clone();
        let keyboard_input = TextBox::new()
            .width(220.0)
            .text("")
            .placeholder_text("ここをクリックして数字キーで入力（テンキー対応）")
            .on_text_changed(move |text: String| {
                let previous = previous_text_state.borrow().clone();
                let new_text = if text.starts_with(&previous) {
                    &text[previous.len()..]
                } else {
                    &text
                };
                for value in new_text
                    .chars()
                    .filter_map(|character| character.to_digit(10))
                {
                    keyboard_send(RammapMessage::EnterNumber(value as u8));
                }
                *previous_text_state.borrow_mut() = text;
            });

        let mut difficulty_buttons = Vec::new();
        for (index, difficulty) in Difficulty::ALL.iter().copied().enumerate() {
            let difficulty_send = send.clone();
            let selected = self.sudoku.difficulty() == difficulty;
            difficulty_buttons.push(KeyedView::new(
                index,
                Button::new()
                    .width(100.0)
                    .height(38.0)
                    .on_click(move || difficulty_send(RammapMessage::SetDifficulty(difficulty)))
                    .content(TextBlock::new().text(if selected {
                        format!("● {}", difficulty.label())
                    } else {
                        difficulty.label().to_string()
                    })),
            ));
        }

        let new_send = send.clone();
        let pencil_surface = self.pencil_mode;
        let mode_banner = Border::new()
            .padding(Thickness::xy(12.0, 8.0))
            .background(Brush::Solid(if pencil_surface {
                Color::argb(255, 92, 72, 32)
            } else {
                Color::argb(255, 43, 46, 54)
            }))
            .border_brush(Brush::Solid(if pencil_surface {
                Color::argb(255, 218, 170, 70)
            } else {
                Color::argb(255, 105, 109, 122)
            }))
            .border_thickness(Thickness::uniform(1.0))
            .corner_radius(CornerRadius::uniform(4.0))
            .content(
                TextBlock::new()
                    .text(if self.pencil_mode {
                        if self.continuous_pencil {
                            "✎ 連続下書き — 数字を選んでセルを順にクリック"
                        } else {
                            "✎ 下書きモード — 候補数字を入力中"
                        }
                    } else {
                        "数字入力モード"
                    })
                    .font_size(14.0)
                    .font_weight(if self.pencil_mode {
                        FontWeight::BOLD
                    } else {
                        FontWeight::NORMAL
                    }),
            );
        let solving_methods = self
            .sudoku
            .difficulty()
            .solving_methods()
            .iter()
            .enumerate()
            .map(|(index, method)| {
                KeyedView::new(
                    index,
                    TextBlock::new()
                        .text(format!("・{}", method))
                        .font_size(13.0),
                )
            })
            .collect::<Vec<_>>();
        let status = if self.sudoku.is_completed() {
            "🎉 クリア！おめでとうございます"
        } else if self.sudoku.selected().is_some() {
            let (row, col) = self.sudoku.selected().unwrap();
            if self.sudoku.is_wrong(row, col) {
                "❌ 不正解です — 別の数字を試してください"
            } else if self.pencil_mode {
                if self.continuous_pencil {
                    "連続下書き — 数字を選んで、複数のセルをクリックできます"
                } else {
                    "下書きモード — 数字ボタンで候補を追加・削除できます"
                }
            } else {
                "セルを選択中 — 数字ボタンで入力できます"
            }
        } else {
            "空いているセルを選択してください"
        };

        Grid::new()
            .key_accelerators(accelerators)
            .children((ScrollViewer::new().content(
                StackPanel::new()
                    .spacing(12.0)
                    .margin(Thickness::uniform(16.0))
                    .children((
                        TextBlock::new()
                            .text(if self.pencil_mode {
                                "Nanai Sudoku  —  下書きモード"
                            } else {
                                "Nanai Sudoku"
                            })
                            .font_size(30.0)
                            .font_weight(FontWeight::BOLD),
                        mode_banner,
                        TextBlock::new()
                            .text("9×9 ナンプレ（入力欄をクリックして数字キーで入力できます）")
                            .font_size(14.0),
                        TextBlock::new()
                            .text(format!(
                                "難易度: {}（{}・ヒント{}・入力{}マス）",
                                self.sudoku.difficulty().label(),
                                self.sudoku.difficulty().description(),
                                self.sudoku.difficulty().clue_count(),
                                entered_count(&self.sudoku),
                            ))
                            .font_size(14.0),
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(8.0)
                            .keyed_children(difficulty_buttons),
                        TextBlock::new().text("解法リスト").font_size(15.0),
                        StackPanel::new()
                            .spacing(2.0)
                            .keyed_children(solving_methods),
                        Border::new()
                            .padding(Thickness::uniform(8.0))
                            .background(Brush::Solid(if self.pencil_mode {
                                Color::argb(255, 58, 50, 34)
                            } else {
                                Color::argb(255, 32, 34, 40)
                            }))
                            .corner_radius(CornerRadius::uniform(6.0))
                            .content(StackPanel::new().spacing(2.0).keyed_children(rows)),
                        keyboard_input,
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(8.0)
                            .keyed_children(keypad),
                        TextBlock::new().text(status).font_size(14.0),
                        Button::new()
                            .width(150.0)
                            .on_click(move || new_send(RammapMessage::NewGame))
                            .content(TextBlock::new().text("新しいゲーム")),
                    )),
            ),))
    }

    fn cell<S: Fn(RammapMessage) + Clone + 'static>(
        &self,
        row: usize,
        col: usize,
        send: S,
    ) -> View {
        let value = self.sudoku.cells()[row][col];
        let selected = self.sudoku.selected() == Some((row, col));
        let same_box = self
            .sudoku
            .selected()
            .is_some_and(|(selected_row, selected_col)| {
                selected_row / 3 == row / 3 && selected_col / 3 == col / 3
            });
        let same_row = self
            .sudoku
            .selected()
            .is_some_and(|(selected_row, _)| selected_row == row);
        let same_col = self
            .sudoku
            .selected()
            .is_some_and(|(_, selected_col)| selected_col == col);
        let conflict = self.sudoku.has_conflict(row, col);
        let wrong = self.sudoku.is_wrong(row, col);
        let given = self.sudoku.is_given(row, col);
        let pencil_marks = self.sudoku.pencil_marks(row, col);
        let cell_send = send.clone();
        let background = if wrong || conflict {
            Color::argb(255, 130, 55, 65)
        } else if selected {
            Color::argb(255, 55, 100, 155)
        } else if same_row || same_col {
            Color::argb(255, 42, 62, 78)
        } else if same_box {
            Color::argb(255, 48, 72, 105)
        } else if given {
            Color::argb(255, 58, 61, 70)
        } else {
            Color::argb(255, 43, 46, 54)
        };
        let border_thickness = Thickness::new(
            if col.is_multiple_of(3) { 2.0 } else { 1.0 },
            if row.is_multiple_of(3) { 2.0 } else { 1.0 },
            if col % 3 == 2 { 2.0 } else { 1.0 },
            if row % 3 == 2 { 2.0 } else { 1.0 },
        );
        Border::new()
            .width(46.0)
            .height(46.0)
            .background(Brush::Solid(background))
            .border_brush(Brush::Solid(Color::argb(255, 105, 109, 122)))
            .border_thickness(border_thickness)
            .corner_radius(CornerRadius::uniform(3.0))
            .content(
                Button::new()
                    .width(44.0)
                    .height(44.0)
                    .on_click(move || cell_send(RammapMessage::SelectCell(row, col)))
                    .content(if value == 0 && pencil_marks != 0 {
                        TextBlock::new()
                            .text(pencil_text(pencil_marks))
                            .font_size(11.0)
                    } else {
                        TextBlock::new()
                            .text(if value == 0 {
                                String::new()
                            } else {
                                value.to_string()
                            })
                            .font_size(20.0)
                            .font_weight(if given {
                                FontWeight::BOLD
                            } else {
                                FontWeight::NORMAL
                            })
                    }),
            )
    }
}

fn pencil_text(marks: u16) -> String {
    (0..3)
        .map(|row| {
            (1..=3)
                .map(|column| {
                    let value = row * 3 + column;
                    if marks & (1 << value) != 0 {
                        char::from_digit(value as u32, 10).unwrap()
                    } else {
                        '·'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn entered_count(sudoku: &crate::sudoku::Sudoku) -> usize {
    sudoku
        .cells()
        .iter()
        .enumerate()
        .flat_map(|(row, cells)| {
            cells
                .iter()
                .enumerate()
                .map(move |(col, &value)| (row, col, value))
        })
        .filter(|&(row, col, value)| value != 0 && !sudoku.is_given(row, col))
        .count()
}
