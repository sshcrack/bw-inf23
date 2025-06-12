use bw_inf23_1_3::{
    parser::parse_sudoku_file,
    solve_sudoku_transformation,
};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::{hint::black_box, time::Duration};

/// Compare performance across all test files
fn compare_sudoku_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("Compare Sudoku Files");
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(20);

    // Load all sudoku files
    for i in 0..=4 {
        let filepath = format!("files/sudoku{}.txt", i);
        let (first, second) = parse_sudoku_file(&filepath);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("sudoku{}", i)),
            &i,
            |b, _| {
                b.iter(|| {
                    solve_sudoku_transformation(black_box(first.clone()), black_box(&second))
                });
            },
        );
    }

    group.finish();
}

// /// Benchmark starting from different algorithm stages
// fn bench_algorithm_stages(c: &mut Criterion) {
//     let mut group = c.benchmark_group("Algorithm Stages");
//     group.measurement_time(Duration::from_secs(10));

//     // Only use one file for this benchmark to keep it manageable
//     let (first, second) = parse_sudoku_file("files/sudoku0.txt");

//     // Benchmark the full solution process
//     group.bench_function("full_solution", |b| {
//         b.iter(|| solve_sudoku_transformation(black_box(first.clone()), black_box(&second)));
//     });

//     group.finish();
// }

criterion_group!(
    benches,
    compare_sudoku_files,
    // bench_algorithm_stages,
);
criterion_main!(benches);
