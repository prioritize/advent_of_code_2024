use std::char;
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;
pub fn transpose_vectors(incoming: Vec<String>) -> Vec<String> {
    let mut vert_buffer = incoming
        .iter()
        .map(|_| String::new())
        .collect::<Vec<String>>();

    incoming.iter().for_each(|line| {
        line.chars().enumerate().for_each(|(idx, c)| {
            vert_buffer[idx].push(c);
        });
    });
    println!("{:?}", vert_buffer);
    vert_buffer
}
pub fn get_incoming_data(fname: &str) -> Vec<Vec<char>> {
    let file = File::open(fname).expect(&format!("Unable to open {}", fname));
    BufReader::new(file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            x.chars().collect()
        })
        .collect()
}
pub fn created_visited(puzzle: &[Vec<char>]) -> Vec<Vec<bool>> {
    let cols = puzzle.len();
    let rows = puzzle[0].len();
    vec![vec![false; rows]; cols]
}
pub struct Puzzle {
    puzzle: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
    match_against: String,
}
impl Puzzle {
    pub fn new(puzzle: Vec<Vec<char>>) -> Self {
        let visited = created_visited(&puzzle);
        let rows = puzzle.len();
        let cols = puzzle[0].len();
        Puzzle {
            puzzle,
            visited,
            rows,
            cols,
            match_against: String::from("XMAS"),
        }
    }
    pub fn check_all_neighbors(&self) -> u32 {
        self.puzzle
            .iter()
            .enumerate()
            .map(|(r_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c_idx, col)| self.check_neighbors((r_idx, c_idx)))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>()
            .iter()
            .flatten()
            .sum()
    }
    pub fn check_cross_neighbors(&self) -> u32 {
        self.puzzle
            .iter()
            .enumerate()
            .map(|(r_idx, c)| {
                c.iter()
                    .enumerate()
                    .map(|(c_idx, _)| self.check_cross(r_idx, c_idx))
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>()
            .iter()
            .flatten()
            .map(|v| if *v { 1 } else { 0 })
            .collect::<Vec<u32>>()
            .iter()
            .sum()
    }
    pub fn check_cross(&self, r_idx: usize, c_idx: usize) -> bool {
        let mut first = false;
        if self.puzzle[r_idx][c_idx] == 'A'
            && c_idx.checked_sub(1).is_some()
            && r_idx.checked_sub(1).is_some()
            && c_idx + 1 < self.cols
            && r_idx + 1 < self.rows
        {
            match self.puzzle[r_idx - 1][c_idx - 1] == 'M' {
                true => {
                    if self.puzzle[r_idx + 1][c_idx + 1] == 'S' {
                        first = true;
                    }
                }
                false => match self.puzzle[r_idx - 1][c_idx - 1] == 'S' {
                    true => {
                        if self.puzzle[r_idx + 1][c_idx + 1] == 'M' {
                            first = true;
                        }
                    }
                    false => {}
                },
            }
            if first {
                match self.puzzle[r_idx - 1][c_idx + 1] == 'M' {
                    true => {
                        if self.puzzle[r_idx + 1][c_idx - 1] == 'S' {
                            return true;
                        }
                    }
                    false => match self.puzzle[r_idx - 1][c_idx + 1] == 'S' {
                        true => {
                            if self.puzzle[r_idx + 1][c_idx - 1] == 'M' {
                                return true;
                            }
                        }
                        false => {}
                    },
                }
            }
        }
        false
    }

    pub fn check_neighbors(&self, idx: (usize, usize)) -> u32 {
        let mut out = 0;
        if self.check_left(idx.0, idx.1, "XMAS") {
            out += 1;
        }
        if self.check_up(idx.0, idx.1, "XMAS") {
            out += 1;
        }
        if self.check_right(idx.0, idx.1, "XMAS") {
            out += 1;
        }
        if self.check_down(idx.0, idx.1, "XMAS") {
            out += 1;
        }
        if self.check_diagonal_left_down(idx.0, idx.1, "XMAS") {
            out += 1;
        }
        if self.check_diagonal_left_up(idx.0, idx.1, "XMAS") {
            out += 1;
        }
        if self.check_diagonal_right_down(idx.0, idx.1, "XMAS") {
            out += 1;
        }
        if self.check_diagonal_right_up(idx.0, idx.1, "XMAS") {
            out += 1;
        }
        out
    }
    fn check_up(&self, row: usize, col: usize, query: &str) -> bool {
        if (row + 1).checked_sub(query.len()).is_some() {
            for (x, v) in query.chars().enumerate() {
                if v == self.puzzle[row - x][col] {
                } else {
                    return false;
                }
            }
            return true;
        }
        false
    }
    fn check_down(&self, row: usize, col: usize, query: &str) -> bool {
        if row + query.len() <= self.cols {
            for (x, v) in query.chars().enumerate() {
                if v == self.puzzle[row + x][col] {
                } else {
                    return false;
                }
            }
            return true;
        }
        false
    }
    fn check_left(&self, row: usize, col: usize, query: &str) -> bool {
        if (col + 1).checked_sub(query.len()).is_some() {
            for (x, v) in query.chars().enumerate() {
                if v == self.puzzle[row][col - x] {
                } else {
                    return false;
                }
            }
            return true;
        }
        false
    }
    fn check_right(&self, row: usize, col: usize, query: &str) -> bool {
        if col + query.len() <= self.cols {
            for (x, v) in query.chars().enumerate() {
                if v == self.puzzle[row][col + x] {
                } else {
                    return false;
                }
            }
            return true;
        }
        false
    }
    fn check_diagonal_right_up(&self, row: usize, col: usize, query: &str) -> bool {
        if col + query.len() <= self.cols && (row + 1).checked_sub(query.len()).is_some() {
            for (x, v) in query.chars().enumerate() {
                if v == self.puzzle[row - x][col + x] {
                } else {
                    return false;
                }
            }
            return true;
        }
        false
    }
    fn check_diagonal_right_down(&self, row: usize, col: usize, query: &str) -> bool {
        if col + query.len() <= self.cols && row + query.len() <= self.rows {
            for (x, v) in query.chars().enumerate() {
                if v == self.puzzle[row + x][col + x] {
                } else {
                    return false;
                }
            }
            return true;
        }
        false
    }
    fn check_diagonal_left_up(&self, row: usize, col: usize, query: &str) -> bool {
        if (col + 1).checked_sub(query.len()).is_some()
            && (row + 1).checked_sub(query.len()).is_some()
        {
            for (x, v) in query.chars().enumerate() {
                if v == self.puzzle[row - x][col - x] {
                } else {
                    return false;
                }
            }
            return true;
        }
        false
    }
    fn check_diagonal_left_down(&self, row: usize, col: usize, query: &str) -> bool {
        if (col + 1).checked_sub(query.len()).is_some() && row + query.len() <= self.rows {
            for (x, v) in query.chars().enumerate() {
                if v == self.puzzle[row + x][col - x] {
                } else {
                    return false;
                }
            }
            return true;
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vert_buffer() {
        let data = get_incoming_data("input/day_4_input.txt");
    }
    #[test]
    fn test_print_chars() {
        let data = get_incoming_data("input/day_4_input.txt");
    }
    #[test]
    fn test_check_right() {
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['X', 'M', 'A', 'S'];
        let v2 = vec!['X', 'M', 'A', 'S'];
        let test_vec = vec![v0, v1, v2];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_right(0, 0, "XMAS");
        assert_eq!(true, test_value);
        let v0 = vec!['X', 'M', 'A', 'M'];
        let v1 = vec!['X', 'M', 'A', 'M'];
        let v2 = vec!['X', 'M', 'A', 'M'];
        let test_vec = vec![v0, v1, v2];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_right(0, 0, "XMAS");
        assert_ne!(test_value, true);
    }
    #[test]
    fn test_check_left() {
        let v0 = vec!['S', 'A', 'M', 'X'];
        let v1 = vec!['X', 'M', 'A', 'S'];
        let v2 = vec!['X', 'M', 'A', 'S'];
        let test_vec = vec![v0, v1, v2];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_left(0, 3, "XMAS");
        assert_eq!(true, test_value);
        let v0 = vec!['M', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['M', 'M', 'A', 'S'];
        let test_vec = vec![v0, v1, v2];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_left(0, 3, "XMAS");
        assert_ne!(test_value, true);
    }
    #[test]
    fn test_check_down() {
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['A', 'M', 'A', 'S'];
        let v3 = vec!['S', 'M', 'A', 'S'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_down(0, 0, "XMAS");
        assert_eq!(true, test_value);
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['A', 'M', 'A', 'S'];
        let v3 = vec!['M', 'M', 'A', 'S'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_down(0, 0, "XMAS");
        assert_ne!(test_value, true);
    }
    #[test]
    fn test_check_up() {
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['A', 'M', 'A', 'S'];
        let v3 = vec!['S', 'M', 'A', 'S'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_up(3, 0, "XMAS");
        assert_eq!(true, test_value);
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['A', 'M', 'A', 'S'];
        let v3 = vec!['X', 'M', 'A', 'S'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_up(3, 0, "XMAS");
        assert_ne!(test_value, true);
    }
    #[test]
    fn test_check_right_up() {
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['A', 'M', 'A', 'S'];
        let v3 = vec!['X', 'M', 'A', 'S'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_diagonal_right_up(3, 0, "XMAS");
        assert_eq!(true, test_value);
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['A', 'M', 'A', 'S'];
        let v3 = vec!['M', 'M', 'A', 'S'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_diagonal_right_up(3, 0, "XMAS");
        assert_ne!(test_value, true);
    }
    #[test]
    fn test_check_right_down() {
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['A', 'M', 'A', 'S'];
        let v3 = vec!['X', 'M', 'A', 'S'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_diagonal_right_down(0, 0, "XMAS");
        assert_eq!(true, test_value);
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['A', 'M', 'A', 'S'];
        let v3 = vec!['M', 'M', 'A', 'X'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_diagonal_right_down(0, 0, "XMAS");
        assert_ne!(test_value, true);
    }
    #[test]
    fn test_check_left_up() {
        let v0 = vec!['S', 'M', 'A', 'S'];
        let v1 = vec!['M', 'A', 'A', 'S'];
        let v2 = vec!['A', 'M', 'M', 'S'];
        let v3 = vec!['X', 'M', 'A', 'X'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_diagonal_left_up(3, 3, "XMAS");
        assert_eq!(true, test_value);
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['A', 'M', 'A', 'S'];
        let v3 = vec!['M', 'M', 'A', 'X'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_diagonal_left_up(3, 3, "XMAS");
        assert_ne!(test_value, true);
    }
    #[test]
    fn test_check_left_down() {
        let v0 = vec!['S', 'M', 'A', 'X'];
        let v1 = vec!['M', 'A', 'M', 'S'];
        let v2 = vec!['A', 'A', 'M', 'S'];
        let v3 = vec!['S', 'M', 'A', 'X'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_diagonal_left_down(0, 3, "XMAS");
        assert_eq!(true, test_value);
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['A', 'M', 'A', 'S'];
        let v3 = vec!['M', 'M', 'A', 'X'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_diagonal_left_down(0, 3, "XMAS");
        assert_ne!(test_value, true);
    }
    #[test]
    fn test_check_neighbors() {
        let v0 = vec!['S', 'A', 'M', 'X', 'M', 'A', 'S'];
        let v1 = vec!['X', 'A', 'M', 'S', 'S', 'A', 'X'];
        let v2 = vec!['M', 'A', 'M', 'S', 'M', 'M', 'V'];
        let v3 = vec!['S', 'A', 'M', 'X', 'M', 'A', 'S'];
        let v4 = vec!['S', 'A', 'M', 'S', 'M', 'M', 'V'];
        let v5 = vec!['M', 'A', 'M', 'S', 'S', 'A', 'X'];
        let v6 = vec!['S', 'A', 'M', 'X', 'M', 'A', 'S'];
        let test_vec = vec![v0, v1, v2, v3, v4, v5, v6];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_all_neighbors();
        assert_eq!(12, test_value);
        let v0 = vec!['X', 'M', 'A', 'S'];
        let v1 = vec!['M', 'M', 'A', 'S'];
        let v2 = vec!['A', 'M', 'A', 'S'];
        let v3 = vec!['M', 'M', 'A', 'X'];
        let test_vec = vec![v0, v1, v2, v3];
        let puzzle = Puzzle::new(test_vec);
        let test_value = puzzle.check_diagonal_left_down(0, 3, "XMAS");
        assert_ne!(test_value, true);
    }
    #[test]
    fn test_day_4_input() {
        let data = get_incoming_data("input/day_4_input.txt");
        let puzzle = Puzzle::new(data);
        let out = puzzle.check_all_neighbors();
        println!("{out}");
    }
    #[test]
    fn test_day_4_test_input() {
        let data = get_incoming_data("input/day_4_test_input.txt");
        let puzzle = Puzzle::new(data);
        let out = puzzle.check_all_neighbors();
        println!("{out}");
    }
    #[test]
    fn test_day_4_cross_test_input() {
        let data = get_incoming_data("input/day_4_test_input.txt");
        let puzzle = Puzzle::new(data);
        let out = puzzle.check_cross_neighbors();
        println!("{out}");
    }

    #[test]
    fn test_day_4_cross_input() {
        let data = get_incoming_data("input/day_4_input.txt");
        let puzzle = Puzzle::new(data);
        let out = puzzle.check_cross_neighbors();
        println!("{out}");
    }
}
