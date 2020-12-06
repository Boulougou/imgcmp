use crate::image::*;
use std::f32::consts::PI;
use nalgebra::DMatrix;
use ndarray::Array2;
use anyhow::{anyhow};

/// Calculates DCT basis matrix for all horizontal and vertical frequencies
pub fn calc_dct_basis(dim : u32) -> Array2<DMatrix<f32>> {
    let matrix_at = |(k, l)| {
        DMatrix::<f32>::from_fn(dim as usize, dim as usize, |m, n| calc_dct_basis_at(dim, k, l, m, n))
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

/// Calculates the DCT coefficients for the passed image.
pub fn calc_dct_coefficients(image : &Image, dct_basis : &Array2<DMatrix<f32>>) -> DMatrix<f32> {
    let c = |x| if x == 0 {1.0 / std::f32::consts::SQRT_2} else {1.0};

    let coefficients = DMatrix::<f32>::from_fn(image.get_width() as usize, image.get_height() as usize,
        |k, l| {
            let a = 0.25 * c(k) * c(l);
            let mut sum = 0.0;
            let dct_mat = dct_basis.get((k, l)).unwrap();
            for m in 0..image.get_width() {
                for n in 0..image.get_height() {
                    let color = image.get_pixel(m, n)[0] as f32;
                    sum += color * dct_mat[(m as usize, n as usize)];
                }
            }
            a * sum
        });

    coefficients
}

/// Takes the top left "corner" of the passed DCT coefficients, computes the average and
/// converts them to single bit, based on whether they are below or above the average.
pub fn reduce_dct_coefficients(coefficients : DMatrix<f32>, dct_reduced_dimension : u32) -> DMatrix<u8> {
    let mut reduced_coefficients = coefficients.resize(dct_reduced_dimension as usize,
                                                   dct_reduced_dimension as usize,
                                                   0.0);
    // Exclude first term which is significantly different than other terms
    reduced_coefficients[(0, 0)] = 0.0;
    let average_coefficient = reduced_coefficients.mean();
    reduced_coefficients.map(|c| if c < average_coefficient { 0 } else { 1 })
}

/// Convert passed Matrix to a 64 bitmap. Passed matrix should only contain 1s or 0s.
/// Matrices with more than 64 elements are not allowed.
pub fn hash_coefficients(coefficients : &DMatrix<u8>) -> anyhow::Result<u64> {
    if coefficients.len() > 64 {
        return Err(anyhow!("Matrices of more than 64 elements are not allowed"));
    }

    let (_, hash) = coefficients.fold((0 as u64, 0 as u64),
        |(index, hash), c| (index + 1, hash | ((c as u64) << index)));
    Ok(hash)
}

/// Computes the Hamming distance between the passed bitmaps
pub fn compare_hashes(hash1 : u64, hash2 : u64) -> u8 {
    let xor = hash1 ^ hash2;
    xor.count_ones() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_dct_example() -> anyhow::Result<()> {
        let image = Image::from(&[
            144, 139, 149, 155, 153, 155, 155, 155,
            151, 151, 151, 159, 156, 156, 156, 158,
            151, 156, 160, 162, 159, 151, 151, 151,
            158, 163, 161, 160, 160, 160, 160, 161,
            158, 160, 161, 162, 160, 155, 155, 156,
            161, 161, 161, 161, 160, 157, 157, 157,
            162, 162, 161, 160, 161, 157, 157, 157,
            162, 162, 161, 160, 163, 157, 158, 154], 8, 1)?;

        let coefficients = calc_dct_coefficients(&image, &calc_dct_basis(8));

        let expected_coefficients = DMatrix::from_column_slice(8, 8, &[
            1257.9, 2.3, -9.7, -4.1,  3.9,  0.6, -2.1,  0.7,
            -21.0, -15.3, -4.3, -2.7,  2.3,  3.5,  2.1, -3.1,
            -11.2, -7.6, -0.9,  4.1,  2.0,  3.4,  1.4,  0.9,
            -4.9, -5.8, 1.8,  1.1,  1.6,  2.7,  2.8, -0.7,
            0.1, -3.8, 0.5,  1.3, -1.4,  0.7,  1.0,  0.9,
            0.9, -1.6, 0.9, -0.3, -1.8, -0.3,  1.4,  0.8,
            -4.4, 2.7, -4.4, -1.5, -0.1,  1.1,  0.4,  1.9,
            -6.4, 3.8, -5.0, -2.6,  1.6,  0.6,  0.1,  1.5]);

        let difference = expected_coefficients - coefficients;
        let are_same = difference.fold(true, |are_same, c| are_same && c.abs() < 0.1);
        assert!(are_same);
        Ok(())
    }

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