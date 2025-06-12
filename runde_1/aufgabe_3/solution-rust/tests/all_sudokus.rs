#[test]
pub fn test_sudoku0() {
    use bw_inf23_1_3::parser::parse_sudoku_file;
    use bw_inf23_1_3::solve_sudoku_transformation;

    let (first, second) = parse_sudoku_file("files/sudoku0.txt");
    solve_sudoku_transformation(first, &second).expect("Failed to solve sudoku transformation");
}

#[test]
pub fn test_sudoku1() {
    use bw_inf23_1_3::parser::parse_sudoku_file;
    use bw_inf23_1_3::solve_sudoku_transformation;

    let (first, second) = parse_sudoku_file("files/sudoku1.txt");
    solve_sudoku_transformation(first, &second).expect("Failed to solve sudoku transformation");
}

#[test]
pub fn test_sudoku2() {
    use bw_inf23_1_3::parser::parse_sudoku_file;
    use bw_inf23_1_3::solve_sudoku_transformation;

    let (first, second) = parse_sudoku_file("files/sudoku2.txt");
    solve_sudoku_transformation(first, &second).expect("Failed to solve sudoku transformation");
}

#[test]
pub fn test_sudoku3() {
    use bw_inf23_1_3::parser::parse_sudoku_file;
    use bw_inf23_1_3::solve_sudoku_transformation;

    let (first, second) = parse_sudoku_file("files/sudoku3.txt");
    let r = solve_sudoku_transformation(first, &second);
    if r.is_some() {
        panic!("Sudoku transformation should not have a solution, but found one.");
    }
}

#[test]
pub fn test_sudoku4() {
    use bw_inf23_1_3::parser::parse_sudoku_file;
    use bw_inf23_1_3::solve_sudoku_transformation;

    let (first, second) = parse_sudoku_file("files/sudoku4.txt");
    solve_sudoku_transformation(first, &second).expect("Failed to solve sudoku transformation");
}
