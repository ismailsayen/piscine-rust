pub use lalgebra_scalar::*;
#[derive(Debug,PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![Vec::new()])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut v: Vec<Vec<T>> = vec![];
        for _r in 0..row {
            let mut mat: Vec<T> = vec![];
            for _c in 0..col {
                mat.push(T::zero());
            }
            v.push(mat);
        }
        Matrix(v)
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut v: Vec<Vec<T>> = vec![];
        for r in 0..n {
            let mut mat: Vec<T> = vec![];
            for c in 0..n {
                if r==c{
                    mat.push(T::one());
                }else{
                    mat.push(T::zero());

                }
            }
            v.push(mat);
        }
        Matrix(v)
    }
}
