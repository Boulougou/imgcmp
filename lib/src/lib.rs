pub fn compare_images(_left_image : &[u8], _right_image : &[u8]) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn identical_images_are_same() -> anyhow::Result<()> {
        let img1 = read_image("../assets/cat.jpg")?;
        let img2 = read_image("../assets/cat.jpg")?;

        assert_eq!(compare_images(&img1, &img2), true);
        Ok(())
    }

    #[test]
    fn different_images_are_not_same() -> anyhow::Result<()> {
        let img1 = read_image("../assets/cat.jpg")?;
        let img2 = read_image("../assets/cat2.jpg")?;

        assert_eq!(compare_images(&img1, &img2), false);
        Ok(())
    }

    fn read_image(path : &str) -> anyhow::Result<Vec<u8>> {
        let reader = image::io::Reader::open(path).
            with_context(|| format!("Failed to open image {}", path))?;
        let decoded_image = reader.decode().
            with_context(|| format!("Failed to decode image {}", path))?;

        Ok(decoded_image.into_bytes())
    }
}