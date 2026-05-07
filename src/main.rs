// use std::{iter::Zip, path::Path};
use nalgebra::{self as na, DMatrix, U2, DVector, dmatrix};
use std::vec::Vec;
mod QuadTree;
use image::{DynamicImage, ImageBuffer, ImageReader, RgbImage};

use crate::imagematrix::{create_matrix, create_matrix_n, create_regression_matrix};

// use crate::{regression_apply::applyRegression};
mod image_chunking;
mod poly_regression;
mod imagematrix;
mod regression_apply;
mod cut;


fn qt_builder<'b, 'a: 'b>(img: &'a DynamicImage) -> Box<dyn Fn((usize, usize)) -> image_chunking::Chunk<'a> + 'b> {
    Box::new(|pos| image_chunking::Chunk::<'a>::new(img, (pos.0 as u32, pos.1 as u32), 1))
}

fn qt_test_builder(pos: (usize, usize)) -> (usize, usize) {
    pos
} 

// fn([&Node<T>;4]) -> Option<Node<T>>



// fn collapse<'a>(children: [&'a QuadTree::Node<image_chunking::Chunk<'a>>;4], reg_mat: &'a DMatrix<f64>) -> Option<QuadTree::Node<image_chunking::Chunk<'a>>> {
    
//     let mut chunks: [&'a image_chunking::Chunk<'a>; 4] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
//     let mut children_cost_r = 0.0;
//     let mut children_cost_g = 0.0;
//     let mut children_cost_b = 0.0;
//     for (i, child) in children.iter().enumerate() {
//         if let QuadTree::Node::Terminal(child_chunk) = *child {
//             children_cost_r += child_chunk.regression_red.cost;
//             children_cost_g += child_chunk.regression_green.cost;
//             children_cost_b += child_chunk.regression_blue.cost;
//             chunks[i] = child_chunk;
//         } else {
//             return None;
//         }
//     }
//     let mut new_chunk = image_chunking::Chunk::new_combine(chunks);

//     if new_chunk.regression_red.cost + new_chunk.regression_red.cost + new_chunk.regression_red.cost
//         < children_cost_r + children_cost_g + children_cost_b {

//         return Some(QuadTree::Node::Terminal(new_chunk));
//     } else {
//         None
//     }
    
// }

// fn make_collapser<'c, 'b: 'c, 'a: 'b+'c>(reg_mat: DMatrix<f64>) -> Box<dyn Fn(&[&'a QuadTree::Node<image_chunking::Chunk<'a>>;4]) -> Option<QuadTree::Node<image_chunking::Chunk<'a>>> + 'static> {
//     Box::new(|children: &[&'a QuadTree::Node<image_chunking::Chunk<'_>>; 4]| 
// }

// pub fn make_collapser<'a>(reg_mat: &'a DMatrix<f64>) -> Box<dyn Fn([&QuadTree::Node<image_chunking::Chunk>;4]) -> Option<QuadTree::Node<image_chunking::Chunk<'a>>> + 'a> {
//     Box::new(move |nodes: [&'a QuadTree::Node<image_chunking::Chunk<'a>>; 4]| collapse(nodes, reg_mat))
// }


fn quad_to_img(qt: QuadTree::QuadTree<image_chunking::Chunk>, width: u32, height: u32) -> DynamicImage {
    let mut img: RgbImage = ImageBuffer::new(width, height);

    for chunk in qt.iter() {
        for x in 0..chunk.size {
            for y in 0..chunk.size {
                let rgb: [u8;3] = [
                    chunk.regression_red.predict(x as f64, y as f64) as u8,
                    chunk.regression_green.predict(x as f64, y as f64) as u8,
                    chunk.regression_blue.predict(x as f64, y as f64) as u8,
                ];
                // println!("rgb {} {} {}", rgb[0], rgb[1], rgb[2]);
                // println!("rgb {:?}", chunk.regression_red.);
                img.put_pixel(
                    x + chunk.coordinate.0,
                    y + chunk.coordinate.1,
                    image::Rgb(rgb)
                );
            }
        }
    }

    image::DynamicImage::ImageRgb8(img)

}


fn main() {

    let img: DynamicImage = match ImageReader::open("images/pexels-noise.png").unwrap().decode() {
        Ok(decoded) => decoded,
        Err(_) => panic!("Encountered error in decoding image"),
    };

    let width = img.width();
    let height = img.height();

    let builder = qt_builder(&img);
    println!("{} {}", width, height);

    let mut qt = QuadTree::QuadTree::new_grid(width as usize, height as usize, &builder);
    let depth = qt.depth();

    println!("Min depth is {}", qt.min_depth());

    // let collapser = make_collapser(reg_mat);

    // print.
    println!("Starting collapsing {}", depth);


    let mut collapsed: usize = 1;
    if depth >= 1 {
        let d = depth-1;
        let size = 1 << (depth-d);

        println!("Collapsing at size {}", size);
        println!("Max polynomial degree size is {}", 1);
        let mats = imagematrix::MatPack::new(size, 1);
        collapsed = qt.collapse_depth(d, &mats);
        println!("Collapsed {} quads\n\n", collapsed);

    }

    if depth >= 2 && collapsed != 0 {
        let d = depth-2;
        let size = 1 << (depth-d);

        println!("Collapsing at size {}", size);
        println!("Max polynomial degree size is {}", 3);
        let mats = imagematrix::MatPack::new(size, 3);
        collapsed = qt.collapse_depth(d, &mats);
        println!("Collapsed {} quads\n\n", collapsed);

    }

    for di in 3..=(depth-1) {
        if collapsed == 0 {
            break;
        }
        let d = depth-di;
        let size = 1 << (depth-d);

        println!("Collapsing at size {}", size);
        println!("Max polynomial degree size is {}", 7);
        let mats = imagematrix::MatPack::new(size, 7);
        collapsed = qt.collapse_depth(d, &mats);
        println!("Collapsed {} quads\n\n", collapsed);
        // let order_mat = imagematrix::create_matrix(size, size);
        // let reg_mat = imagematrix::create_regression_matrix(&order_mat);
        // match reg_mat {
        //     Some(reg) => {
        //         println!("Collapsing at size {}\n\n", size);
        //         // println!("reg_mat:\n{}\n", reg);
        //         let collapsed = qt.collapse_depth(d, &reg, &order_mat);
        //         println!("Collapsed {} quads", collapsed);
        //         if collapsed == 0 {
        //             break;
        //         }
        //     }
        //     None => {
        //         panic!("Failed to get reg_mat");
        //     }
        // }
    }
    println!("New Min depth is {}", qt.min_depth());
    println!("New depth is {}", qt.depth());

    println!("Creating result image");
    let res_img = quad_to_img(qt, width, height);
    match res_img.save("images/pexels-noise-out3.png") {
        Ok(_) => println!("Saved image"),
        Err(_) => println!("Failed to save image"),
    }



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

    // let mut quad_tree: QuadTree::QuadTree<(usize, usize)> = QuadTree::QuadTree::new_grid(17,9, &qt_test_builder);
    // println!("depth: {}", quad_tree.depth());
    // println!("min depth {}", quad_tree.min_depth());
    // for (x, y) in quad_tree.iter() {
    //     println!("Terminal: ({0}, {1})", x, y);
    // }
    // for quad in quad_tree.iter_depth(3) {
    //     // let quad;
    //     // if let QuadTree::Node::Quad(q) = node {
    //     //     quad = q;
    //     // } else {
    //     //     panic!("expected quad");
    //     // }
    //     println!("Quad: {0:?}", quad.depth());
    //     // *node = QuadTree::Node::Empty;
    // }
} 

