use crate::matrix::Matrix;

#[derive(Debug, PartialEq, Clone)]
pub struct Image {
    img_data: Matrix,
    label: usize,
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
mod image_tests {
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

pub fn csv_to_images(file_path: &str, number_of_images: usize) -> Vec<Image> {
    todo!()
}
