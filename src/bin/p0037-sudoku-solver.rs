struct Sudoku {
    grid: [[usize; 9]; 9],
    row: [usize; 9],
    column: [usize; 9],
    block: [usize; 9]
}

impl Sudoku {
    fn new() -> Sudoku {
        Sudoku { grid: [[0; 9]; 9], row: [0; 9], column: [0; 9], block: [0; 9] }
    }

    fn xor(&mut self, x: usize, y: usize, v: usize) {
        self.row[x] ^= 1 << v;
        self.column[y] ^= 1 << v;
        self.block[x / 3 * 3 + y / 3] ^= 1 << v;
    }

    fn set(&mut self, x: usize, y: usize, v: usize) {
        self.xor(x, y, self.grid[x][y]);
        self.grid[x][y] = v;
        self.xor(x, y, v);
    }

    fn check(&self, x: usize, y: usize, v: usize) -> bool{
        return (self.row[x] | self.column[y] | self.block[x / 3 * 3 + y / 3]) >> v & 1 == 0;
    }

    fn solve(&mut self, mut x: usize, mut y: usize) -> bool{
        if y == 9 {
            x += 1;
            y = 0;
        }
        if x == 9 {
            return true;
        }
        if self.grid[x][y] != 0 {
            return self.solve(x, y + 1);
        }
        for v in 1..10 {
            if self.check(x, y, v as usize) {
                self.set(x, y, v as usize);
                if self.solve(x, y + 1) {
                    return true;
                }
                self.set(x, y, 0);
            }
        }
        return false;
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut sudoku = Sudoku::new();
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    sudoku.set(i, j, board[i][j] as usize - 48);
                }
            }
        }
        sudoku.solve(0, 0);
        for i in 0..9 {
            for j in 0..9 {
                board[i][j] = (sudoku.grid[i][j] as u8 + b'0') as char;
            }
        }
    }
}