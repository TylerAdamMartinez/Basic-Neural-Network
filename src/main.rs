#[warn(unused_imports)]
use matrix_math::{load_matrix, save_matrix, Matrix};

fn main() {
    let mut new_matrix = Matrix::new(5, 7);
    new_matrix.fill(0.432);
    save_matrix(&new_matrix, &"./file.txt").expect("failed to save to file");
    let another_new_matrix = load_matrix(&"./file.txt").expect("failed to load from file");
    println!("After load_matrix\n{:?}", another_new_matrix);
}
