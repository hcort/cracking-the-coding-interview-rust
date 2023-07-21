// rotate Matrix: Given an image represented by an HxH matrix, where each pixel in the image is 4
// bytes, write a method to rotate the image by 90 degrees. Can you do this in place?
// Hints: #51, #100

fn rotate_matrix_transpose(matrix: &[Vec<u32>]) -> Vec<Vec<u32>> {

    let mut rotated_image = matrix.to_owned();

    let x_len = rotated_image.len();
    for x in 0..x_len
    {
        for y in x..x_len
        {
            let temp = rotated_image[x][y];
            rotated_image[x][y] = rotated_image[y][x];
            rotated_image[y][x] = temp;
        }
    }

    for row in rotated_image.iter_mut().take(x_len)
    {
        for y in 0..x_len/2
        {
            row.swap(y, x_len-y-1);
        }
    }
    rotated_image
}

fn rotate_image_90_degrees(image: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut rotated_image = image.to_owned();

    let x_len = rotated_image.len();
    for x in 0..(x_len / 2) {
        let y_len = rotated_image[x].len();
        for y in 0..y_len {
            rotated_image[x][y] ^= rotated_image[x_len - x - 1][y];
            rotated_image[x_len - x - 1][y] ^= rotated_image[x][y];
            rotated_image[x][y] ^= rotated_image[x_len - x - 1][y];
        }
    }
    for x in 0..x_len {
        let y_len = rotated_image[x].len();
        for y in 0..y_len {
            if y <= x {
                continue;
            }
            rotated_image[x][y] ^= rotated_image[y][x];
            rotated_image[y][x] ^= rotated_image[x][y];
            rotated_image[x][y] ^= rotated_image[y][x];
        }
    }

    rotated_image
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rotate_matrix() {
        let image4 = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let image3 = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let rotated_image = rotate_image_90_degrees(&image4);
        let rotated_image3 = rotate_matrix_transpose(&image3);
        let rotated_image4 = rotate_matrix_transpose(&image4);

        let expected_rotated_image4 = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];

        let expected_rotated_image3 = vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ];

        assert_eq!(rotated_image, expected_rotated_image4);
        assert_eq!(rotated_image4, expected_rotated_image4);
        assert_eq!(rotated_image3, expected_rotated_image3);
    }
}

fn main() {
    //
}