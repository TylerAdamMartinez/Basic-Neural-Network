use crate::image::Image;
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
    hidden_weights: Option<Matrix>,
    output_weights: Option<Matrix>,
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
    pub fn new(input: i32, hidden: i32, output: i32, learning_rate: f64) -> Self {
        Self {
            input,
            hidden,
            output,
            learning_rate,
            hidden_weights: None,
            output_weights: None,
        }
    }

    pub fn add_hidden_weights(&mut self, hidden_weights: Matrix) {
        self.hidden_weights = Some(hidden_weights);
    }

    pub fn add_output_weights(&mut self, output_weights: Matrix) {
        self.output_weights = Some(output_weights);
    }

    pub fn train(&mut self, input_data: &Matrix, output_data: &Matrix) {}

    pub fn train_batch_images(&mut self, images: &Vec<Image>) {}

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
