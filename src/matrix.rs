extern crate nalgebra as na;

use na::{DMatrix, Dyn, Matrix, VecStorage, LU};

fn matrix_to_vector(matrix: DMatrix<f64>) -> Vec<Vec<f64>> {
    let rows: usize = matrix.nrows();
    let mut vec_vec: Vec<Vec<f64>> = Vec::with_capacity(rows);
    for row in matrix.row_iter() {
        let row_vec: Vec<f64> = row.iter().cloned().collect();
        vec_vec.push(row_vec);
    }
    vec_vec
}

fn vector_to_matrix(vec_vec: Vec<Vec<f64>>) -> DMatrix<f64> {
    let rows: usize = vec_vec.len();
    let cols: usize = vec_vec[0].len();
    let mut data: Vec<f64> = Vec::with_capacity(rows * cols);
    for row_vec in vec_vec {
        data.extend_from_slice(&row_vec);
    }
    let matrix = DMatrix::from_row_slice(rows, cols, &data);
    matrix
}

pub fn solve_matrix(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
    let a_mat: Matrix<f64, Dyn, Dyn, VecStorage<f64, Dyn, Dyn>> = vector_to_matrix(a);
    let b_mat: Matrix<f64, Dyn, Dyn, VecStorage<f64, Dyn, Dyn>> = vector_to_matrix(b);
    if a_mat.is_square() && a_mat.nrows() == b_mat.nrows() {
        let decomp: LU<f64, Dyn, Dyn> = a_mat.lu();
        let x_mat_opt = decomp.solve(&b_mat);
        match x_mat_opt {
            None => None,
            Some(x_mat) => Some(matrix_to_vector(x_mat.clone())),
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_matrix() {
        let a = vec![vec![1.0, 1.0], vec![2.0, 1.0]];
        let b = vec![vec![0.0], vec![1.0]];
        let solution = solve_matrix(a, b).unwrap();
        assert_eq!(solution[0][0], 1.0);
        assert_eq!(solution[1][0], -1.0);

        let a = vec![vec![1.0], vec![1.0]];
        let b = vec![vec![1.0], vec![0.5]];
        let solution = solve_matrix(a, b);
        assert!(solution.is_none());
    }
}
