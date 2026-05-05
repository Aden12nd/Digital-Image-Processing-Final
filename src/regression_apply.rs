use nalgebra::{self as na, DMatrix, U2, DVector};

use crate::poly_regression::PolyFunction2D;


struct ChunkPosIter {
    dimensions: (usize, usize),
    pos: (usize, usize)
}

impl ChunkPosIter {
    fn new(size: (usize, usize)) -> Self {
        ChunkPosIter{
            dimensions: size,
            pos: (0,0)
        }
    }
}

impl Iterator for ChunkPosIter {
    // We can refer to this type using Self::Item
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        
        if self.pos.1 == self.dimensions.1 {
            return None;
        }
        let ret = self.pos;
        self.pos.0 += 1;
        if self.pos.0 == self.dimensions.0 {
            self.pos.0 = 0;
            self.pos.1 += 1;
        }
        Some(ret)
    }
}

pub fn applyRegression(regres_mat: DMatrix<f64>, pixels: &Vec<f64>) -> DVector<f64> {

    let mat_shape = regres_mat.shape();
    println!("{}, {}", mat_shape.0, mat_shape.1);

    let pix_vec = DVector::from_row_slice(&pixels);
    let vec_shape = pix_vec.shape();
    println!("{}, {}", vec_shape.0, vec_shape.1);
    let coeffs = regres_mat * pix_vec;

    let shape = coeffs.shape();
    println!("{}, {}", shape.0, shape.1);

    return coeffs;

}