use anyhow::{anyhow};

#[derive (Debug, PartialEq, Eq, Clone)]
pub struct Image {
    width : u32,
    height : u32,
    channels_per_pixel : u8,
    buffer : Vec<u8>
}

impl Image {
    pub fn new(raw_image : &[u8], width : u32, channels_per_pixel : u8) -> anyhow::Result<Image> {
        if raw_image.is_empty() || width == 0 || channels_per_pixel == 0 {
            return Err(anyhow!("Invalid parameters passed"));
        }
        let num_pixels = raw_image.len() as u32 / channels_per_pixel as u32;
        let height = (num_pixels / width) as u32;

        let adjusted_data_size = width * height * channels_per_pixel as u32;
        Ok(Image{width, height, channels_per_pixel, buffer : Vec::from(&raw_image[..adjusted_data_size as usize])})
    }

    pub fn from_rgb(raw_pixels : &[(u8, u8, u8)], width : u32) -> anyhow::Result<Image> {
        let height = raw_pixels.len() as u32 / width;

        let raw_data = raw_pixels.iter().fold(Vec::new(), |mut acc, (r, g, b)| {
            acc.push(*r);
            acc.push(*g);
            acc.push(*b);
            acc
        });

        let image = Image {width, height, channels_per_pixel : 3, buffer : raw_data};
        Ok(image)
    }

    pub fn from_rgba(raw_pixels : &[(u8, u8, u8, u8)], width : u32) -> anyhow::Result<Image> {
        let height = raw_pixels.len() as u32 / width;

        let raw_data = raw_pixels.iter().fold(Vec::new(), |mut acc, (r, g, b, a)| {
            acc.push(*r);
            acc.push(*g);
            acc.push(*b);
            acc.push(*a);
            acc
        });

        let image = Image {width, height, channels_per_pixel : 4, buffer : raw_data};
        Ok(image)
    }

    pub fn get_pixel(&self, x : u32, y : u32) -> Vec<u8> {
        let index = y * self.channels_per_pixel as u32 * self.width + x * self.channels_per_pixel as u32;
        Vec::from(&self.buffer[index as usize..index as usize + self.channels_per_pixel as usize])
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
}
