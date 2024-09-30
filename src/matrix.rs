#[allow(dead_code, clippy::needless_return, non_snake_case)]
#[derive(Debug, Clone)]
pub struct Matrix {
    M: usize,
    N: usize,
    elements: Vec<f64>
}
// Constructors
impl Matrix {
    pub fn new(m: usize, n: usize, elements: Vec<f64>) -> Self {
        Self {
            M: m,
            N: n,
            elements
        }
    }
}
// Methods
impl Matrix {

}
// Operations
impl std::ops::Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix::new(
            self.M, self.N,
            std::iter::zip(self.elements, rhs.elements)
                .map(|(a,b)| a+b)
                .collect()
        )
    }
}
impl std::ops::Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix::new(
            self.M, self.N,
            std::iter::zip(self.elements, rhs.elements)
                .map(|(a,b)| a-b)
                .collect()
        )
    }
}
// Multiplication implementations for various number types
impl std::ops::Mul<usize> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: usize) -> Self::Output {
        Matrix::new(
            self.M, self.N,
            self.elements.iter().map(|a| a*(rhs as f64)).collect()
        )
    }
}
