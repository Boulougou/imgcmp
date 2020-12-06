use crate::image::*;
use anyhow::{anyhow};

pub fn scale_image(image : &Image, new_width : u32, new_height : u32) -> anyhow::Result<Image> {
    if new_width == 0 || new_height == 0 {
        return Err(anyhow!("Passed dimensions should not be zero"));
    }

    if new_width == image.get_width() && new_height == image.get_height() {
        return Ok(image.clone());
    }

    let scale_x = new_width as f32 / image.get_width() as f32;
    let scale_y = new_height as f32 / image.get_height() as f32;

    let mut scaled_data = Vec::new();
    for new_y in 0..new_height {
        for new_x in 0..new_width {
            let pixel = sample_pixels(image, new_x, new_y, scale_x, scale_y);
            for channel in pixel {
                scaled_data.push(channel as u8);
            }
        }
    }

    let scaled_image = Image::from(&scaled_data, new_width, image.get_channels_per_pixel())?;
    Ok(scaled_image)
}

fn sample_pixels(image: &Image, new_x: u32, new_y: u32, scale_x: f32, scale_y: f32) -> Vec<u32> {
    let left = (new_x as f32 / scale_x).floor() as u32;
    let right = ((new_x + 1) as f32 / scale_x).ceil() as u32;
    let top = (new_y as f32 / scale_y).floor() as u32;
    let bottom = ((new_y + 1) as f32 / scale_y).ceil() as u32;

    let mut original_pixels = Vec::new();
    for x in left..right {
        for y in top..bottom {
            let original_pixel = image.get_pixel(x, y);
            original_pixels.push(original_pixel);
        }
    }

    average_pixels(&original_pixels)
}

fn average_pixels(pixels: &[&Vec<u8>]) -> Vec<u32> {
    let channels_per_pixel = pixels[0].len();

    let mut average_pixel = Vec::new();
    for _i in 0..channels_per_pixel {
        average_pixel.push(0 as u32);
    }

    for pixel in pixels {
        for i in 0..pixel.len() {
            average_pixel[i] += pixel[i] as u32;
        }
    }

    for i in 0..channels_per_pixel {
        average_pixel[i as usize] = (average_pixel[i as usize] as f32 / pixels.len() as f32).floor() as u32;
    }
    average_pixel
}

pub fn into_grayscale(mut image : Image) -> Image {
    image.apply(|pixel| {
        let sum : u32 = pixel.iter().map(|x| *x as u32).sum();
        let average = (sum as f32 / pixel.len() as f32).floor() as u8;

        pixel.clear();
        pixel.push(average);
    });

    image
}

#[cfg(test)]
mod tests {
    mod scale_image {
        use crate::image_processing::scale_image;
        use crate::Image;

        #[test]
        fn return_original_image_when_already_in_passed_dimensions() -> anyhow::Result<()> {
            let color1 = (100, 200, 50);
            let color2 = (20, 150, 80);
            let color3 = (255, 10, 0);
            let raw_data = vec!(
                color1, color2, color3, color1,
                color1, color3, color2, color1,
                color1, color2, color1, color1,
                color2, color1, color1, color1);
            let source_image = Image::from_rgb(&raw_data, 4)?;

            let scaled_image = scale_image(&source_image, 4, 4)?;

            assert_eq!(source_image, scaled_image);
            Ok(())
        }

        #[test]
        fn reduce_both_dimensions() -> anyhow::Result<()> {
            let color1 = (100, 200, 50);
            let color2 = (20, 150, 80);
            let color3 = (255, 10, 0);
            let raw_data = vec!(
                color1, color2, color3, color3,
                color1, color3, color3, color1,
                color1, color2, color1, color1,
                color2, color1, color1, color1);
            let source_image = Image::from_rgb(&raw_data, 4)?;

            let scaled_image = scale_image(&source_image, 2, 2)?;

            assert_eq!(scaled_image.get_width(), 2);
            assert_eq!(scaled_image.get_height(), 2);
            assert_eq!(*scaled_image.get_pixel(0, 0), vec!(118, 140, 45));
            assert_eq!(*scaled_image.get_pixel(1, 0), vec!(216, 57, 12));
            assert_eq!(*scaled_image.get_pixel(0, 1), vec!(60, 175, 65));
            assert_eq!(*scaled_image.get_pixel(1, 1), vec!(100, 200, 50));
            Ok(())
        }

        #[test]
        fn reduce_width_only() -> anyhow::Result<()> {
            let color1 = (100, 200, 50);
            let color2 = (20, 150, 80);
            let color3 = (255, 10, 0);
            let raw_data = vec!(
                color1, color2, color3, color3,
                color1, color3, color3, color1,
                color1, color2, color1, color1,
                color2, color1, color1, color1);
            let source_image = Image::from_rgb(&raw_data, 4)?;

            let scaled_image = scale_image(&source_image, 2, 4)?;

            let expected_image = Image::from_rgb(&vec!(
                (60, 175, 65), (255, 10, 0),
                (177, 105, 25), (177, 105, 25),
                (60, 175, 65), (100, 200, 50),
                (60, 175, 65), (100, 200, 50)), 2)?;
            assert_eq!(scaled_image, expected_image);
            Ok(())
        }

        #[test]
        fn reduce_height_only() -> anyhow::Result<()> {
            let color1 = (100, 200, 50);
            let color2 = (20, 150, 80);
            let color3 = (255, 10, 0);
            let raw_data = vec!(
                color1, color2, color3, color3,
                color1, color3, color3, color1,
                color1, color2, color1, color1,
                color2, color1, color1, color1);
            let source_image = Image::from_rgb(&raw_data, 4)?;

            let scaled_image = scale_image(&source_image, 4, 2)?;

            assert_eq!(scaled_image.get_width(), 4);
            assert_eq!(scaled_image.get_height(), 2);
            assert_eq!(*scaled_image.get_pixel(0, 0), vec!(100, 200, 50));
            assert_eq!(*scaled_image.get_pixel(1, 0), vec!(137, 80, 40));
            assert_eq!(*scaled_image.get_pixel(2, 0), vec!(255, 10, 0));
            assert_eq!(*scaled_image.get_pixel(3, 0), vec!(177, 105, 25));
            assert_eq!(*scaled_image.get_pixel(0, 1), vec!(60, 175, 65));
            assert_eq!(*scaled_image.get_pixel(1, 1), vec!(60, 175, 65));
            assert_eq!(*scaled_image.get_pixel(2, 1), vec!(100, 200, 50));
            assert_eq!(*scaled_image.get_pixel(3, 1), vec!(100, 200, 50));
            Ok(())
        }

        #[test]
        fn increase_both_dimensions() -> anyhow::Result<()> {
            let color1 = (100, 200, 50, 200);
            let color2 = (20, 150, 80, 255);
            let color3 = (255, 10, 0, 0);
            let color4 = (80, 80, 80, 100);
            let raw_data = vec!(
                color1, color2,
                color3, color4);
            let source_image = Image::from_rgba(&raw_data, 2)?;

            let scaled_image = scale_image(&source_image, 4, 4)?;

            assert_eq!(scaled_image.get_width(), 4);
            assert_eq!(scaled_image.get_height(), 4);
            assert_eq!(*scaled_image.get_pixel(0, 0), vec!(100, 200, 50, 200));
            assert_eq!(*scaled_image.get_pixel(1, 0), vec!(100, 200, 50, 200));
            assert_eq!(*scaled_image.get_pixel(2, 0), vec!(20, 150, 80, 255));
            assert_eq!(*scaled_image.get_pixel(3, 0), vec!(20, 150, 80, 255));
            assert_eq!(*scaled_image.get_pixel(0, 1), vec!(100, 200, 50, 200));
            assert_eq!(*scaled_image.get_pixel(1, 1), vec!(100, 200, 50, 200));
            assert_eq!(*scaled_image.get_pixel(2, 1), vec!(20, 150, 80, 255));
            assert_eq!(*scaled_image.get_pixel(3, 1), vec!(20, 150, 80, 255));
            assert_eq!(*scaled_image.get_pixel(0, 2), vec!(255, 10, 0, 0));
            assert_eq!(*scaled_image.get_pixel(1, 2), vec!(255, 10, 0, 0));
            assert_eq!(*scaled_image.get_pixel(2, 2), vec!(80, 80, 80, 100));
            assert_eq!(*scaled_image.get_pixel(3, 2), vec!(80, 80, 80, 100));
            assert_eq!(*scaled_image.get_pixel(0, 3), vec!(255, 10, 0, 0));
            assert_eq!(*scaled_image.get_pixel(1, 3), vec!(255, 10, 0, 0));
            assert_eq!(*scaled_image.get_pixel(2, 3), vec!(80, 80, 80, 100));
            assert_eq!(*scaled_image.get_pixel(3, 3), vec!(80, 80, 80, 100));
            Ok(())
        }

        #[test]
        fn reduce_to_not_exactly_divisible_dimensions() -> anyhow::Result<()> {
            let color1 = (100, 200, 50);
            let color2 = (20, 150, 80);
            let color3 = (255, 10, 0);
            let raw_data = vec!(
                color1, color2, color3, color3,
                color1, color3, color3, color1,
                color1, color2, color1, color1,
                color2, color1, color1, color1);
            let source_image = Image::from_rgb(&raw_data, 4)?;

            let scaled_image = scale_image(&source_image, 3, 3)?;

            assert_eq!(scaled_image.get_width(), 3);
            assert_eq!(scaled_image.get_height(), 3);
            assert_eq!(*scaled_image.get_pixel(0, 0), vec!(118, 140, 45));
            assert_eq!(*scaled_image.get_pixel(1, 0), vec!(196, 45, 20));
            assert_eq!(*scaled_image.get_pixel(2, 0), vec!(216, 57, 12));
            assert_eq!(*scaled_image.get_pixel(0, 1), vec!(118, 140, 45));
            assert_eq!(*scaled_image.get_pixel(1, 1), vec!(157, 92, 32));
            assert_eq!(*scaled_image.get_pixel(2, 1), vec!(138, 152, 37));
            assert_eq!(*scaled_image.get_pixel(0, 2), vec!(60, 175, 65));
            assert_eq!(*scaled_image.get_pixel(1, 2), vec!(80, 187, 57));
            assert_eq!(*scaled_image.get_pixel(2, 2), vec!(100, 200, 50));
            Ok(())
        }

        #[test]
        fn return_error_when_passed_dimensions_are_zero() -> anyhow::Result<()> {
            let color1 = (100, 200, 50);
            let raw_data = vec!(color1, color1);
            let source_image = Image::from_rgb(&raw_data, 1)?;

            let result = scale_image(&source_image, 0, 1);
            assert!(result.is_err());
            let result = scale_image(&source_image, 1, 0);
            assert!(result.is_err());
            let result = scale_image(&source_image, 0, 0);
            assert!(result.is_err());
            Ok(())
        }
    }

    mod into_grayscale {
        use crate::image_processing::into_grayscale;
        use crate::Image;

        #[test]
        fn return_average_of_all_channels() -> anyhow::Result<()> {
            let color1 = (100, 200, 50);
            let color2 = (20, 150, 80);
            let color3 = (255, 10, 0);
            let raw_data = vec!(
                color1, color2, color3,
                color1, color3, color2,
                color1, color2, color1);
            let source_image = Image::from_rgb(&raw_data, 3)?;

            let scaled_image = into_grayscale(source_image);

            assert_eq!(scaled_image.get_width(), 3);
            assert_eq!(scaled_image.get_height(), 3);
            assert_eq!(*scaled_image.get_pixel(0, 0), vec!(116));
            assert_eq!(*scaled_image.get_pixel(1, 0), vec!(83));
            assert_eq!(*scaled_image.get_pixel(2, 0), vec!(88));
            assert_eq!(*scaled_image.get_pixel(0, 1), vec!(116));
            assert_eq!(*scaled_image.get_pixel(1, 1), vec!(88));
            assert_eq!(*scaled_image.get_pixel(2, 1), vec!(83));
            assert_eq!(*scaled_image.get_pixel(0, 2), vec!(116));
            assert_eq!(*scaled_image.get_pixel(1, 2), vec!(83));
            assert_eq!(*scaled_image.get_pixel(2, 2), vec!(116));
            Ok(())
        }
    }
}