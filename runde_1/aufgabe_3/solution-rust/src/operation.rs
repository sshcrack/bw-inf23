use std::hash::Hash;

use array2d::Array2D;

use crate::{parser::Sudoku, structs::BlockType, utils::get_from};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Operation {
    /// Swap two rows or columns (this is in block coordinates)
    SwapBlock(BlockType, usize, usize),
    SwapSingle(BlockType, usize, usize),
    SwapSpecial(BlockType, (usize, usize), (usize, usize)),
    /// Parameters are first from, seconds is to
    SwapNumbers(u8, u8),
    Rotate(u8)
}

fn swap_single(sudoku: &mut Array2D<u8>, block_type: BlockType, idx_a: usize, idx_b: usize) {
    let a = get_from(sudoku, block_type, idx_a);
    let b = get_from(sudoku, block_type, idx_b);

    //println!("A: {:?}\nB: {:?}", a, b);
    if a == b {
        return; // No need to swap if they are the same
    }

    for (i, new) in a.into_iter().enumerate() {
        let row = match block_type {
            BlockType::Row => idx_b,
            BlockType::Column => i,
        };
        let col = match block_type {
            BlockType::Row => i,
            BlockType::Column => idx_b,
        };

        sudoku[(row, col)] = new;
    }

    for (i, new) in b.into_iter().enumerate() {
        let row = match block_type {
            BlockType::Row => idx_a,
            BlockType::Column => i,
        };
        let col = match block_type {
            BlockType::Row => i,
            BlockType::Column => idx_a,
        };
        sudoku[(row, col)] = new;
    }
}

fn swap_block(sudoku: &mut Array2D<u8>, block_type: BlockType, idx_a: usize, idx_b: usize) {
    for i in 0..3 {
        swap_single(sudoku, block_type, idx_a * 3 + i, idx_b * 3 + i);
    }
}

fn swap_numbers(sudoku: &mut Array2D<u8>, a: u8, b: u8) {
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[(i, j)] == a {
                sudoku[(i, j)] = b;
            } else if sudoku[(i, j)] == b {
                sudoku[(i, j)] = a;
            }
        }
    }
}

impl Operation {
    pub fn block(block_type: BlockType, a: usize, b: usize) -> Self {
        Operation::SwapBlock(block_type, a.min(b), b.max(a))
    }

    pub fn single(block_type: BlockType, a: usize, b: usize) -> Self {
        Operation::SwapSingle(block_type, a.min(b), b.max(a))
    }

    pub fn special(block_type: BlockType, a: (usize, usize), b: (usize, usize)) -> Self {
        Operation::SwapSpecial(
            block_type,
            (a.0.min(a.1), a.0.max(a.1)),
            (b.0.min(b.1), b.0.max(b.1)),
        )
    }

    pub fn apply(&self, sudoku: &mut Sudoku) {
        match self {
            Operation::SwapSingle(block_type, a, b) => {
                // println!("Applying operation: {:?}", self);
                        swap_single(&mut sudoku.grid, *block_type, *a, *b)
                    }
            Operation::SwapSpecial(block_type, (a1, a2), (b1, b2)) => {
                        swap_single(&mut sudoku.grid, *block_type, *a1, *a2);
                        swap_single(&mut sudoku.grid, *block_type, *b1, *b2);
                    }
            Operation::SwapBlock(block_type, a, b) => {
                        swap_block(&mut sudoku.grid, *block_type, *a, *b);
                    }
            Operation::SwapNumbers(a, b) => swap_numbers(&mut sudoku.grid, *a, *b),
            Operation::Rotate(_) => todo!(),
        }
    }
}
