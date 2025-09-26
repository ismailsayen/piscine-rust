
pub use lalgebra_scalar::*;
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Clone > Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![])
    }
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut res: Vec<T> = vec![];
        for i in 0..self.0.len() {
            res.push(self.0[i][n].clone())
        }
        res
    }
}

impl<T: Scalar<Item = T> + Mul<Output = T> + Copy + Add<Output = T>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn mul(self, other: Self) -> Self::Output {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        let mut matrix = Matrix::new();

        for i in 0..self.number_of_rows() {
            let mut row_matrix = vec![];

            for j in 0..self.number_of_cols() {
                let mut sum = T::zero();

                let row = self.row(i);
                let col = other.col(j);
                for k in 0..self.number_of_cols() {
                    sum = sum + (row[k] * col[k]);
                }

                row_matrix.push(sum);
            }
            matrix.0.push(row_matrix);
        }

        Some(matrix)
    }
}
