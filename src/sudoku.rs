use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

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

    fn board_for_game(self, seed: u64) -> ([[u8; 9]; 9], [[u8; 9]; 9]) {
        let mut random = Random::new(seed);
        let mut digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        random.shuffle(&mut digits);

        let puzzle = map_digits(self.puzzle(), digits);
        let solution = map_digits(SOLUTION, digits);
        (puzzle, solution)
    }
}

fn map_digits(board: [[u8; 9]; 9], digits: [u8; 9]) -> [[u8; 9]; 9] {
    let mut mapped = [[0; 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            let value = board[row][col];
            mapped[row][col] = if value == 0 {
                0
            } else {
                digits[(value - 1) as usize]
            };
        }
    }
    mapped
}

fn random_seed(game_number: u32) -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos() as u64);
    time ^ (u64::from(game_number) << 32) ^ COUNTER.fetch_add(1, Ordering::Relaxed)
}

struct Random(u64);

impl Random {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 7;
        self.0 ^= self.0 >> 9;
        self.0 ^= self.0 << 8;
        self.0
    }

    fn shuffle(&mut self, values: &mut [u8; 9]) {
        for index in (1..values.len()).rev() {
            let other = (self.next() as usize) % (index + 1);
            values.swap(index, other);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputResult {
    Updated,
    Ignored,
    Completed,
}

#[derive(Debug, Clone)]
pub struct Sudoku {
    cells: [[u8; 9]; 9],
    solution: [[u8; 9]; 9],
    pencil_marks: [[u16; 9]; 9],
    givens: [[bool; 9]; 9],
    selected: Option<(usize, usize)>,
    completed: bool,
    answers_checked: bool,
    difficulty: Difficulty,
    game_number: u32,
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
        Self::new_game_state(difficulty, 0)
    }

    fn new_game_state(difficulty: Difficulty, game_number: u32) -> Self {
        let (puzzle, expected_solution) = difficulty.board_for_game(random_seed(game_number));
        let mut derived = puzzle;
        assert!(
            solve_board(&mut derived),
            "generated Sudoku puzzle is unsolvable"
        );
        debug_assert!(contains_givens(&derived, &puzzle));
        let solution = if derived == expected_solution {
            derived
        } else {
            // A puzzle may have multiple solutions, but the generated answer is preferred.
            expected_solution
        };
        let mut givens = [[false; 9]; 9];
        for row in 0..9 {
            for col in 0..9 {
                givens[row][col] = puzzle[row][col] != 0;
            }
        }
        Self {
            cells: puzzle,
            solution,
            pencil_marks: [[0; 9]; 9],
            givens,
            selected: None,
            completed: false,
            answers_checked: false,
            difficulty,
            game_number,
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

    #[allow(dead_code)]
    pub fn solve(&self) -> Option<[[u8; 9]; 9]> {
        let mut board = self.cells;
        solve_board(&mut board).then_some(board)
    }

    pub fn select(&mut self, row: usize, col: usize) {
        if row < 9 && col < 9 {
            self.selected = Some((row, col));
        }
    }

    #[allow(dead_code)]
    pub fn move_selection(&mut self, row_delta: isize, col_delta: isize) {
        let (row, col) = self.selected.unwrap_or((0, 0));
        let next_row = (row as isize + row_delta).clamp(0, 8) as usize;
        let next_col = (col as isize + col_delta).clamp(0, 8) as usize;
        self.select(next_row, next_col);
    }

    pub fn select_next_editable(&mut self) {
        let Some((row, col)) = self.selected else {
            return;
        };
        for offset in 1..=81 {
            let index = (row * 9 + col + offset) % 81;
            let next_row = index / 9;
            let next_col = index % 9;
            if !self.givens[next_row][next_col] {
                self.select(next_row, next_col);
                return;
            }
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
        self.answers_checked = false;
        self.completed = self.cells == self.solution;
        if self.completed {
            InputResult::Completed
        } else {
            InputResult::Updated
        }
    }

    pub fn clear(&mut self) -> InputResult {
        self.input(0)
    }

    pub fn clear_all(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                if !self.givens[row][col] {
                    self.cells[row][col] = 0;
                    self.pencil_marks[row][col] = 0;
                }
            }
        }
        self.answers_checked = false;
        self.completed = false;
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
        self.answers_checked && value != 0 && value != self.solution[row][col]
    }

    pub fn check_answers(&mut self) {
        self.answers_checked = true;
    }

    pub fn new_game(&mut self) {
        *self = Self::new_game_state(self.difficulty, self.game_number.wrapping_add(1));
    }
}

fn solve_board(board: &mut [[u8; 9]; 9]) -> bool {
    if !is_valid_partial_board(board) {
        return false;
    }
    let Some((row, col)) = first_empty(board) else {
        return true;
    };

    for value in 1..=9 {
        if is_safe(board, row, col, value) {
            board[row][col] = value;
            if solve_board(board) {
                return true;
            }
            board[row][col] = 0;
        }
    }
    false
}

fn first_empty(board: &[[u8; 9]; 9]) -> Option<(usize, usize)> {
    for (row, values) in board.iter().enumerate() {
        if let Some(col) = values.iter().position(|&value| value == 0) {
            return Some((row, col));
        }
    }
    None
}

fn is_safe(board: &[[u8; 9]; 9], row: usize, col: usize, value: u8) -> bool {
    !(0..9).any(|index| board[row][index] == value)
        && !(0..9).any(|index| board[index][col] == value)
        && !(row / 3 * 3..row / 3 * 3 + 3).any(|box_row| {
            (col / 3 * 3..col / 3 * 3 + 3).any(|box_col| board[box_row][box_col] == value)
        })
}

fn is_valid_partial_board(board: &[[u8; 9]; 9]) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            let value = board[row][col];
            if value != 0 {
                let mut without_value = *board;
                without_value[row][col] = 0;
                if !is_safe(&without_value, row, col, value) {
                    return false;
                }
            }
        }
    }
    true
}

fn contains_givens(solution: &[[u8; 9]; 9], puzzle: &[[u8; 9]; 9]) -> bool {
    (0..9).all(|row| {
        (0..9).all(|col| puzzle[row][col] == 0 || puzzle[row][col] == solution[row][col])
    })
}

#[cfg(test)]
mod tests {
    use super::{InputResult, Sudoku, contains_givens, is_valid_partial_board};

    #[test]
    fn givens_cannot_be_changed() {
        let mut game = Sudoku::new();
        game.select(0, 0);
        assert_eq!(game.input(1), InputResult::Ignored);
        assert!(game.cells()[0][0] != 0);
    }

    #[test]
    fn entering_solution_completes_game() {
        let mut game = Sudoku::new();
        let solution = game.solve().expect("puzzle should be solvable");
        for row in 0..9 {
            for col in 0..9 {
                if !game.is_given(row, col) {
                    game.select(row, col);
                    assert!(matches!(
                        game.input(solution[row][col]),
                        InputResult::Updated | InputResult::Completed
                    ));
                }
            }
        }
        assert!(game.is_completed());
    }

    #[test]
    fn duplicate_values_are_reported() {
        let mut game = Sudoku::new();
        game.select(0, 2);
        let conflicting_value = game.cells()[0][0];
        assert_eq!(game.input(conflicting_value), InputResult::Updated);
        assert!(game.has_conflict(0, 2));
        assert!(!game.is_wrong(0, 2));
        game.check_answers();
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

    #[test]
    fn selection_can_move_with_arrows_and_skip_givens_after_input() {
        let mut game = Sudoku::new();
        game.select(0, 2);
        game.move_selection(0, -1);
        assert_eq!(game.selected(), Some((0, 1)));
        game.move_selection(0, 1);
        assert_eq!(game.selected(), Some((0, 2)));
        game.input(4);
        game.select_next_editable();
        assert_eq!(game.selected(), Some((0, 3)));
    }

    #[test]
    fn new_game_clears_progress_and_creates_a_new_board() {
        let mut game = Sudoku::new();
        let initial = *game.cells();
        game.select(0, 2);
        game.toggle_pencil_mark(4);
        game.input(4);

        game.new_game();

        assert_ne!(*game.cells(), initial);
        assert_eq!(game.selected(), None);
        assert_eq!(game.pencil_marks(0, 2), 0);
        assert!(!game.is_completed());
    }

    #[test]
    fn clear_all_removes_entries_and_pencil_marks_but_keeps_givens() {
        let mut game = Sudoku::new();
        let given = game.cells()[0][0];
        game.select(0, 2);
        assert_eq!(game.toggle_pencil_mark(4), InputResult::Updated);
        assert_eq!(game.input(4), InputResult::Updated);
        game.select(0, 0);
        game.clear_all();

        assert_eq!(game.cells()[0][0], given);
        assert_eq!(game.cells()[0][2], 0);
        assert_eq!(game.pencil_marks(0, 2), 0);
        assert!(!game.is_completed());
    }

    #[test]
    fn every_difficulty_can_be_derived_from_its_clues() {
        for difficulty in super::Difficulty::ALL {
            let game = Sudoku::new_with_difficulty(difficulty);
            let solution = game.solve().expect("puzzle should be solvable");
            assert!(is_valid_partial_board(&solution));
            assert!(contains_givens(&solution, game.cells()));
        }
    }
}
