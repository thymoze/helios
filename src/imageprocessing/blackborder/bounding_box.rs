use crate::imageprocessing::blackborder::Bounds;
use image::*;
use itertools::Itertools;
use num_traits::NumCast;

pub fn trim(mut img: RgbImage, rect: &Bounds) -> RgbImage {
    imageops::crop(&mut img, rect.x, rect.y, rect.width, rect.height).to_image()
}

/// (x, y, w, h)
pub fn bounding_box<I>(img: &I) -> Option<Bounds>
    where
        I: GenericImageView,
        I::Pixel: std::marker::Sync,
{
    let mut bounds = Bounds {
        width: 0,
        height: 0,
        x: img.width(),
        y: img.height(),
    };

    let target0 = img.get_pixel(0, 0);
    let target1 = img.get_pixel(img.width() - 1, 0);
    let target2 = img.get_pixel(0, img.height() - 1);

    let iterator = (0..img.width())
        .cartesian_product(0..img.height())
        .map(|(x, y)| (x, y, img.get_pixel(x, y)))
        .collect::<Vec<_>>();

    let mut xx = img.width();
    let mut yy = img.height();
    let mut width = 0;
    let mut height = 0;

    rayon::scope(|s| {
        s.spawn(|_| {
            xx = iterator.iter().fold(bounds.x, |acc, &(x, y, p)| {
                if x < acc && !fuzzy_match(&p, &target0) {
                    x
                } else {
                    acc
                }
            });
        });

        s.spawn(|_| {
            width = iterator.iter().fold(bounds.width, |acc, &(x, y, p)| {
                if x > acc && !fuzzy_match(&p, &target1) {
                    x
                } else {
                    acc
                }
            });
        });

        s.spawn(|_| {
            yy = iterator.iter().fold(bounds.y, |acc, &(x, y, p)| {
                if y < acc && !fuzzy_match(&p, &target0) {
                    y
                } else {
                    acc
                }
            });
        });

        s.spawn(|_| {
            height = iterator.iter().fold(bounds.height, |acc, &(x, y, p)| {
                if y > acc && !fuzzy_match(&p, &target2) {
                    y
                } else {
                    acc
                }
            });
        });
    });

    bounds = Bounds {
        width,
        height,
        x: xx,
        y: yy,
    };

    //    for y in 0..img.height() {
    //        let mut bounding_box = bounds.clone();
    //        //println!("y loop {}", y);
    //
    //        for x in 0..img.width() {
    //            //println!("x loop {}", x);
    //
    //            let p = img.get_pixel(x, y);
    //
    //            if x < bounding_box.x && !fuzzy_match(&p, &target0) {
    //                bounding_box.x = x;
    //            }
    //            if x > bounding_box.width && !fuzzy_match(&p, &target1) {
    //                bounding_box.width = x;
    //            }
    //            if y < bounding_box.y && !fuzzy_match(&p, &target0) {
    //                bounding_box.y = y;
    //            }
    //            if y > bounding_box.height && !fuzzy_match(&p, &target2) {
    //                bounding_box.height = y;
    //            }
    //        }
    //
    //        //println!("bounds {:?} {:?}", bounds, bounding_box);
    //
    //        if bounding_box.x < bounds.x {
    //            bounds.x = bounding_box.x;
    //        }
    //        if bounding_box.y < bounds.y {
    //            bounds.y = bounding_box.y;
    //        }
    //        if bounding_box.width > bounds.width {
    //            bounds.width = bounding_box.width;
    //        }
    //        if bounding_box.height > bounds.height {
    //            bounds.height = bounding_box.height;
    //        }
    //    }

    if bounds.width == 0 && bounds.height == 0 {
        eprintln!("Geometry does not contain image.");
        return None;
    } else {
        bounds.width = bounds.width.wrapping_sub(bounds.x.wrapping_sub(1));
        bounds.height = bounds.height.wrapping_sub(bounds.y.wrapping_sub(1));
    }

    Some(bounds)
}

fn fuzzy_match<P: Pixel>(p1: &P, p2: &P) -> bool {
    let mut fuzz = 5f32;
    fuzz *= fuzz;
    fuzz *= 3f32;

    let p1 = p1.to_rgb();
    let p2 = p2.to_rgb();

    let scale = 1.0;
    let mut distance = 0.0;

    let mut pixel: f32 = NumCast::from(p1[0] - p2[0]).unwrap();
    distance += pixel * pixel * scale;
    if distance > fuzz {
        return false;
    }

    pixel = NumCast::from(p1[1] - p2[1]).unwrap();
    distance += pixel * pixel * scale;
    if distance > fuzz {
        return false;
    }

    pixel = NumCast::from(p1[2] - p2[2]).unwrap();
    distance += pixel * pixel * scale;
    if distance > fuzz {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::imageprocessing::blackborder::bounding_box::{bounding_box, trim};
    use image::Rgb;

    // 0 -- black
    // 1 -- gray
    // 2- white
    // 3 -- gray
    // 4 --black

    #[test]
    fn blackborder_trim() {
        let base = Rgb([0; 3]);
        let mut image = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::from_fn(200, 150, |x, y| {
            match (y / 30) as i32 {
                0 | 4 => {
                    let range = (10..21);
                    if range.contains(&x) && range.contains(&y) {
                        Rgb([10; 3])
                    } else {
                        Rgb([0; 3])
                    }
                }
                1 | 3 => Rgb([6; 3]),
                _ => Rgb([255; 3])
            }
        });

        image.save("test.png");

        let bounds = bounding_box(&image).unwrap();
        println!("{:?}", bounds);

        let trimmed = trim(image, &bounds);
        trimmed.save("trimmed.png");
        //assert_eq!(bounds, None);
        panic!();
    }
}