#[warn(unused_imports)]
use matrix_math::{load_matrix, randomize_matrix, save_matrix, Matrix};

fn main() {
    //build matrix funcs
    let mut new_matrix = Matrix::new(5, 7);
    new_matrix.fill(0.432);
    println!("matrix after fill\n{}", new_matrix);
    randomize_matrix(&mut new_matrix, 5);
    println!("matrix after randomize_matrix\n{}", new_matrix);

    // save and load funcs
    save_matrix(&new_matrix, &"./file.txt").expect("failed to save to file");
    let another_new_matrix = load_matrix(&"./file.txt").expect("failed to load from file");
    println!("After load_matrix\n{:?}", another_new_matrix);
}
