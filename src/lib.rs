use std::{fmt, fs::write, fs::File, io::prelude::*, io::BufReader, path::Path};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    entries: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
}

pub fn save_matrix(matrix: &Matrix, file_name: &str) -> Result<(), String> {
    let file_path = Path::new(file_name);

    let mut write_buffer = String::new();
    write_buffer.push_str(&format!("{}\n", matrix.rows.to_string()));
    write_buffer.push_str(&format!("{}\n", matrix.cols.to_string()));
    write_buffer.push_str(&format!("{}", matrix));
    write(file_path, write_buffer).expect("failed to write to file");

    Ok(())
}

pub fn load_matrix(file_name: &str) -> Result<Matrix, String> {
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
mod tests {
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
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
