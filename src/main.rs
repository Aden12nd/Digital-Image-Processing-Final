use std::path::Path;

mod QuadTree;
use image::ImageReader;
mod image_chunking;
mod poly_regression;

fn qt_builder(x: usize, y: usize) -> (usize, usize) {
    println!("called with {0}, {1}\n", x, y);
    return (x, y);
}

fn main() {
    println!("Hello, world!");

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
    
    // match img2.save("images/image-out.png") {
    //     Ok(_) => {},
    //     Err(_) => {
    //         println!("Failed to save image\n");
    //         return;
    //     },
    // }

    let quad_tree = QuadTree::QuadTree::newGrid(17,9, qt_builder);
    for (x, y) in quad_tree.iter() {
        println!("Terminal: ({0}, {1})", x, y);
    }
    for quad in quad_tree.iter_depth(3) {
        println!("Quad: {0:?}", quad.depth());
    }
}

