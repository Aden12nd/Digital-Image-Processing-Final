use image::ImageReader;

mod QuadTree;

fn qt_builder(x: usize, y: usize) -> usize {
    return x + y;
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
    // match img2.save("images/image-out.png") {
    //     Ok(_) => {},
    //     Err(_) => {
    //         println!("Failed to save image\n");
    //         return;
    //     },
    // }

    let quad_tree = QuadTree::QuadTree::newGrid(14, 17, qt_builder);
    for i in quad_tree.iter() {
        
    }
// 
}
