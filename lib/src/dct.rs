use crate::image::*;
use std::f32::consts::PI;
use nalgebra::DMatrix;
use ndarray::Array2;
use anyhow::{anyhow};

pub fn calc_dct_basis(dim : u32) -> Array2<DMatrix<f32>> {
    let matrix_at = |(m, n)| {
        DMatrix::<f32>::from_fn(dim as usize, dim as usize, |k, l| calc_dct_basis_at(dim, k, l, m, n))
    };
    Array2::from_shape_fn((dim as usize, dim as usize), matrix_at)
}

fn calc_dct_basis_at(dim : u32, k : usize, l : usize, m : usize, n : usize) -> f32 {
    let two_pi = 2.0 * PI;
    let two_dim = 2.0 * dim as f32;
    let horiz_cos = f32::cos(two_pi * (l as f32 / two_dim) * (n as f32 + 0.5));
    let vert_cos = f32::cos(two_pi * (k as f32 / two_dim) * (m as f32 + 0.5));
    return horiz_cos * vert_cos;
}

pub fn calc_dct_coefficients(image : &Image, dct_basis : &Array2<DMatrix<f32>>, dct_reduced_dimensions : u32) -> DMatrix<u8> {
    let image_matrix = DMatrix::<f32>::from_fn(image.get_width() as usize, image.get_height() as usize,
                                                |k, l| image.get_pixel(k as u32, l as u32)[0] as f32);

    let coefficients = DMatrix::<f32>::from_fn(image.get_width() as usize, image.get_height() as usize,
                            |k, l| (&image_matrix * dct_basis.get((k, l)).unwrap()).sum());

    // take top left 8x8 from DCT
    // compute average and convert to 1bit 8x8
    let reduced_coefficients = coefficients.resize(dct_reduced_dimensions as usize,
                                                   dct_reduced_dimensions as usize,
                                                   0.0);
    let average_coefficient = reduced_coefficients.mean();
    reduced_coefficients.map(|c| if c < average_coefficient { 0 } else { 1 })
}

pub fn hash_coefficients(coefficients : &DMatrix<u8>) -> anyhow::Result<u64> {
    if coefficients.len() > 64 {
        return Err(anyhow!("Matrices of more than 64 elements are not allowed"));
    }

    let (_, hash) = coefficients.fold((0 as u64, 0 as u64),
        |(index, hash), c| (index + 1, hash | ((c as u64) << index)));
    Ok(hash)
}

pub fn compare_hashes(hash1 : u64, hash2 : u64) -> u8 {
    let xor = hash1 ^ hash2;
    xor.count_ones() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_hash_from_matrix() -> anyhow::Result<()> {
        let coefficients = DMatrix::from_row_slice(3, 3, &[
            0, 1, 0,
            1, 1, 1,
            1, 0, 0]);

        let hash = hash_coefficients(&coefficients)?;

        assert_eq!(hash, 0b010011110);
        Ok(())
    }

    #[test]
    fn do_not_calculate_hash_when_matrix_dimension_is_greater_than_allowed() -> anyhow::Result<()> {
        let coefficients = DMatrix::zeros(9, 9);

        let result = hash_coefficients(&coefficients);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn return_zero_when_comparing_equal_hashes() -> anyhow::Result<()> {
        let result = compare_hashes(0b1011100100, 0b1011100100);

        assert_eq!(result, 0);
        Ok(())
    }

    #[test]
    fn return_non_zero_when_comparing_different_hashes() -> anyhow::Result<()> {
        let result = compare_hashes(0b1101101100, 0b1011100100);

        assert_eq!(result, 3);
        Ok(())
    }
}