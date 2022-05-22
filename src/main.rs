#[warn(unused_imports)]
use matrix_math::{flatten_matrix, load_matrix, randomize_matrix, save_matrix, Matrix, Vector};

fn main() {
    //build matrix funcs
    let mut new_matrix = Matrix::new(2, 3);
    randomize_matrix(&mut new_matrix, 99);
    println!("matrix after randomize_matrix\n{}", new_matrix);

    // save and load funcs
    save_matrix(&new_matrix, &"./file.txt").expect("failed to save to file");
    let another_new_matrix = load_matrix(&"./file.txt").expect("failed to load from file");
    println!("After load_matrix\n{:?}", another_new_matrix);

    let flatten_matrix_row = flatten_matrix(&another_new_matrix, Vector::Row);
    println!("Row:\n{}", flatten_matrix_row);

    let flatten_matrix_col = flatten_matrix(&another_new_matrix, Vector::Column);
    println!("Col:\n{}", flatten_matrix_col);
}
