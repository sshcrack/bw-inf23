use std::fs;

use array2d::Array2D;

#[derive(Clone, Debug)]
pub struct Sudoku {
    pub grid: Array2D<u8>,
}

impl Sudoku {
    pub fn new(grid: Array2D<u8>) -> Self {
        Self { grid }
    }
}

pub fn parse_sudoku(sudoku_str: &str) -> Sudoku {
    let sudoku_str = sudoku_str.trim();
    let mut grid = Array2D::filled_with(0, 9, 9);
    for (i, line) in sudoku_str.lines().enumerate() {
        line.split(" ").enumerate().for_each(|(j, cell)| {
            grid[(i, j)] = cell.parse::<u8>().expect(format!("Invalid number: '{:?}' at {},{}", cell, j, i).as_str());
        });
    }

    Sudoku::new(grid)
}

/// Parse a sudoku file containing two sudoku puzzles separated by a blank line
pub fn parse_sudoku_file(filepath: &str) -> (Sudoku, Sudoku) {
    let content = fs::read_to_string(filepath)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filepath))
        .replace("\r", "");

    // Remove UTF-8 BOM if present
    let content = if content.starts_with('\u{FEFF}') {
        content.trim_start_matches('\u{FEFF}').to_string()
    } else {
        content
    };

    let mut parts = content.split("\n\n");
    let first_str = parts.next().expect("Missing first sudoku");
    let second_str = parts.next().expect("Missing second sudoku");

    let first = parse_sudoku(first_str);
    let second = parse_sudoku(second_str);

    (first, second)
}
