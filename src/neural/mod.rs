use crate::image::Image;
use crate::matrix::{
    apply, dot_product, flatten_matrix, randomize_matrix, transpose, Matrix, Vector,
};

pub fn sigmoid(input: f64) -> f64 {
    1.0 / (1.0 + (-1.0 * input).exp())
}

pub fn sigmoid_prime(matrix: &Matrix) -> Matrix {
    let mut ones_matrix = Matrix::new(matrix.rows, matrix.cols);
    ones_matrix.fill(1.0);
    let subtracted_matrix = ones_matrix - matrix.clone();
    let multiplied_matrix = matrix.clone() * subtracted_matrix;
    multiplied_matrix
}

pub fn soft_max(matrix: &Matrix) -> Matrix {
    let mut total: f64 = 0.0;
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            total += matrix.entries[i][j].exp();
        }
    }

    let mut soft_max_matrix = Matrix::new(matrix.rows, matrix.cols);
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            soft_max_matrix.entries[i][j] = matrix.entries[i][j].exp() / total;
        }
    }

    soft_max_matrix
}

pub struct NeuralNetwork {
    input: usize,
    hidden: usize,
    output: usize,
    learning_rate: f64,
    hidden_weights: Matrix,
    output_weights: Matrix,
}

impl std::fmt::Display for NeuralNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Neural Network Stats:\ninput: {}\nhidden: {}\noutput: {}\nlearning_rate: {}\nhidden weights\n{:#?}\noutput weights\n{:#?}\n",
            self.input, self.hidden, self.output, self.learning_rate, self.hidden_weights, self.output_weights
        )
    }
}

impl NeuralNetwork {
    pub fn new(input: usize, hidden: usize, output: usize, learning_rate: f64) -> Self {
        let mut hidden_matrix = Matrix::new(hidden, input);
        let mut output_matrix = Matrix::new(output, hidden);

        randomize_matrix(&mut hidden_matrix, hidden as u64);
        randomize_matrix(&mut output_matrix, output as u64);

        Self {
            input,
            hidden,
            output,
            learning_rate,
            hidden_weights: hidden_matrix,
            output_weights: output_matrix,
        }
    }

    pub fn train(&mut self, input_data: &Matrix, output_data: &Matrix) {
        // Feed
        let hidden_inputs = dot_product(&self.hidden_weights, input_data);
        let hidden_outputs = apply(sigmoid, &hidden_inputs);
        let final_inputs = dot_product(&self.output_weights, &hidden_outputs);
        let final_outputs = apply(sigmoid, &final_inputs);

        // Errors
        let output_errors = output_data.clone() - final_outputs.clone();
        let hidden_errors = dot_product(&transpose(&self.output_weights), &output_errors);

        // Backprograte output_weights
        let mut sigmoid_prime_matrix = sigmoid_prime(&final_outputs);
        let mut multiplied_matrix = output_errors.clone() * sigmoid_prime_matrix;
        let mut transpose_matrix = transpose(&hidden_outputs);
        let mut dot_product_matrix = dot_product(&multiplied_matrix, &transpose_matrix);
        let mut scaled_matrix = dot_product_matrix.clone();
        scaled_matrix.scale(self.learning_rate);
        let mut added_matrix = self.output_weights.clone() + dot_product_matrix;
        self.output_weights = added_matrix;

        // Backprograte hidden_weights
        sigmoid_prime_matrix = sigmoid_prime(&hidden_outputs);
        multiplied_matrix = hidden_errors * sigmoid_prime_matrix;
        transpose_matrix = transpose(&input_data);
        dot_product_matrix = dot_product(&multiplied_matrix, &transpose_matrix);
        scaled_matrix = dot_product_matrix.clone();
        scaled_matrix.scale(self.learning_rate);

        added_matrix = self.hidden_weights.clone() + dot_product_matrix;
        self.hidden_weights = added_matrix;
    }

    pub fn train_batch_images(&mut self, images: &Vec<Image>) {
        for image in images {
            let image_data = flatten_matrix(&image.img_data, Vector::Column);
            let mut output = Matrix::new(10, 1);
            output.entries[image.label][0] = 1.0;
            self.train(&image_data, &output);
        }
    }

    pub fn predict(&mut self, input_matrix: &Matrix) -> Matrix {
        todo!()
    }

    pub fn predict_image(&mut self, image: &Image) -> Matrix {
        todo!()
    }

    pub fn predict_images(&mut self, images: &Vec<Image>) -> f64 {
        todo!()
    }
}

pub fn save_neural_network(neural_network: &NeuralNetwork, file_path: &str) -> Result<(), String> {
    todo!()
}

pub fn laod_neural_network(file_path: &str) -> Result<NeuralNetwork, String> {
    todo!()
}
