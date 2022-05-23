#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub entries: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut matrix_output: String = String::new();
        for rows in self.entries.iter() {
            for element in rows.iter() {
                matrix_output.push_str(&format!("{} ", element));
            }
            matrix_output.push_str(&"\n");
        }
        write!(f, "{}", matrix_output)
    }
}

fn check_dimenstions(mat0: &Matrix, mat1: &Matrix) {
    if !(mat0.rows == mat1.rows && mat0.cols == mat1.cols) {
        panic!("The two matrixes dimensions do not match");
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Self) -> Self {
        check_dimenstions(&self, &other);

        let mut new_matix = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                new_matix.entries[i][j] = self.entries[i][j] + other.entries[i][j];
            }
        }
        new_matix
    }
}

impl std::ops::Sub for Matrix {
    type Output = Matrix;

    fn sub(self, other: Self) -> Self {
        check_dimenstions(&self, &other);

        let mut new_matix = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                new_matix.entries[i][j] = self.entries[i][j] - other.entries[i][j];
            }
        }
        new_matix
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Self {
        check_dimenstions(&self, &other);

        let mut new_matix = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                new_matix.entries[i][j] = self.entries[i][j] * other.entries[i][j];
            }
        }
        new_matix
    }
}

pub fn dot_product(mat0: &Matrix, mat1: &Matrix) -> Matrix {
    if mat0.cols == mat1.rows {
        panic!("The column size of the first matrix must match the row size of the second matrix");
    }

    let mut build_matrix = Matrix::new(mat0.rows, mat1.cols);
    for i in 0..mat0.rows {
        for j in 0..mat1.cols {
            let mut sum: f64 = 0.0;
            for k in 0..mat1.rows {
                sum += mat0.entries[i][k] * mat1.entries[k][j];
            }
            build_matrix.entries[i][j] = sum;
        }
    }
    build_matrix
}

pub fn transpose(mat: &Matrix) -> Matrix {
    let mut build_matrix = Matrix::new(mat.cols, mat.rows);
    for i in 0..mat.rows {
        for j in 0..mat.cols {
            build_matrix.entries[j][i] = mat.entries[i][j];
        }
    }
    build_matrix
}

#[cfg(test)]
mod matrix_ops_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn mismatched_dimenstion_matrixes() {
        let matrix0 = Matrix::new(4, 5);
        let matrix1 = Matrix::new(5, 4);

        let _sum_matrix = matrix0 + matrix1;
    }

    #[test]
    fn add() {
        let mut ctrl_matrix = Matrix::new(5, 5);
        ctrl_matrix.fill(25.0);

        let mut add0_matrix = Matrix::new(5, 5);
        add0_matrix.fill(20.0);
        let mut add1_matrix = Matrix::new(5, 5);
        add1_matrix.fill(5.0);

        let sum_matrix = add0_matrix + add1_matrix;
        assert_eq!(ctrl_matrix, sum_matrix);
    }

    #[test]
    fn sub() {
        let mut ctrl_matrix = Matrix::new(2, 7);
        ctrl_matrix.fill(75.6);

        let mut sub0_matrix = Matrix::new(2, 7);
        sub0_matrix.fill(25.0);
        let mut sub1_matrix = Matrix::new(2, 7);
        sub1_matrix.fill(100.6);

        let sum_matrix = sub1_matrix - sub0_matrix;
        assert_eq!(ctrl_matrix, sum_matrix);
    }

    #[test]
    fn mul() {
        let mut ctrl_matrix = Matrix::new(1, 2);
        ctrl_matrix.fill(12.0);

        let mut mul0_matrix = Matrix::new(1, 2);
        mul0_matrix.fill(4.0);
        let mut mul1_matrix = Matrix::new(1, 2);
        mul1_matrix.fill(3.0);

        let sum_matrix = mul1_matrix * mul0_matrix;
        assert_eq!(ctrl_matrix, sum_matrix);
    }

    #[test]
    fn dot() {
        let mut ctrl_matrix = Matrix::new(3, 3);
        ctrl_matrix.fill(15.0);

        let mut mul0_matrix = Matrix::new(1, 3);
        mul0_matrix.fill(5.0);
        let mut mul1_matrix = Matrix::new(3, 5);
        mul1_matrix.fill(3.0);

        let sum_matrix = dot_product(&mul1_matrix, &mul0_matrix);
        assert_eq!(ctrl_matrix, sum_matrix);
    }

    #[test]
    fn transpose_test() {
        let mut ctrl_matrix = Matrix::new(5, 2);
        ctrl_matrix.fill(15.0);

        let mut new_matrix = Matrix::new(2, 5);
        new_matrix.fill(15.0);

        let transpose_matrix = transpose(&new_matrix);
        assert_eq!(ctrl_matrix, transpose_matrix);
    }
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut build_entries = Vec::<Vec<f64>>::new();
        let mut filler_vec = Vec::<f64>::new();
        for _ in 0..rows {
            filler_vec.clear();
            for _ in 0..cols {
                filler_vec.push(0.0);
            }
            build_entries.push(filler_vec.clone());
        }

        Self {
            entries: build_entries,
            rows,
            cols,
        }
    }

    fn load(entries: Vec<Vec<f64>>) -> Result<Self, String> {
        if entries.is_empty() {
            return Err("Cannot load an empty matrix".to_string());
        }

        let rows: usize = entries.len();
        let cols: usize = entries[0].len();
        Ok(Self {
            entries,
            rows,
            cols,
        })
    }

    pub fn fill(&mut self, filler_num: f64) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.entries[i][j] = filler_num;
            }
        }
    }

    pub fn max_value(&self) -> (usize, usize) {
        let mut max_value: f64 = 0.0;
        let mut max_value_row_index: usize = 0;
        let mut max_value_col_index: usize = 0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.entries[i][j] > max_value {
                    max_value = self.entries[i][j];
                    max_value_row_index = i;
                    max_value_col_index = j;
                }
            }
        }

        (max_value_row_index, max_value_col_index)
    }

    pub fn scale(&mut self, scale_factor: f64) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.entries[i][j] *= scale_factor;
            }
        }
    }

    pub fn add_scalar(&mut self, scalar: f64) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.entries[i][j] += scalar;
            }
        }
    }
}

#[cfg(test)]
mod matrix_methods_tests {
    use super::*;

    #[test]
    fn new() {
        let new_matix = Matrix::new(3, 4);
        let mut build_entries = Vec::<Vec<f64>>::new();
        let mut filler_vec = Vec::<f64>::new();
        for _ in 0..3 {
            filler_vec.clear();
            for _ in 0..4 {
                filler_vec.push(0.0);
            }
            build_entries.push(filler_vec.clone());
        }

        assert_eq!(
            new_matix,
            Matrix {
                entries: build_entries,
                rows: 3,
                cols: 4,
            }
        );
    }

    #[test]
    fn fill() {
        let mut new_matix = Matrix::new(5, 7);
        new_matix.fill(0.5);

        let mut ctrl_entries = Vec::<Vec<f64>>::new();
        let mut ctrl_vec = Vec::<f64>::new();
        for _ in 0..5 {
            ctrl_vec.clear();
            for _ in 0..7 {
                ctrl_vec.push(0.5);
            }
            ctrl_entries.push(ctrl_vec.clone());
        }

        assert_eq!(
            new_matix,
            Matrix {
                entries: ctrl_entries,
                rows: 5,
                cols: 7,
            }
        );
    }

    #[test]
    fn load() {
        let mut new_matix = Matrix::new(5, 7);
        new_matix.fill(0.5);

        let mut ctrl_entries = Vec::<Vec<f64>>::new();
        let mut ctrl_vec = Vec::<f64>::new();
        for _ in 0..5 {
            ctrl_vec.clear();
            for _ in 0..7 {
                ctrl_vec.push(0.5);
            }
            ctrl_entries.push(ctrl_vec.clone());
        }

        let ctrl_matrix = Matrix::load(ctrl_entries).unwrap();
        assert_eq!(new_matix, ctrl_matrix);
    }

    #[test]
    fn max_value() {
        use rand::Rng;

        let max_value: f64 = 999999.9999;

        let mut ctrl_entries = Vec::<Vec<f64>>::new();
        let mut ctrl_vec = Vec::<f64>::new();

        let rows: usize = 5;
        let cols: usize = 6;
        for i in 0..rows {
            ctrl_vec.clear();
            for j in 0..cols {
                if i == 4 && j == 3 {
                    ctrl_vec.push(max_value);
                    continue;
                }
                let rand_f64: f64 = rand::thread_rng().gen_range(0.0..500.0);
                ctrl_vec.push(rand_f64);
            }
            ctrl_entries.push(ctrl_vec.clone());
        }

        let ctrl_matrix = Matrix::load(ctrl_entries).unwrap();

        assert_eq!(ctrl_matrix.max_value(), (4, 3));
    }

    #[test]
    fn scale() {
        let mut ctrl_matrix = Matrix::new(5, 7);
        ctrl_matrix.fill(35.0);

        let mut new_matrix = Matrix::new(5, 7);
        new_matrix.fill(7.0);
        new_matrix.scale(5.0);
        assert_eq!(new_matrix, ctrl_matrix);
    }

    #[test]
    fn add_scalar() {
        let mut ctrl_matrix = Matrix::new(5, 7);
        ctrl_matrix.fill(47.0);

        let mut new_matrix = Matrix::new(5, 7);
        new_matrix.fill(40.0);
        new_matrix.add_scalar(7.0);
        assert_eq!(new_matrix, ctrl_matrix);
    }
}

pub fn save_matrix(matrix: &Matrix, file_name: &str) -> Result<(), String> {
    use std::fs::write;
    use std::path::Path;

    let file_path = Path::new(file_name);

    let mut write_buffer = String::new();
    write_buffer.push_str(&format!("{}", matrix));
    write(file_path, write_buffer).expect("failed to write to file");

    Ok(())
}

pub fn load_matrix(file_name: &str) -> Result<Matrix, String> {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    use std::path::Path;

    let file_path = Path::new(file_name);
    let file_path_string = file_path.display();

    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(error) => {
            return Err(format!(
                "Could not open {} because {}",
                file_path_string, error
            ))
        }
    };

    let file_buffer = BufReader::new(file);

    let mut build_matrix = Vec::<Vec<f64>>::new();
    let mut build_row = Vec::<f64>::new();
    for line in file_buffer.lines() {
        build_row.clear();
        for value in line.unwrap().split(' ') {
            let parsed_value = match value.parse::<f64>() {
                Ok(parsed_value) => parsed_value,
                Err(..) => break,
            };
            build_row.push(parsed_value);
        }
        build_matrix.push(build_row.clone());
    }

    let new_matix = Matrix::load(build_matrix).unwrap();
    Ok(new_matix)
}

#[cfg(test)]
mod matrix_files_tests {
    //use super::*;

    #[test]
    fn save() {}

    #[test]
    fn load() {}
}

fn uniform_distribution(low: f64, high: f64) -> f64 {
    use rand::prelude::*;

    let difference = high - low;
    let scale: u64 = 10000;

    let scaled_difference: f64 = difference * scale as f64;
    let rand_number: u64 = rand::thread_rng().gen_range(1..32767);

    let uniform_distribution: f64 =
        low + ((1.0 * (rand_number as f64 % scaled_difference)) / scale as f64) as f64;
    uniform_distribution
}

pub fn randomize_matrix(matrix: &mut Matrix, demonenator: u64) {
    let min: f64 = -1.0 / (demonenator as f64).sqrt();
    let max: f64 = 1.0 / (demonenator as f64).sqrt();

    if min == 0.0 || max == 0.0 {
        println!("max or min is zero at start");
    }

    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            matrix.entries[i][j] = uniform_distribution(min, max);
        }
    }
}

pub enum Vector {
    Row,
    Column,
}

pub fn flatten_matrix(matrix: &Matrix, flag: Vector) -> Matrix {
    let mut row_size: usize = 1;
    let mut col_size: usize = 1;
    match flag {
        Vector::Row => {
            row_size = matrix.rows * matrix.cols;
            let mut flatten_matrix = Matrix::new(row_size, col_size);
            flatten_matrix.fill(0.0);

            for i in 0..matrix.rows {
                for j in 0..matrix.cols {
                    flatten_matrix.entries[i * matrix.cols + j][0] = matrix.entries[i][j];
                }
            }

            return flatten_matrix;
        }
        Vector::Column => {
            col_size = matrix.rows * matrix.cols;
            let mut flatten_matrix = Matrix::new(row_size, col_size);
            flatten_matrix.fill(0.0);

            for i in 0..matrix.rows {
                for j in 0..matrix.cols {
                    flatten_matrix.entries[0][i * matrix.cols + j] = matrix.entries[i][j];
                }
            }

            return flatten_matrix;
        }
    };
}

pub fn apply(applied_func: fn(f64) -> f64, matrix: &Matrix) -> Matrix {
    let mut applied_matrix = matrix.clone();

    for i in 0..applied_matrix.rows {
        for j in 0..applied_matrix.cols {
            applied_matrix.entries[i][j] = applied_func(matrix.entries[i][j]);
        }
    }

    applied_matrix
}
