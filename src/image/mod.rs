use crate::matrix::Matrix;

#[derive(Debug, PartialEq, Clone)]
pub struct Image {
    pub img_data: Matrix,
    pub label: usize,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Image Label:{}\nImage Data:\n{}",
            self.label, self.img_data
        )
    }
}

impl Image {
    pub fn new(img_data: Matrix, label: usize) -> Self {
        Self { img_data, label }
    }
}

#[cfg(test)]
mod image_new_test {
    use super::*;

    #[test]
    fn new() {
        let image_matrix = Matrix::new(3, 4);
        let image_matrix_clone = image_matrix.clone();
        let new_image = Image::new(image_matrix, 0);

        assert_eq!(
            new_image,
            Image {
                img_data: image_matrix_clone,
                label: 0,
            }
        )
    }
}

pub fn csv_to_images(file_path: &str) -> Result<Vec<Image>, String> {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    use std::path::Path;

    let IMAGE_WIDTH: usize = 28;
    let IMAGE_HEIGHT: usize = 28;

    let file_path = Path::new(file_path);
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

    let mut images = Vec::<Image>::new();
    let mut build_matrix = Vec::<Vec<f64>>::new();
    let mut build_row = Vec::<f64>::new();
    let mut label: usize = 0;
    let mut count: usize = 0;

    for line in file_buffer.lines() {
        build_row.clear();
        for value in line.unwrap().split(',') {
            let parsed_value = match value.parse::<f64>() {
                Ok(parsed_value) => parsed_value / 256.0,
                Err(..) => break,
            };
            build_row.push(parsed_value);
        }
        build_matrix.push(build_row.clone());
        if count % IMAGE_HEIGHT == 0 {
            images.push(Image::new(
                Matrix::build_from_matrix(build_matrix.clone()).unwrap(),
                label,
            ));
            build_matrix.clear();
            label += 1;
        }
        count += 1;
    }

    Ok(images)
}

#[cfg(test)]
mod image_csv_to_image_test {
    use super::*;

    #[test]
    fn csv_to_images_test() {
        let IMAGE_WIDTH: usize = 28;
        let IMAGE_HEIGHT: usize = 28;

        let mut ctrl_images = Vec::<Image>::new();
        for label in 0..8 {
            let mut ctrl_matrix = Matrix::new(IMAGE_HEIGHT, IMAGE_WIDTH);
            ctrl_matrix.fill(4.32 / 256.0);
            ctrl_images.push(Image::new(ctrl_matrix, label));
        }

        let new_images = csv_to_images("./src/image/TEST_FILE_csv_to_images.txt").unwrap();
        assert_eq!(ctrl_images, new_images);
    }
}
