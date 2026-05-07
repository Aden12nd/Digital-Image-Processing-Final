use nalgebra::{self as na, DMatrix, U2, DVector};
use strum::IntoEnumIterator;

use crate::{
    cut::{Cut, belongs_to_first, cut_segment}, 
    image_chunking::{Chunk, pix_to_channels}, 
    poly_regression::{ChunkRegression, PolyFunction2D},
    imagematrix::{create_regression_matrix, create_matrix}
};


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

// pub fn applyRegression(regres_mat: DMatrix<f64>, pixels: &Vec<f64>) -> DVector<f64> {

//     let mat_shape = regres_mat.shape();
//     println!("{}, {}", mat_shape.0, mat_shape.1);

//     let pix_vec = DVector::from_row_slice(&pixels);
//     let vec_shape = pix_vec.shape();
//     println!("{}, {}", vec_shape.0, vec_shape.1);
//     let coeffs = regres_mat * pix_vec;

//     let shape = coeffs.shape();
//     println!("{}, {}", shape.0, shape.1);

//     return coeffs;

// }


fn apply_regression_channel(regres_mat: &DMatrix<f64>, order_mat: &DMatrix<f64>, degree: usize, pix_values: &Vec<f64>) -> ChunkRegression {
    let coeffs = regres_mat*DVector::from_row_slice(pix_values);
    let predict = order_mat * &coeffs;
    let predict_vec = predict.as_slice();
    let mut MSE = 0.0;
    for (actual, predict) in pix_values.iter().zip(predict_vec) {
        // MSE += (actual - predict).abs();
        MSE += (actual - predict).powi(2);
    }
    MSE /= pix_values.len() as f64;
    // println!("{:?}", pix_values);
    // println!("{:?}", predict_vec);
    // println!("MSE: {}", MSE);
    let poly = ChunkRegression::new_global(degree, Vec::from(coeffs.as_slice()), MSE);
    poly
}

pub fn apply_regression_to_chunk(regres_mat: &DMatrix<f64>, order_mat: &DMatrix<f64>, degree: usize, chunk: &mut Chunk, chunk_cut: Option<Cut>) {
    if let Some(cut) = chunk_cut {
        // Split pixels vec into two vecs from each side of the cut
        let (lhs, _rhs) = cut_segment(chunk.pixels(), chunk.size as usize, cut);

        let (r, g, b) = pix_to_channels(lhs);
        let red_poly = apply_regression_channel(regres_mat, order_mat, degree, &r);
        let green_poly = apply_regression_channel(regres_mat, order_mat, degree, &g);
        let blue_poly = apply_regression_channel(regres_mat, order_mat, degree, &b);
        chunk.load_regression(red_poly, green_poly, blue_poly);

    } else {
        let (r, g, b) = pix_to_channels(chunk.pixels());
        let red_poly = apply_regression_channel(regres_mat, order_mat, degree, &r);
        let green_poly = apply_regression_channel(regres_mat, order_mat, degree, &g);
        let blue_poly = apply_regression_channel(regres_mat, order_mat, degree, &b);
        chunk.load_regression(red_poly, green_poly, blue_poly);
    }
}

pub fn apply_cuts_on_chunks(chunk: &mut Chunk) {
    for cut in Cut::iter() {
        let degree = 2;
        let X = create_matrix(2,2);
        let new_regres_mat = match create_regression_matrix(&X) {
            Some(regres_mat) => regres_mat,
            None => return 
        };
        apply_regression_to_chunk(&new_regres_mat, &X, degree, chunk, Some(cut));
    }
}
