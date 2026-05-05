// use std::{iter::Zip, path::Path};
use nalgebra::{self as na, DMatrix, U2, DVector, dmatrix};
use std::vec::Vec;
mod QuadTree;
use image::{DynamicImage, ImageReader};

// use crate::{regression_apply::applyRegression};
mod image_chunking;
mod poly_regression;
mod imagematrix;
mod regression_apply;
mod cut;

fn qt_builder<'a>(img: &'a DynamicImage) -> Box<dyn Fn((usize, usize)) -> image_chunking::Chunk<'a> + 'a> {
    Box::new(move |pos| image_chunking::Chunk::new(img, (pos.0 as u32, pos.1 as u32), 1))
}

fn qt_test_builder(x: usize, y: usize) -> (usize, usize) {
    (x,y)
} 

fn main() {

    // let img: ImageReader<std::io::BufReader<std::fs::File>> = ImageReader::open("images/image.png").unwrap();

    // let builder = qt_builder(img);

    // let mut qt: QuadTree::QuadTree<image_chunking::Chunk> = QuadTree::QuadTree::newGrid(xWidth, yWidth, init)


    // // let test = dmatrix![1 as f64,1 as f64,1 as f64,1 as f64];
    // // let test = DMatrix::from_column_slice(2, 2, &[1.0, 2.0, 3.0, 4.0]);
    // let pixels = vec![
    //     2.0, 1.5,
    //     1.5, 0.0
    // ];
    // let pixel_order_mat = imagematrix::create_matrix(2,2);
    // println!("{}", pixel_order_mat);
    // println!("{}", pixel_order_mat.shape().0);
    
    // let regres_mat_opt  = imagematrix::create_regression_matrix(pixel_order_mat.clone());
    
    // let regres_mat: DMatrix<f64>;
    // if let Some(mat) = regres_mat_opt {
    //     regres_mat = mat;
    // } else {
    //     println!("Create regression matrix failed");
    //     return;
    // }

    // println!("Created mats");
    // println!("{}", regres_mat);
    
    // let coeffs = applyRegression(regres_mat, &pixels);

    // println!("Coefficients:\n{}", coeffs);
    // let predict_pixels = pixel_order_mat * coeffs;
    // println!("Predicted pixels:\n{}", predict_pixels);

    // let mut MSE: f64 = 0.0;
    // for (predict, actual) in predict_pixels.as_slice().into_iter().zip(&pixels) {
    //     let tmp = predict - actual;
    //     MSE += tmp*tmp;
    // }
    // MSE /= pixels.len() as f64;
    // println!("MSE: {}", MSE);

    // cut::test_cut();



    // println!("Hello, world!");
    // imagematrix::matVecMult(testMat, test);
    //     let img = match ImageReader::open("images/image.png") {
    //     Ok(i) => match i.decode() {
    //             Ok(decoded) => decoded,
    //         Err(_) => {
    //             println!("Failed to decode image\n");
    //             return;
    //         } 
    //     },
    //     Err(_) => {
    //         println!("Failed to open image\n");
    //         return;
    //     }
    // };
    // let img2 = img.blur(5.0);
    // match image_chunking::img_to_chunks(Path::new("images/image.png")) {
    //     Ok (chunks_vec) => {
    //         for (i, chunk) in chunks_vec.iter().enumerate() {
    //             print!("Chunk {0}: \n{1}\n{2:?}\n", i, chunk.top_left, chunk.pix_data.len());
    //         }
    //     },
    //     Err(_) => {
    //         println!("You errored out, chud.");
    //         return;
    //     }
    // }
    
    // match img2.save("images/image-out1.png") {
    //     Ok(_) => {},
    //     Err(_) => {
    //         println!("Failed to save image\n");
    //         return;
    //     },
    // }

    let mut quad_tree: QuadTree::QuadTree<(usize, usize)> = QuadTree::QuadTree::new_grid(17,9, qt_test_builder);
    // for (x, y) in quad_tree.iter() {
    //     println!("Terminal: ({0}, {1})", x, y);
    // }
    for quad in quad_tree.iter_depth(3) {
        // let quad;
        // if let QuadTree::Node::Quad(q) = node {
        //     quad = q;
        // } else {
        //     panic!("expected quad");
        // }
        println!("Quad: {0:?}", quad.depth());
        // *node = QuadTree::Node::Empty;
    }
} 

