const SOLUTION: [[u8; 9]; 9] = [
    [5, 3, 4, 6, 7, 8, 9, 1, 2],
    [6, 7, 2, 1, 9, 5, 3, 4, 8],
    [1, 9, 8, 3, 4, 2, 5, 6, 7],
    [8, 5, 9, 7, 6, 1, 4, 2, 3],
    [4, 2, 6, 8, 5, 3, 7, 9, 1],
    [7, 1, 3, 9, 2, 4, 8, 5, 6],
    [9, 6, 1, 5, 3, 7, 2, 8, 4],
    [2, 8, 7, 4, 1, 9, 6, 3, 5],
    [3, 4, 5, 2, 8, 6, 1, 7, 9],
];

const PUZZLE: [[u8; 9]; 9] = [
    [5, 3, 0, 0, 7, 0, 0, 0, 2],
    [6, 0, 0, 1, 9, 5, 0, 0, 0],
    [0, 9, 8, 0, 0, 0, 0, 6, 0],
    [8, 0, 0, 7, 6, 0, 0, 0, 3],
    [4, 0, 0, 8, 0, 3, 0, 0, 1],
    [7, 0, 0, 0, 2, 0, 0, 0, 6],
    [0, 6, 0, 0, 0, 0, 2, 8, 0],
    [0, 0, 0, 4, 1, 9, 0, 0, 5],
    [0, 0, 0, 0, 8, 0, 0, 7, 9],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub const ALL: [Self; 3] = [Self::Easy, Self::Normal, Self::Hard];

    pub fn label(self) -> &'static str {
        match self {
            Self::Easy => "初級",
            Self::Normal => "中級",
            Self::Hard => "上級",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Easy => "ヒント多め・単純な消去法向け",
            Self::Normal => "標準的な候補絞り込み向け",
            Self::Hard => "ヒント少なめ・複数候補の検討向け",
        }
    }

    pub fn solving_methods(self) -> &'static [&'static str] {
        match self {
            Self::Easy => &["裸のシングル", "隠れたシングル", "行・列・ブロックの消去"],
            Self::Normal => &[
                "裸のシングル",
                "隠れたシングル",
                "裸のペア",
                "ブロックと行・列の相互作用",
            ],
            Self::Hard => &[
                "裸のペア・トリプル",
                "隠れたペア・トリプル",
                "ブロックと行・列の相互作用",
                "X-Wingなどの候補パターン",
            ],
        }
    }

    pub fn clue_count(self) -> usize {
        match self {
            Self::Easy => 44,
            Self::Normal => 35,
            Self::Hard => 28,
        }
    }

    fn puzzle(self) -> [[u8; 9]; 9] {
        let mut puzzle = PUZZLE;
        match self {
            Self::Easy => {
                for &(row, col) in &[
                    (0, 3),
                    (0, 5),
                    (1, 1),
                    (1, 6),
                    (2, 3),
                    (2, 4),
                    (3, 5),
                    (4, 4),
                    (5, 3),
                ] {
                    puzzle[row][col] = SOLUTION[row][col];
                }
            }
            Self::Normal => {}
            Self::Hard => {
                for &(row, col) in &[(0, 0), (0, 1), (1, 0), (1, 3), (2, 1), (3, 0), (4, 0)] {
                    puzzle[row][col] = 0;
                }
            }
        }
        puzzle
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputResult {
    Updated,
    Incorrect,
    Ignored,
    Completed,
}

#[derive(Debug, Clone)]
pub struct Sudoku {
    cells: [[u8; 9]; 9],
    pencil_marks: [[u16; 9]; 9],
    givens: [[bool; 9]; 9],
    selected: Option<(usize, usize)>,
    completed: bool,
    difficulty: Difficulty,
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl Sudoku {
    pub fn new() -> Self {
        Self::new_with_difficulty(Difficulty::Normal)
    }

    pub fn new_with_difficulty(difficulty: Difficulty) -> Self {
        let puzzle = difficulty.puzzle();
        let mut givens = [[false; 9]; 9];
        for row in 0..9 {
            for col in 0..9 {
                givens[row][col] = puzzle[row][col] != 0;
            }
        }
        Self {
            cells: puzzle,
            pencil_marks: [[0; 9]; 9],
            givens,
            selected: None,
            completed: false,
            difficulty,
        }
    }

    pub fn cells(&self) -> &[[u8; 9]; 9] {
        &self.cells
    }
    pub fn pencil_marks(&self, row: usize, col: usize) -> u16 {
        self.pencil_marks[row][col]
    }
    pub fn is_given(&self, row: usize, col: usize) -> bool {
        self.givens[row][col]
    }
    pub fn selected(&self) -> Option<(usize, usize)> {
        self.selected
    }
    pub fn is_completed(&self) -> bool {
        self.completed
    }
    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn select(&mut self, row: usize, col: usize) {
        if row < 9 && col < 9 {
            self.selected = Some((row, col));
        }
    }

    pub fn input(&mut self, value: u8) -> InputResult {
        let Some((row, col)) = self.selected else {
            return InputResult::Ignored;
        };
        if self.givens[row][col] || value > 9 {
            return InputResult::Ignored;
        }
        self.cells[row][col] = value;
        self.completed = self.cells == SOLUTION;
        if self.completed {
            InputResult::Completed
        } else if value != 0 && value != SOLUTION[row][col] {
            InputResult::Incorrect
        } else {
            InputResult::Updated
        }
    }

    pub fn clear(&mut self) -> InputResult {
        self.input(0)
    }

    pub fn toggle_pencil_mark(&mut self, value: u8) -> InputResult {
        let Some((row, col)) = self.selected else {
            return InputResult::Ignored;
        };
        if self.givens[row][col] || self.cells[row][col] != 0 || !(1..=9).contains(&value) {
            return InputResult::Ignored;
        }
        self.pencil_marks[row][col] ^= 1u16 << value;
        InputResult::Updated
    }

    pub fn has_conflict(&self, row: usize, col: usize) -> bool {
        let value = self.cells[row][col];
        if value == 0 {
            return false;
        }
        (0..9).any(|other| other != col && self.cells[row][other] == value)
            || (0..9).any(|other| other != row && self.cells[other][col] == value)
            || {
                let box_row = row / 3 * 3;
                let box_col = col / 3 * 3;
                (box_row..box_row + 3).any(|r| {
                    (box_col..box_col + 3)
                        .any(|c| (r != row || c != col) && self.cells[r][c] == value)
                })
            }
    }

    pub fn is_wrong(&self, row: usize, col: usize) -> bool {
        let value = self.cells[row][col];
        value != 0 && value != SOLUTION[row][col]
    }

    pub fn reset(&mut self) {
        *self = Self::new_with_difficulty(self.difficulty);
    }
}

#[cfg(test)]
mod tests {
    use super::{InputResult, Sudoku};

    #[test]
    fn givens_cannot_be_changed() {
        let mut game = Sudoku::new();
        game.select(0, 0);
        assert_eq!(game.input(1), InputResult::Ignored);
        assert_eq!(game.cells()[0][0], 5);
    }

    #[test]
    fn entering_solution_completes_game() {
        let mut game = Sudoku::new();
        for row in 0..9 {
            for col in 0..9 {
                if !game.is_given(row, col) {
                    game.select(row, col);
                    assert_eq!(
                        game.input(
                            [
                                [5, 3, 4, 6, 7, 8, 9, 1, 2],
                                [6, 7, 2, 1, 9, 5, 3, 4, 8],
                                [1, 9, 8, 3, 4, 2, 5, 6, 7],
                                [8, 5, 9, 7, 6, 1, 4, 2, 3],
                                [4, 2, 6, 8, 5, 3, 7, 9, 1],
                                [7, 1, 3, 9, 2, 4, 8, 5, 6],
                                [9, 6, 1, 5, 3, 7, 2, 8, 4],
                                [2, 8, 7, 4, 1, 9, 6, 3, 5],
                                [3, 4, 5, 2, 8, 6, 1, 7, 9]
                            ][row][col]
                        ),
                        InputResult::Updated
                    );
                }
            }
        }
        assert!(game.is_completed());
    }

    #[test]
    fn duplicate_values_are_reported() {
        let mut game = Sudoku::new();
        game.select(0, 2);
        assert_eq!(game.input(5), InputResult::Incorrect);
        assert!(game.has_conflict(0, 2));
        assert!(game.is_wrong(0, 2));
    }

    #[test]
    fn pencil_marks_are_retained_when_entering_and_clearing_value() {
        let mut game = Sudoku::new();
        game.select(0, 2);
        assert_eq!(game.toggle_pencil_mark(4), InputResult::Updated);
        assert_eq!(game.pencil_marks(0, 2), 1 << 4);
        game.input(4);
        assert_eq!(game.pencil_marks(0, 2), 1 << 4);
        game.clear();
        assert_eq!(game.pencil_marks(0, 2), 1 << 4);
    }
}
