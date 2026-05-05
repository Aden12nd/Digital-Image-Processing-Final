use image::{DynamicImage, GenericImage, GenericImageView, SubImage, Rgba};

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

        for p_y in self.coordinate.1 .. self.coordinate.1+self.size {
            for p_x in self.coordinate.0 .. self.coordinate.0+self.size {
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