use std::collections::HashMap;

use array2d::Array2D;

use crate::{parser::Sudoku, structs::BlockType};

pub fn get_filled_block(sudoku: &Sudoku, block_type: BlockType) -> Vec<Vec<usize>> {
    let grid = &sudoku.grid;
    let mut block = Vec::new();
    for block_id in 0..3 {
        let mut number_counts = Vec::new();
        for inner_b in 0..3 {
            let mut count = 0;
            for i in 0..9 {
                let row = match block_type {
                    BlockType::Row => block_id * 3 + inner_b,
                    BlockType::Column => i,
                };

                let col = match block_type {
                    BlockType::Row => i,
                    BlockType::Column => block_id * 3 + inner_b,
                };

                if grid[(row, col)] != 0 {
                    count += 1;
                }
            }
            number_counts.push(count);
        }

        block.push(number_counts);
    }
    block
}

pub fn get_from(sudoku: &Array2D<u8>, block_type: BlockType, idx: usize) -> Vec<u8> {
    match block_type {
        BlockType::Row => sudoku.row_iter(idx).unwrap().map(|&x| x).collect(),
        BlockType::Column => sudoku.column_iter(idx).unwrap().map(|&x| x).collect(),
    }
}

pub fn get_freq_map(a: &Vec<usize>) -> HashMap<usize, usize> {
    let mut freq_map = HashMap::new();
    for &item in a {
        *freq_map.entry(item).or_insert(0) += 1;
    }
    freq_map
}

pub fn are_vec_same(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for (a_row, b_row) in a.iter().zip(b.iter()) {
        let freq_a = get_freq_map(a_row);
        let freq_b = get_freq_map(b_row);

        if freq_a != freq_b {
            return false;
        }
    }

    true
}

pub fn rotate_sudoku(sudoku: &Array2D<u8>) -> Array2D<u8> {
    let mut new_grid = Array2D::filled_with(0, 9, 9);
    for (y, col) in sudoku.as_columns().iter().enumerate() {
        for (x, &value) in col.iter().enumerate() {
            new_grid[(y, x)] = value;
        }
    }

    new_grid
}