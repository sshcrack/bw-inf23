use std::collections::HashSet;

use crate::{operation::Operation, structs::BlockType, utils::get_freq_map};

pub fn needed_single_swaps(
    block_type: BlockType,
    first_counts: &Vec<Vec<usize>>,
    second_counts: &Vec<Vec<usize>>,
) -> Vec<Operation> {
    let mut operations = Vec::with_capacity(first_counts.len() * 3); // Pre-allocate

    for (block_idx, block) in first_counts.iter().enumerate() {
        for (inner_idx, &count) in block.iter().enumerate() {
            let idx = block_idx * 3 + inner_idx;
            let desired_count = &second_counts[block_idx][inner_idx];
            if count == *desired_count {
                continue; // Already matches
            }

            // Find first valid swap candidate using more efficient filtering
            let can_swap_to = (0..block.len())
                .filter(|&i| {
                    // Skip self
                    if i == inner_idx {
                        return false;
                    }
                    
                    let inner_count = &block[i];
                    let swap_desired = &second_counts[block_idx][i];
                    
                    // All conditions combined for early rejection
                    inner_count != swap_desired && 
                    swap_desired == &count && 
                    inner_count == desired_count
                })
                .next();

            if let Some(can_swap_to) = can_swap_to {
                let swap_idx = block_idx * 3 + can_swap_to;
                operations.push(Operation::single(block_type, idx, swap_idx));
            }
        }

        // Check for special case
        // When it is like this:
        // 1 2 3
        // 3 1 2

        // 3 2 1
        let desired_block = &second_counts[block_idx];
        let is_case = !block
            .iter()
            .enumerate()
            .any(|(i, c)| desired_block[i] == *c);

        if is_case {
            let start_block = block_idx * 3;
            operations.push(Operation::special(
                block_type,
                (start_block, start_block + 2),
                (start_block + 1, start_block + 2),
            ));
        }
    }

    operations
}

pub fn needed_block_swaps(
    block_type: BlockType,
    first_counts: &Vec<Vec<usize>>,
    second_counts: &Vec<Vec<usize>>,
) -> Vec<Operation> {
    // Pre-compute frequency maps for better performance
    let freq_maps_first: Vec<_> = first_counts.iter().map(get_freq_map).collect();
    let freq_maps_second: Vec<_> = second_counts.iter().map(get_freq_map).collect();

    let mut operations = Vec::with_capacity(first_counts.len()); // Pre-allocate

    for (i, _) in first_counts.iter().enumerate() {
        let freq_a = &freq_maps_first[i];
        if &freq_maps_second[i] == freq_a {
            continue; // Already matches
        }

        let desired_counts = &freq_maps_second[i];
        for (j, _) in first_counts.iter().enumerate() {
            if i == j {
                continue; // Don't swap with itself
            }

            let freq_b = &freq_maps_first[j];
            if freq_b == freq_a {
                continue; // Already matches
            }

            if freq_b != desired_counts {
                continue; // Not a valid swap
            }

            let swap_desired = &freq_maps_second[j];
            if swap_desired != freq_a {
                continue; // Not a valid swap
            }

            operations.push(Operation::block(block_type, i, j));
        }
    }

    operations
}

/// Use when already properly aligned with second_counts
pub fn possible_same_block_swaps(
    block_type: BlockType,
    first_counts: &Vec<Vec<usize>>,
) -> HashSet<Operation> {
    let mut operations = HashSet::new();
    
    // Pre-compute frequency maps to avoid redundant calculations
    let freq_maps: Vec<_> = first_counts.iter().map(get_freq_map).collect();
    
    for (i, _) in first_counts.iter().enumerate() {
        let freq_a = &freq_maps[i];
        for (j, _) in first_counts.iter().enumerate() {
            if i == j {
                continue; // Don't swap with itself
            }

            let freq_b = &freq_maps[j];
            if freq_a != freq_b {
                continue; // Already matches
            }

            operations.insert(Operation::block(block_type, i, j));
        }
    }

    operations
}

//TODO
pub fn possible_same_single_swaps(
    block_type: BlockType,
    first_counts: &Vec<Vec<usize>>,
) -> HashSet<Operation> {
    let mut operations = HashSet::with_capacity(first_counts.len() * 3); // Pre-allocate space
    
    for (block_idx, block) in first_counts.iter().enumerate() {
        for (inner_idx, count) in block.iter().enumerate() {
            let idx = block_idx * 3 + inner_idx;
            
            // Use iterator for better performance
            let swap_candidates: Vec<_> = block.iter()
                .enumerate()
                .filter(|(i, inner_count)| {
                    *i != inner_idx && **inner_count == *count
                })
                .map(|(i, _)| i)
                .collect();
                
            for can_swap_to in swap_candidates {
                let swap_idx = block_idx * 3 + can_swap_to;
                operations.insert(Operation::single(block_type, idx, swap_idx));
            }
        }
    }

    operations
}
