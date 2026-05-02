use std::path::Path;

use image::GenericImageView;
use image::ImageError;
use image::ImageReader;
use image::Rgba;

const CHUNK_SIZE : usize = 16;

pub struct Chunk {
    pub top_left : u32,
    pub pix_data : Vec<Rgba<u8>>
}

pub fn img_to_chunks(img : &Path) -> Result<Vec<Chunk>, ImageError> {
    let img = ImageReader::open(img)?.decode()?;
    let width = img.width();
    let pixels = img.pixels();
    let mut chunks: Vec<Chunk> = vec![];
    for (i, px) in pixels.enumerate() {
        let chunk_idx = i%(width as usize)/CHUNK_SIZE;
        if chunk_idx < chunks.len() {
            chunks[chunk_idx].pix_data.push(px.2);
        }
        else {
            chunks.push(Chunk { top_left: (i as u32), pix_data: (vec![]) });
        }
    }
    Ok(chunks)
}