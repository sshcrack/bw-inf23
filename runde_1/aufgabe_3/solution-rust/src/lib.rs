// lib.rs - Main library module for the Sudoku operations
// Make all modules public so they can be accessed from the crate root
pub mod operation;
pub mod parser;
pub mod possible_swaps;
pub mod structs;
pub mod utils;

use itertools::Itertools;

use crate::{
    operation::Operation,
    parser::Sudoku,
    possible_swaps::{
        needed_block_swaps, needed_single_swaps, possible_same_block_swaps,
        possible_same_single_swaps,
    },
    structs::{AlgorithmStage, BlockType},
    utils::{are_vec_same, get_filled_block, rotate_sudoku},
};

/// Start the backtracking algorithm to solve the Sudoku transformation
pub fn start_backtracking(
    first: Sudoku,
    second: &Sudoku,
    stage: AlgorithmStage,
    operations: Vec<Operation>,
) -> Option<Vec<Operation>> {
    // Pre-compute the filled block counts to avoid recalculation
    let f_col_counts = get_filled_block(&first, BlockType::Column);
    let f_row_counts = get_filled_block(&first, BlockType::Row);

    let s_col_counts = get_filled_block(second, BlockType::Column);
    let s_row_counts = get_filled_block(second, BlockType::Row);

    // Quick early check if grids are already equal
    if first.grid == second.grid {
        return Some(operations);
    }

    match stage {
        AlgorithmStage::SwapBlocks => {
            // println!("Stage: SwapBlocks \n{:?}\n{:?}", f_col_counts, s_col_counts);
            // println!("Row \n{:?}\n{:?}", f_row_counts, s_row_counts);
            if are_vec_same(&f_col_counts, &s_col_counts)
                && are_vec_same(&f_row_counts, &s_row_counts)
            {
                let col_swaps = possible_same_block_swaps(BlockType::Column, &f_col_counts);
                let row_swaps = possible_same_block_swaps(BlockType::Row, &f_row_counts);

                let powerset = col_swaps
                    .into_iter()
                    .interleave(row_swaps.into_iter())
                    .powerset()
                    .collect_vec();

                // println!(
                //     "Possible column swaps: {:?}\nPossible row swaps: {:?}",
                //     col_swaps, row_swaps
                // );
                //REVIEW Can i really do it like that?
                for ops in powerset {
                    let mut clone = first.clone();
                    let mut new_ops = operations.clone();
                    // println!("----------");
                    for o in &ops {
                        o.apply(&mut clone);
                    }

                    new_ops.extend_from_slice(&ops);
                    let r = start_backtracking(clone, second, AlgorithmStage::SwapSingle, new_ops);
                    // println!("----------");
                    if r.is_some() {
                        return r;
                    }
                }

                return None;
            }

            let col_needed = needed_block_swaps(BlockType::Column, &f_col_counts, &s_col_counts);
            for op in col_needed {
                let mut clone = first.clone();
                op.apply(&mut clone);

                let mut operations = operations.clone();
                operations.push(op);

                let res = start_backtracking(clone, second, AlgorithmStage::SwapBlocks, operations);
                if res.is_some() {
                    return res;
                }
            }
            let row_needed = needed_block_swaps(BlockType::Row, &f_row_counts, &s_row_counts);
            for op in row_needed {
                let mut clone = first.clone();
                op.apply(&mut clone);

                let mut operations = operations.clone();
                operations.push(op);

                let res = start_backtracking(clone, second, AlgorithmStage::SwapBlocks, operations);
                if res.is_some() {
                    return res;
                }
            }

            // println!("No possible block swaps found");
            return None; // No solution found in this stage
        }
        AlgorithmStage::SwapSingle => {
            if f_col_counts == s_col_counts && f_row_counts == s_row_counts {
                // println!("Stage: SwapSingle \n{:?}\n{:?} \n\n{:?}\n{:?}", f_col_counts, s_col_counts, f_row_counts, s_row_counts);
                let col_swaps = possible_same_single_swaps(BlockType::Column, &f_col_counts);
                let rol_swaps = possible_same_single_swaps(BlockType::Row, &f_row_counts);

                // println!(
                //     "Possible column swaps: {:?}\nPossible row swaps: {:?}",
                //     col_swaps, rol_swaps
                // );
                let powerset = col_swaps
                    .into_iter()
                    .interleave(rol_swaps.into_iter())
                    .powerset()
                    .collect_vec();

                //println!("Powerset of operations: {:?}", powerset);
                for ops in powerset {
                    let mut clone = first.clone();
                    let mut operations = operations.clone();

                    for op in &ops {
                        op.apply(&mut clone);
                    }

                    operations.extend_from_slice(&ops);
                    let res =
                        start_backtracking(clone, second, AlgorithmStage::SwapNumbers, operations);
                    if res.is_some() {
                        return res; // Found a solution
                    }
                }

                return None;
            }

            // println!("Stage: SwapSingle \n{:?}\n{:?} \n\n{:?}\n{:?}", f_col_counts, s_col_counts, f_row_counts, s_row_counts);
            let col_needed = needed_single_swaps(BlockType::Column, &f_col_counts, &s_col_counts);
            let row_needed = needed_single_swaps(BlockType::Row, &f_row_counts, &s_row_counts);

            let needed = col_needed
                .into_iter()
                .interleave(row_needed.into_iter())
                .collect_vec();

            for op in needed {
                let mut clone = first.clone();
                op.apply(&mut clone);

                let mut operations = operations.clone();
                operations.push(op);

                let res = start_backtracking(clone, second, AlgorithmStage::SwapSingle, operations);
                if res.is_some() {
                    return res;
                }
            }

            // println!("No possible single swaps found");
            return None; // No solution found in this stage
        }
        AlgorithmStage::SwapNumbers => {
            if first.grid == second.grid {
                return Some(operations); // Sudoku grids are already equal
            }

            let mut change_from = None;
            let mut change_to = None;

            let rows = first.grid.as_rows();
            for (y, row) in rows.iter().enumerate() {
                for (x, numb) in row.iter().enumerate() {
                    let desired = &second.grid[(y, x)];
                    if let (Some(from), Some(to)) = (change_from, change_to) {
                        if numb != &from {
                            continue; // Not the number we want to swap
                        }
                        if numb == &from && desired == &to {
                            continue; // Already swapped this number
                        }

                        // for ele in first.grid.as_rows() {
                        //     println!("{:?}", ele);
                        // }

                        // println!(
                        //     "Invalid state at ({},{}) wanted from {} to {}",
                        //     x, y, from, to
                        // );
                        // println!(
                        //     "Invalid state at ({},{}) wanted from {} to {}",
                        //     x, y, from, to
                        // );
                        return None; // Invalid state, we can't swap more than one number at a time
                    }

                    if numb == desired || numb == &0 || desired == &0 {
                        continue; // Already matches
                    }

                    change_from = Some(*numb);
                    change_to = Some(*desired);
                }
            }

            if change_from.is_none() || change_to.is_none() {
                // println!("No numbers to swap found");
                return None; // No numbers to swap found
            }

            let from = change_from.unwrap();
            let to = change_to.unwrap();

            let op = Operation::SwapNumbers(from, to);

            let mut clone = first.clone();
            op.apply(&mut clone);

            let mut operations = operations.clone();
            operations.push(op);

            return start_backtracking(clone, second, AlgorithmStage::SwapNumbers, operations);
        }
    }

    //println!("Counts: \nCols: \n{:?}\n{:?}\nRows: \n{:?}\n{:?}", f_col_counts, s_col_counts, f_row_counts, s_row_counts);
}

/// Solve a Sudoku transformation problem
pub fn solve_sudoku_transformation(first: Sudoku, second: &Sudoku) -> Option<Vec<Operation>> {
    let mut curr_grid = first;
    let mut operations = None;
    for i in 0..4 {
        let out = start_backtracking(
            curr_grid.clone(),
            second,
            AlgorithmStage::SwapBlocks,
            vec![Operation::Rotate(i)],
        );
        if out.is_some() {
            operations = out;
            break; // Found a solution
        }

        curr_grid = Sudoku::new(rotate_sudoku(&curr_grid.grid));
    }

    operations
}

/// Format operations as human-readable strings
pub fn format_operations(operations: &[Operation]) -> Vec<String> {
    let mut result = Vec::new();

    for op in operations {
        match op {
            Operation::SwapBlock(block_type, a, b) => {
                result.push(format!(
                    "Swap {:?} block {} with block {}",
                    block_type, a, b
                ));
            }
            Operation::SwapSingle(block_type, a, b) => {
                result.push(format!(
                    "Swap single {:?} at ({} with {})",
                    block_type, a, b
                ));
            }
            Operation::SwapSpecial(block_type, (a1, a2), (b1, b2)) => {
                result.push(format!(
                    "Swap single {:?} blocks ({} with {}) and ({} with {})",
                    block_type, a1, a2, b1, b2
                ));
            }
            Operation::SwapNumbers(a, b) => {
                result.push(format!("Swap numbers {} with {}", a, b));
            }
            Operation::Rotate(i) => {
                if i != &0 {
                    result.push(format!("Rotate sudoku by {} degrees", i * 90));
                }
            }
        }
    }

    result
}
