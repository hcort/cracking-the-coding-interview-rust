
// 1.8 Zero Matrix: Write an algorithm such that if an element in an MxN matrix is 0, its entire row and
// column are set to 0.
// Hints: #17, #74, #102
// _ pg 204


fn zero_matrix(matrix: &[Vec<u32>]) -> Vec<Vec<u32>> {

    let mut zeroed_matrix = matrix.to_owned();

    let x_len = zeroed_matrix.len();
    let y_len = zeroed_matrix[0].len();
    let mut zeroed_rows: Vec<bool> = vec![false; x_len];
    let mut zeroed_cols: Vec<bool> = vec![false; y_len];

    for x in 0..x_len
    {
        for (y, item) in zeroed_matrix[x].iter_mut().enumerate().skip(x)
        {
            if *item == 0 {
                zeroed_rows[x] = true;
                zeroed_cols[y] = true;
            }
        }
    }

    //for x in 0..x_len
    for (x, item_r) in zeroed_rows.iter().enumerate()
    {
        for (y, item_c) in zeroed_cols.iter().enumerate()
        {
            if *item_r || *item_c {
                zeroed_matrix[x][y] = 0;
            }
        }
    }
    zeroed_matrix
}

fn zero_row_col(mat: &mut [Vec<u32>], x: usize, y: usize)
{
    for row in mat.iter_mut() {
        row[y] = 0;
    }

    // Zero out column
    for i in 0..mat[x].len() {
        mat[x][i] = 0;
    }
}

fn zero_matrix_no_extra(matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {

    let mut zeroed_matrix = matrix.to_owned();

    for (x, row) in matrix.iter().enumerate()
    {
        for (y, item) in row.iter().enumerate()
        {
            if *item == 0 {
                zero_row_col(&mut zeroed_matrix, x, y);
            }
        }
    }
    zeroed_matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zero_matrix() {
        let image1 = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 0, 12],
            vec![13, 14, 15, 16],
        ];
        let image2 = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![9, 10, 0, 12],
            vec![13, 14, 15, 16],
        ];
        let image3 = vec![
            vec![0, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 0, 12],
            vec![13, 14, 15, 0],
        ];
        let image4 = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];



        let expected_image1 = vec![
            vec![1, 2, 0, 4],
            vec![5, 6, 0, 8],
            vec![0, 0, 0, 0],
            vec![13, 14, 0, 16],
        ];
        let expected_image2 = vec![
            vec![1, 0, 0, 4],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![13, 0, 0, 16],
        ];
        let expected_image3 = vec![
            vec![0, 0, 0, 0],
            vec![0, 6, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let expected_image4 = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];

        let rotated_image1 = zero_matrix(&image1);
        let rotated_image2 = zero_matrix(&image2);
        let rotated_image3 = zero_matrix(&image3);
        let rotated_image4 = zero_matrix(&image4);
        assert_eq!(rotated_image1, expected_image1);
        assert_eq!(rotated_image2, expected_image2);
        assert_eq!(rotated_image3, expected_image3);
        assert_eq!(rotated_image4, expected_image4);

        let rotated_image1 = zero_matrix_no_extra(&image1);
        let rotated_image2 = zero_matrix_no_extra(&image2);
        let rotated_image3 = zero_matrix_no_extra(&image3);
        let rotated_image4 = zero_matrix_no_extra(&image4);
        assert_eq!(rotated_image1, expected_image1);
        assert_eq!(rotated_image2, expected_image2);
        assert_eq!(rotated_image3, expected_image3);
        assert_eq!(rotated_image4, expected_image4);
    }
}

fn main() {
    //
}