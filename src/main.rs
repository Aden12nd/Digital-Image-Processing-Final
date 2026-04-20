use image::ImageReader;


fn main() {
    println!("Hello, world!");

    let img = match ImageReader::open("images/image.png") {
        Ok(i) => match i.decode() {
                Ok(decoded) => decoded,
            Err(_) => {
                println!("Failed to decode image\n");
                return;
            } 
        },
        Err(_) => {
            println!("Failed to open image\n");
            return;
        }
    };
    let img2 = img.blur(5.0);
    match img2.save("images/image-out.png") {
        Ok(_) => {},
        Err(_) => {
            println!("Failed to save image\n");
            return;
        },
    }

}
