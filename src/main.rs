use std::path::Path;
use nalgebra::{self as na, DMatrix, U2, DVector, dmatrix};
use std::vec::Vec;
mod QuadTree;
use image::ImageReader;
mod image_chunking;
mod poly_regression;
mod imagematrix;

fn qt_builder(x: usize, y: usize) -> usize {
    return x + y;
}

fn main() {
    //let test = dmatrix![1 as f64,1 as f64,1 as f64,1 as f64];
    let testMat : DMatrix<f64> = imagematrix::create_matrix_n(2,2,2);
    //let testMat : DMatrix<f64> = imagematrix::matrixOp(imagematrix::create_matrix_n(2,2,2));
    println!("Hello, world!");
    //imagematrix::matVecMult(testMat, test);
    // let img = match ImageReader::open("images/image.png") {
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

    // let quad_tree = QuadTree::QuadTree::newGrid(14, 17, qt_builder);
    // for i in quad_tree.iter() {
        
    // }
} 

