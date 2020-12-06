use crate::image::*;
use std::f32::consts::PI;
use nalgebra::DMatrix;
use ndarray::Array2;

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

pub fn hash_coefficients(coefficients : &DMatrix<u8>) -> u64 {
    let (_, hash) = coefficients.fold((0 as u64, 0 as u64),
        |(index, hash), c| (index + 1, hash | ((c as u64) << index)));
    hash
}