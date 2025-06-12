use bw_inf23_1_3::{
    parser::parse_sudoku,
    solve_sudoku_transformation,
    format_operations,
};

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} <sudoku_file>", args[0]);
        return Ok(());
    }
    let sudoku_file = &args[1];
    let mut sudoku_content = std::fs::read_to_string(sudoku_file)?.replace("\r", "");

    // Remove UTF-8 BOM if present
    if sudoku_content.starts_with('\u{FEFF}') {
        sudoku_content = sudoku_content.trim_start_matches('\u{FEFF}').to_string();
    }

    let mut sudoku_content = sudoku_content.split("\n\n");
    let first = sudoku_content.next().unwrap();
    let second = sudoku_content.next().unwrap();

    let first = parse_sudoku(first);
    let second = parse_sudoku(second);

    // Solve the Sudoku transformation problem
    let operations = solve_sudoku_transformation(first, &second);
    
    if operations.is_none() {
        println!("No solution found");
        return Ok(());
    }
    
    let operations = operations.unwrap();
    if operations.is_empty() {
        println!("Sudoku grids are already equal");
        return Ok(());
    }
    
    println!("Sudoku grids can be made equal with the following operations:");
    for message in format_operations(&operations) {
        println!("{}", message);
    }
    
    Ok(())
}
