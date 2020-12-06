mod image;
mod image_processing;
mod dct;

pub use crate::image::Image;
use anyhow::Context;
use nalgebra::DMatrix;
use ndarray::Array2;

pub struct Config {
    dct_dimension : u32,
    dct_reduced_dimension : u32
}

impl Default for Config {
    fn default() -> Self {
        Config { dct_dimension : 32, dct_reduced_dimension : 8 }
    }
}

pub fn compare_images(left_image : &Image, right_image : &Image, config : Config) -> anyhow::Result<bool> {
    let dct_basis_signals = dct::calc_dct_basis(config.dct_dimension);
    let left_hash = hash_image(&left_image, &dct_basis_signals, config.dct_reduced_dimension).
        context("Failed to create hash for first image")?;
    let right_hash = hash_image(&right_image, &dct_basis_signals, config.dct_reduced_dimension).
        context("Failed to create hash for second image")?;

    Ok(left_hash == right_hash)
}

fn hash_image(image : &Image, dct_basis : &Array2<DMatrix<f32>>, dct_reduced_dimension : u32) -> anyhow::Result<u64> {
    // Scale down to DCT size
    let (dct_dimension, _) = dct_basis.dim();
    let shrank_image = image_processing::
        scale_image(image, dct_dimension as u32, dct_dimension as u32).
        context("Failed to scale image")?;

    // convert to grayscale
    let shrank_grayscale_image = image_processing::into_grayscale(shrank_image);

    // compute NxN DCT coefficients
    let dct_coefficients = dct::calc_dct_coefficients(&shrank_grayscale_image,
                                                      &dct_basis, dct_reduced_dimension);

    // create hash
    Ok(dct::hash_coefficients(&dct_coefficients))
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
    fn resized_image_is_same_with_original() -> anyhow::Result<()> {
        let img = read_image("../assets/cat.jpg")?;
        let distorted_img = img.resize_exact(img.width() / 4, img.height() / 2, FilterType::Gaussian);

        assert_eq!(compare_images(&to_image(img)?,
                                  &to_image(distorted_img)?, Config::default())?, true);
        Ok(())
    }

    #[test]
    fn resized_and_blurred_image_is_same_with_original() -> anyhow::Result<()> {
        let img = read_image("../assets/cat.jpg")?;
        let blurred_img = img.
            resize_exact(img.width() / 10, img.height() / 2, FilterType::Gaussian).
            blur(3.0);

        assert_eq!(compare_images(&to_image(img)?,
                                  &to_image(blurred_img)?, Config::default())?, true);
        Ok(())
    }

    #[test]
    fn blurred_copies_of_original_are_same() -> anyhow::Result<()> {
        let img = read_image("../assets/cat.jpg")?;
        let blurred_img1 = img.blur(3.0);
        let blurred_img2 = img.blur(0.5);

        assert_eq!(compare_images(&to_image(blurred_img1)?,
                                  &to_image(blurred_img2)?, Config::default())?, true);
        Ok(())
    }

    #[test]
    fn resized_copies_of_original_are_same() -> anyhow::Result<()> {
        let img = read_image("../assets/cat.jpg")?;
        let resized_img1 = img.resize_exact(img.width() / 4, img.height() / 2, FilterType::Lanczos3);
        let resized_img2 = img.resize_exact(img.width() / 2, img.height() / 4, FilterType::CatmullRom);

        assert_eq!(compare_images(&to_image(resized_img1)?,
                                  &to_image(resized_img2)?, Config::default())?, true);
        Ok(())
    }

    #[test]
    fn shrank_and_blurred_copies_of_original_are_same() -> anyhow::Result<()> {
        let img = read_image("../assets/cat.jpg")?;
        let blurred_img1 = img.
            resize_exact(img.width() / 5, img.height() / 3, FilterType::Gaussian).
            blur(3.0);
        let blurred_img2 = img.
            resize_exact(img.width() / 10, img.height() / 2, FilterType::Gaussian).
            blur(0.5);

        assert_eq!(compare_images(&to_image(blurred_img1)?,
                                  &to_image(blurred_img2)?, Config::default())?, true);
        Ok(())
    }

    #[test]
    fn different_shrank_and_blurred_images_are_not_same() -> anyhow::Result<()> {
        let img1 = read_image("../assets/cat.jpg").
            and_then(|x| Ok(x.resize_exact(32, 32, FilterType::Gaussian))).
            and_then(|x| Ok(x.blur(3.0))).
            and_then(|x| to_image(x))?;
        let img2 = read_image("../assets/cat2.jpg").
            and_then(|x| Ok(x.resize_exact(32, 32, FilterType::Gaussian))).
            and_then(|x| Ok(x.blur(3.0))).
            and_then(|x| to_image(x))?;

        assert_eq!(compare_images(&img1, &img2, Config::default())?, false);
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