use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba, SubImage};

pub struct Chunk<'a> {
    pub image: &'a DynamicImage,
    pub coordinate: (u32, u32),
    pub size: u32
}

impl<'a> Chunk<'a> {
    pub fn new(image: &'a DynamicImage, coordinate: (u32, u32), size : u32) -> Self {
        Self { image, coordinate, size}
    }

    pub fn pixels(&self) -> Vec<Rgba<u8>> {
        let mut out: Vec<Rgba<u8>>  = Vec::with_capacity((self.size*self.size) as usize);

        let img_size = self.image.dimensions();

        // If the 
        if self.coordinate.0 > img_size.0 || self.coordinate.1 > img_size.1 {
            return out
        }

        for p_x in self.coordinate.0 .. self.coordinate.0+self.size {
            for p_y in self.coordinate.1 .. self.coordinate.1+self.size {
                if p_x > img_size.0 || p_y > img_size.1 {
                    return out;
                } else {
                    out.push(self.image.get_pixel(p_x, p_y));
                }
            } 
        }
        out
    }
}

// Using some pixel data (obtained probably through Chunk.pixels), get a tuple containing 3 vecs. One for each color channel.
pub fn pix_to_channels(pixel_data: Vec<Rgba<u8>>, ) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut out_r: Vec<f64> = Vec::with_capacity(0);
    let mut out_g: Vec<f64> = Vec::with_capacity(0);
    let mut out_b: Vec<f64> = Vec::with_capacity(0);

    for Rgba {0: [r, g, b, _a]} in pixel_data {
        out_r.push((r as f64) / 255.0);
        out_g.push((g as f64) / 255.0);
        out_b.push((b as f64) / 255.0);
    }

    (out_r, out_g, out_b)
}