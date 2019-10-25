use image::Rgb;
use rayon::prelude::*;

pub fn linear_smoothing(from: &Vec<Rgb<u8>>, to: &Vec<Rgb<u8>>, factor: f32) -> Vec<Rgb<u8>> {
    if factor >= 1f32 {
        return to.clone();
    }

    from.par_iter()
        .zip(to)
        .map(|(from, to)| {
            let diff = [
                (f32::from(to.0[0]) - f32::from(from.0[0])) * factor,
                (f32::from(to.0[1]) - f32::from(from.0[1])) * factor,
                (f32::from(to.0[2]) - f32::from(from.0[2])) * factor,
            ];

            Rgb([
                (f32::from(from.0[0]) + diff[0]) as u8,
                (f32::from(from.0[1]) + diff[1]) as u8,
                (f32::from(from.0[2]) + diff[2]) as u8,
            ])
        })
        .collect()
}
