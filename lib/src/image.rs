use anyhow::{anyhow};

type Pixel = Vec<u8>;

/// Struct keeping data of a multi-channel image
#[derive (Debug, PartialEq, Eq, Clone)]
pub struct Image {
    width : u32,
    height : u32,
    channels_per_pixel : u8,
    pixels : Vec<Pixel>
}

impl Image {
    /// Create an image from a byte buffer.
    pub fn from(raw_image : &[u8], width : u32, channels_per_pixel : u8) -> anyhow::Result<Image> {
        if raw_image.is_empty() || width == 0 || channels_per_pixel == 0 {
            return Err(anyhow!("Invalid parameters passed"));
        }
        let num_pixels = raw_image.len() as u32 / channels_per_pixel as u32;
        let height = (num_pixels / width) as u32;

        let mut pixels = Vec::new();
        for i in 0..num_pixels as usize {
            let index = i * channels_per_pixel as usize;
            let pixel = Pixel::from(&raw_image[index..index + channels_per_pixel as usize]);
            pixels.push(pixel);
        }

        Ok(Image{width, height, channels_per_pixel, pixels})
    }

    /// Create an image from an array of RGB tuples.
    pub fn from_rgb(raw_pixels : &[(u8, u8, u8)], width : u32) -> anyhow::Result<Image> {
        let height = raw_pixels.len() as u32 / width;

        let pixels = raw_pixels.iter().fold(Vec::new(), |mut acc, (r, g, b)| {
            acc.push(vec!(*r, *g, *b));
            acc
        });

        let image = Image {width, height, channels_per_pixel : 3, pixels};
        Ok(image)
    }

    /// Create an image from an array of RGBA tuples.
    pub fn from_rgba(raw_pixels : &[(u8, u8, u8, u8)], width : u32) -> anyhow::Result<Image> {
        let height = raw_pixels.len() as u32 / width;

        let pixels = raw_pixels.iter().fold(Vec::new(), |mut acc, (r, g, b, a)| {
            acc.push(vec!(*r, *g, *b, *a));
            acc
        });

        let image = Image {width, height, channels_per_pixel : 4, pixels};
        Ok(image)
    }

    pub fn get_pixel(&self, x : u32, y : u32) -> &Pixel {
        let index = y * self.width + x;
        &self.pixels[index as usize]
    }

    fn access_pixel(&mut self, x : u32, y : u32) -> &mut Pixel {
        let index = y * self.width + x;
        &mut self.pixels[index as usize]
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_channels_per_pixel(&self) -> u8 {
        self.channels_per_pixel
    }

    /// Apply a function on each pixel of the image. Pixels are passed as mutable references
    /// so that mutations can happen in place.
    pub fn apply<F>(&mut self, mut f : F) where F: FnMut(&mut Pixel) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.access_pixel(x, y);
                f(pixel);
            }
        }
    }
}
