# Sudoku generator and solver

This is a simple script to generate sudokus and their solutions in the Rust language. It was primarily used by me to
learn more about the rust language.

Currently, only traditional 9x9 sudokus can be generated, further development is needed
to generate larger sudokus.


In order to generate a sudoku, simply run the main.rs:
```rust
fn main() {

    // Create a filled in sudoku
    let sudoku = generate_full_sudoku(9, 9);
    println!("Filled sudoku");
    print_sudoku(&sudoku);

    // Start the process of creating an non filled
    let sudoku_to_solve = generate_sudoku_to_solve(&sudoku, 50);
    println!("To Solve Sudoku");
    print_sudoku(&sudoku_to_solve);

}
```