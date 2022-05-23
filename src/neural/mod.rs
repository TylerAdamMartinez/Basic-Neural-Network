use crate::matrix::Matrix;

pub fn sigmoid(input: f64) -> f64 {
    todo!()
}

pub fn sigmoid_prime(matrix: &Matrix) -> Matrix {
    todo!()
}

pub fn soft_max(matrix: &Matrix) -> Matrix {
    todo!()
}

pub struct NeuralNetwork {
    input: i32,
    hidden: i32,
    output: i32,
    learning_rate: f64,
    hidden_weights: Matrix,
    output_weights: Matrix,
}
