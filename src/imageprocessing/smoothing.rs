use image::Rgb;

pub fn linear_smoothing(from: &Vec<Rgb<u8>>, to: &Vec<Rgb<u8>>, factor: f32) -> Vec<Rgb<u8>> {
    if factor >= 1f32 {
        return to.clone();
    }

    from.iter()
        .zip(to)
        .map(|(from, to)| {
            let mut diff = [
                f32::from(to.0[0]) - f32::from(from.0[0]),
                f32::from(to.0[1]) - f32::from(from.0[1]),
                f32::from(to.0[2]) - f32::from(from.0[2])
            ];

            diff.iter_mut().for_each(|c| *c *= factor);

            Rgb([
                (f32::from(from.0[0]) + diff[0]) as u8,
                (f32::from(from.0[1]) + diff[1]) as u8,
                (f32::from(from.0[2]) + diff[2]) as u8,
            ])
        })
        .collect()
}
