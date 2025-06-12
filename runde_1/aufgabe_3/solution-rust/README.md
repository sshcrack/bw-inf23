# Sudokopie - Rust Solution

This is a Rust implementation for solving the "Sudokopie" problem from the Bundeswettbewerb Informatik. The program detects if two Sudoku puzzles are variants of each other through a series of transformations like row/column swaps, rotations, and digit renaming.

## Prerequisites

### Installing Rust

To run this code, you need to have Rust installed on your system:

1. **Install Rust** - Follow the official installation instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

   - **On Linux/macOS**:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
     Follow the on-screen instructions, then restart your terminal or run:
     ```bash
     source "$HOME/.cargo/env"
     ```

   - **On Windows**:
     Download and run the [Rust installer](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)

2. **Verify installation**:
   ```bash
   rustc --version
   cargo --version
   ```

## Building and Running

### Compiling the Project

1. Navigate to the project directory:
   ```bash
   cd path/to/solution-rust
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

### Running the Program

The program expects a file containing two Sudoku puzzles separated by a blank line:

```bash
cargo run --release -- path/to/sudoku_file.txt
```

For example, to test with the provided test files:

```bash
cargo run --release -- files/sudoku0.txt
```

### Example Input Format

The input file should contain two Sudoku puzzles separated by a blank line. Each Sudoku is represented as a 9×9 grid of digits, where 0 represents an empty cell:

```
530070000
600195000
098000060
800060003
400803001
700020006
060000280
000419005
000080079

530070000
600195000
098000060
800060003
400803001
700020006
060000280
000419005
000080079
```

### Output

The program will output the sequence of operations needed to transform the first Sudoku into the second, or report that no such transformation exists:

```
Sudoku grids can be made equal with the following operations:
Swap Column block 0 with block 2
Swap single Row at (4 with 5)
Swap numbers 1 with 2
Swap numbers 2 with 3
Swap numbers 3 with 4
Swap numbers 4 with 5
Swap numbers 5 with 6
Swap numbers 6 with 7
Swap numbers 7 with 8
Swap numbers 8 with 9
Swap numbers 9 with 1
```

## Running Tests

To run the tests:

```bash
cargo test
```

## Benchmarking

The project includes benchmarks to measure performance. To run them:

```bash
./run_benchmarks.sh
```

or

```bash
cargo bench
```

## Project Structure

- `src/lib.rs` - Core algorithm implementation
- `src/main.rs` - Command-line interface
- `src/operation.rs` - Sudoku transformation operations
- `src/parser.rs` - Sudoku parsing functionality
- `src/possible_swaps.rs` - Functions to determine possible swaps
- `src/structs.rs` - Data structures
- `src/utils.rs` - Utility functions
