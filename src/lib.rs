#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    entries: Vec<Vec<f64>>,
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

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            entries: Vec::<Vec<f64>>::new(),
            rows,
            cols,
        }
    }

    fn load(rows: usize, cols: usize, entries: Vec<Vec<f64>>) -> Self {
        Self {
            entries,
            rows,
            cols,
        }
    }

    pub fn build(&mut self) {
        if self.entries.is_empty() {
            let mut filler_vec = Vec::<f64>::new();
            for _ in 0..self.rows {
                filler_vec.clear();
                for _ in 0..self.cols {
                    filler_vec.push(0.0);
                }
                self.entries.push(filler_vec.clone());
            }
        }
    }

    pub fn fill(&mut self, filler_num: f64) {
        let mut filler_vec = Vec::<f64>::new();
        for _ in 0..self.rows {
            filler_vec.clear();
            for _ in 0..self.cols {
                filler_vec.push(filler_num);
            }
            self.entries.push(filler_vec.clone());
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
}

#[cfg(test)]
mod matrix_methods_tests {
    use super::*;

    #[test]
    fn new() {
        let new_matix = Matrix::new(3, 4);
        assert_eq!(
            new_matix,
            Matrix {
                entries: Vec::<Vec<f64>>::new(),
                rows: 3,
                cols: 4,
            }
        );
    }

    #[test]
    fn fill() {
        let mut new_matix = Matrix::new(3, 4);
        new_matix.fill(0.5);

        let mut ctrl_entries = Vec::<Vec<f64>>::new();
        let mut ctrl_vec = Vec::<f64>::new();
        for _ in 0..3 {
            ctrl_vec.clear();
            for _ in 0..4 {
                ctrl_vec.push(0.5);
            }
            ctrl_entries.push(ctrl_vec.clone());
        }

        assert_eq!(
            new_matix,
            Matrix {
                entries: ctrl_entries,
                rows: 3,
                cols: 4,
            }
        );
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

        let ctrl_matrix = Matrix::load(rows, cols, ctrl_entries);

        assert_eq!(ctrl_matrix.max_value(), (4, 3));
    }
}

pub fn save_matrix(matrix: &Matrix, file_name: &str) -> Result<(), String> {
    use std::fs::write;
    use std::path::Path;

    let file_path = Path::new(file_name);

    let mut write_buffer = String::new();
    write_buffer.push_str(&format!("{}\n", matrix.rows.to_string()));
    write_buffer.push_str(&format!("{}\n", matrix.cols.to_string()));
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

    let mut count: usize = 0;
    let mut row_size: usize = 0;
    let mut cols_size: usize = 0;
    let mut build_matrix = Vec::<Vec<f64>>::new();
    let mut build_row = Vec::<f64>::new();
    for line in file_buffer.lines() {
        if count == 0 {
            row_size = line.unwrap().parse::<u32>().unwrap() as usize;
            count += 1;
        } else if count == 1 {
            cols_size = line.unwrap().parse::<u32>().unwrap() as usize;
            count += 1;
        } else {
            build_row.clear();
            for value in line.unwrap().split(' ') {
                let parsed_value = match value.parse::<f64>() {
                    Ok(parsed_value) => parsed_value,
                    Err(..) => break,
                };
                build_row.push(parsed_value);
            }
            build_matrix.push(build_row.clone());
            count += 1;
        }
    }

    let new_matix = Matrix::load(row_size, cols_size, build_matrix);
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
