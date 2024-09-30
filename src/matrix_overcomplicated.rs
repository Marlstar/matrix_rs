#![allow(clippy::needless_return)]

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Matrix<const M: usize, const N: usize> {
    mat: Vec<f64>
}
impl<const M: usize, const N: usize> Matrix<M,N> {
    pub const fn new(input: Vec<f64>) -> Self {
        Self { mat: input }
    }
    
    pub const fn raw_ref(&self) -> &Vec<f64> {
        &(self.mat)
    }
}
// Methods
impl<const M: usize, const N: usize> Matrix<M,N> {

}

// Operations
impl<const M: usize, const N: usize> std::ops::Add for Matrix<M,N> {
    type Output = Matrix<M,N>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            std::iter::zip(self.mat, rhs.mat)
                .map(|(a,b)| a+b)
                .collect()
        )
    }
}
impl<const M: usize, const N: usize> std::ops::Sub for Matrix<M,N> {
    type Output = Matrix<M,N>;
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix::new(
            std::iter::zip(self.mat, rhs.mat)
                .map(|(a,b)| a-b)
                .collect()
        )
    }
}
// Multiplication implementations for various number types
impl<const M: usize, const N: usize> std::ops::Mul<usize> for Matrix<M,N> {
    type Output = Matrix<M,N>;
    fn mul(self, rhs: usize) -> Self::Output {
        Matrix::new(
            self.mat.iter().map(|a| a*(rhs as f64)).collect()
        )
    }
}
impl<const M: usize, const N: usize> std::ops::Mul<f64> for Matrix<M,N> {
    type Output = Matrix<M,N>;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix::new(
            self.mat.iter().map(|a| a*rhs).collect()
        )
    }
}
