mod image;
mod utils;

use crate::image::Image;
use anyhow::Context;

pub struct Config {
    dct_dimension : u32
}

impl Default for Config {
    fn default() -> Self {
        Config { dct_dimension : 32 }
    }
}

pub fn compare_images(left_image : &Image, right_image : &Image, config : Config) -> anyhow::Result<bool> {
    // Scale both images down to DCT size
    let left_scaled = utils::scale_image(left_image, config.dct_dimension, config.dct_dimension).
        context("Failed to scale left image")?;
    let right_scaled = utils::scale_image(right_image, config.dct_dimension, config.dct_dimension).
        context("Failed to scale left image")?;

    // convert to grayscale
    let left_scaled_grayscale = utils::into_grayscale(left_scaled);
    let right_scaled_grayscale = utils::into_grayscale(right_scaled);

    // compute 32x32 DCT
    // take top left 8x8 from DCT
    // compute average and convert to 1bit 8x8
    // create hash

    Ok(left_scaled_grayscale == right_scaled_grayscale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use ::image::GenericImageView;
    use ::image::DynamicImage;
    use ::image::imageops::FilterType;

    #[test]
    fn identical_images_are_same() -> anyhow::Result<()> {
        let img1 = read_image("../assets/cat.jpg").and_then(|x| to_image(x))?;
        let img2 = read_image("../assets/cat.jpg").and_then(|x| to_image(x))?;

        assert_eq!(compare_images(&img1, &img2, Config::default())?, true);
        Ok(())
    }

    #[test]
    fn different_images_are_not_same() -> anyhow::Result<()> {
        let img1 = read_image("../assets/cat.jpg").and_then(|x| to_image(x))?;
        let img2 = read_image("../assets/cat2.jpg").and_then(|x| to_image(x))?;

        assert_eq!(compare_images(&img1, &img2, Config::default())?, false);
        Ok(())
    }

    #[test]
    fn grayscale_image_is_same_with_original() -> anyhow::Result<()> {
        let img = read_image("../assets/cat.jpg")?;
        let grayscale_img = img.grayscale();

        assert_eq!(compare_images(&to_image(img)?,
                                  &to_image(grayscale_img)?, Config::default())?, true);
        Ok(())
    }

    #[test]
    fn blurred_image_is_same_with_original() -> anyhow::Result<()> {
        let img = read_image("../assets/cat.jpg")?;
        let blurred_img = img.blur(3.0);

        assert_eq!(compare_images(&to_image(img)?,
                                  &to_image(blurred_img)?, Config::default())?, true);
        Ok(())
    }

    #[test]
    fn image_with_different_aspect_ratio_is_same_with_original() -> anyhow::Result<()> {
        let img = read_image("../assets/cat.jpg")?;
        let distorted_img = img.resize(img.width() / 4, img.height() / 2, FilterType::Gaussian);

        assert_eq!(compare_images(&to_image(img)?,
                                  &to_image(distorted_img)?, Config::default())?, true);
        Ok(())
    }

    fn read_image(path : &str) -> anyhow::Result<DynamicImage> {
        let reader = ::image::io::Reader::open(path).
            with_context(|| format!("Failed to open image {}", path))?;
        let decoded_image = reader.decode().
            with_context(|| format!("Failed to decode image {}", path))?;

        Ok(decoded_image)
    }

    fn to_image(decoded_image : DynamicImage) -> anyhow::Result<Image> {
        let width = decoded_image.width();
        let channel_count = decoded_image.color().channel_count();
        Image::from(&decoded_image.into_bytes(),width, channel_count)
    }
}