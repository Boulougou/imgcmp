use imgcmp_lib;
use std::env;
use anyhow::Context;
use image::GenericImageView;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let img1 = read_image(&args[1])?;
    let img2 = read_image(&args[2])?;

    let config = imgcmp_lib::Config {
        dct_dimension : 32,
        dct_reduced_dimension : 8,
        allowed_distance : 3
    };

    let are_same = imgcmp_lib::compare_images(&img1, &img2, config)?;
    if are_same {
        println!("Pictures are the same");
    }
    else {
        println!("Pictures are different");
    }
    return Ok(());
}

fn read_image(path : &str) -> anyhow::Result<imgcmp_lib::Image> {
    let reader = image::io::Reader::open(path).
        with_context(|| format!("Failed to open image {}", path))?;
    let decoded_image = reader.decode().
        with_context(|| format!("Failed to decode image {}", path))?;

    let width = decoded_image.width();
    let channel_count = decoded_image.color().channel_count();
    imgcmp_lib::Image::from(&decoded_image.into_bytes(),width, channel_count)
}
