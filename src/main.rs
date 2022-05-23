mod matrix;
mod neural;

fn main() {
    //build matrix funcs
    let mut new_matrix = matrix::Matrix::new(10, 10);
    matrix::randomize_matrix(&mut new_matrix, 99);
    println!("matrix after randomize_matrix\n{}", new_matrix);

    // save and load funcs
    matrix::save_matrix(&new_matrix, &"./file.txt").expect("failed to save to file");
    let another_new_matrix = matrix::load_matrix(&"./file.txt").expect("failed to load from file");
    println!("After load_matrix\n{:?}", another_new_matrix);

    let flatten_matrix_row = matrix::flatten_matrix(&another_new_matrix, matrix::Vector::Row);
    println!("Row:\n{}", flatten_matrix_row);

    let flatten_matrix_col = matrix::flatten_matrix(&another_new_matrix, matrix::Vector::Column);
    println!("Col:\n{}", flatten_matrix_col);
}
