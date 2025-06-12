# Sudoku Transformation Solver Benchmarks

This directory contains benchmarks for the Sudoku transformation solver library.

## Running Benchmarks

You can run the benchmarks using the included script:

```bash
./run_benchmarks.sh
```

Or directly with Cargo:

```bash
cargo bench
```

## Available Benchmarks

**unified_benchmark.rs**:
   - `compare_sudoku_files`: Compares performance across all test files

## Viewing Results

After running the benchmarks, detailed HTML reports will be available in the `target/criterion/` directory. You can open these in a web browser to see interactive charts and detailed performance metrics.

## Test Files

The benchmarks use the sudoku files in the `files/` directory:
- sudoku0.txt
- sudoku1.txt
- sudoku2.txt
- sudoku3.txt
- sudoku4.txt

Each file contains two sudoku puzzles separated by a blank line. The solver attempts to transform the first puzzle into the second puzzle through a series of operations.
