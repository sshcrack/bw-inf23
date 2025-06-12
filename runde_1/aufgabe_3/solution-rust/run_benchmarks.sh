#!/bin/bash

# Script to run benchmarks for the Sudoku transformation library
echo "Running Sudoku transformation benchmarks..."

# Ensure we're in the project root
cd $(dirname "$0")

# Run all benchmarks and generate reports
if [ $# -eq 0 ]; then
    echo "Running all benchmarks..."
    cargo bench --bench unified_benchmark -- --verbose
else
    echo "Running specific benchmark: $1"
    cargo bench --bench unified_benchmark "$1" -- --verbose
fi

echo "Benchmarks completed. Results are available in target/criterion/"

# Optionally, generate a summary of results
echo "Benchmark Summary:"
echo "================="
echo "Check the HTML reports in target/criterion/ for detailed results"
echo "You can open them in a browser for interactive visualization"
