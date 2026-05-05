use nalgebra::{self as na, DMatrix, U2, DVector};

use crate::{image_chunking::{Chunk, pix_to_channels}, poly_regression::{ChunkRegression, PolyFunction2D}};


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

pub fn applyRegressionToChunk(regres_mat: &DMatrix<f64>, degree: usize, chunk: Chunk) -> (DVector<f64>, DVector<f64>, DVector<f64>) {

    let (r, g, b) = pix_to_channels(chunk.pixels());
    let red_coeffs = regres_mat*DVector::from_row_slice(&r);
    let blue_coeffs = regres_mat*DVector::from_row_slice(&g);
    let green_coeffs = regres_mat*DVector::from_row_slice(&b);

    // The actual arguments need to figured out
    let red_poly = PolyFunction2D::from(degree, Vec::from(red_coeffs.as_slice()));
    let blue_poly = PolyFunction2D::from(degree, Vec::from(blue_coeffs.as_slice()));
    let green_poly = PolyFunction2D::from(degree, Vec::from(green_coeffs.as_slice()));

    return (red_coeffs, blue_coeffs, green_coeffs);

}