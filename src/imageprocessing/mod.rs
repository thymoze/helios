pub mod blackborder;
pub mod smoothing;

use image::{self, imageops, Rgb};
use std::collections::HashMap;

pub struct LedMap {
    width: u32,
    height: u32,
    size: u32,
    start_offset: u32,
    skip: u32,
    skip_offset: u32,
    reverse: bool,

    map: HashMap<u32, Option<(u32, u32)>>,
}

impl LedMap {
    pub fn init(
        w: u32,
        h: u32,
        start: u32,
        skip: Option<u32>,
        skip_offset: Option<u32>,
        reverse: bool,
    ) -> LedMap {
        LedMap {
            width: w,
            height: h,
            size: 2 * w + 2 * h,
            start_offset: start,
            skip: skip.unwrap_or(0),
            skip_offset: skip_offset.unwrap_or(0),
            reverse,
            map: HashMap::with_capacity((2 * w + 2 * h) as usize),
        }
    }

    pub fn map_rows(&mut self, img: &image::RgbImage) -> Vec<image::Rgb<u8>> {
        //let img = imageops::resize(img, self.width, self.height, image::FilterType::Gaussian);

        let mut left = Vec::with_capacity((img.height() - 2) as usize);

        let mut border = img
            .enumerate_rows()
            .flat_map(|(y, row)| {
                if y == 0 {
                    row.map(|(xx, yy, &pixel)| pixel).collect()
                } else if y == img.height() - 1 {
                    let mut vec: Vec<Rgb<u8>> = row.map(|(_, _, &pixel)| pixel).collect();
                    vec.reverse();
                    return vec;
                } else {
                    let mut vec = row.map(|(_, _, &pixel)| pixel);
                    left.push(vec.next().unwrap());
                    return vec![vec.last().unwrap()];
                }
            })
            .collect::<Vec<Rgb<u8>>>();
        left.reverse();
        border.append(&mut left);

        let mut res = border
            .iter()
            .enumerate()
            .filter(|&(i, _)| {
                !(self.skip_offset..self.skip_offset + self.skip).contains(&(i as u32))
            })
            .map(|(_, &pixel)| pixel)
            .collect::<Vec<Rgb<u8>>>();

        res.rotate_left(self.start_offset as usize);

        if self.reverse {
            res.reverse();
        }

        res
    }

    pub fn map_pixels(&mut self, img: &image::RgbImage) -> Vec<image::Rgb<u8>> {
        //let img = imageops::resize(img, self.width, self.height, image::FilterType::Gaussian);

        let mut left = Vec::with_capacity((img.height() - 2) as usize);
        let mut bottom = Vec::with_capacity(img.width() as usize);

        let mut border = img
            .enumerate_pixels()
            .filter(|&(x, y, &pixel)| {
                let on_border = y == 0 || x == img.width() - 1;

                if !on_border && x == 0 {
                    left.push(pixel);
                } else if !on_border && y == img.height() - 1 {
                    bottom.push(pixel);
                }

                on_border
            })
            .map(|(_, _, &pixel)| pixel)
            .collect::<Vec<Rgb<u8>>>();
        bottom.reverse();
        left.reverse();
        border.append(&mut bottom);
        border.append(&mut left);

        let mut res = border
            .iter()
            .enumerate()
            .filter(|&(i, _)| {
                !(self.skip_offset..self.skip_offset + self.skip).contains(&(i as u32))
            })
            .map(|(_, &pixel)| pixel)
            .collect::<Vec<Rgb<u8>>>();

        res.rotate_left(self.start_offset as usize);

        if self.reverse {
            res.reverse();
        }

        res
    }

    pub fn map_hash(&mut self, img: &image::RgbImage) -> Vec<image::Rgb<u8>> {
        let img = imageops::resize(img, self.width, self.height, image::FilterType::Gaussian);

        let mut res = Vec::with_capacity(self.size as usize);

        for mut i in 0..self.size {
            i += self.start_offset;
            i %= self.size;

            if let Some(value) = self.map.get(&i) {
                if let Some(coords) = value {
                    res.push(img.get_pixel(coords.0, coords.1).clone());
                }
                continue;
            }

            if self.skip_offset + self.skip > self.size {
                if i >= self.skip_offset || i < (self.skip_offset + self.skip) % self.size {
                    self.map.insert(i, None);
                    continue;
                }
            } else if i >= self.skip_offset && i < (self.skip_offset + self.skip) {
                self.map.insert(i, None);
                continue;
            }

            let x;
            let y;
            if i < self.width {
                x = i;
                y = 0;
            } else if i < self.width + self.height {
                x = self.width - 1;
                y = i - self.width;
            } else if i < 2 * self.width + self.height {
                x = 2 * self.width + self.height - i - 1;
                y = self.height - 1;
            } else {
                x = 0;
                y = 2 * self.width + 2 * self.height - i - 1;
            }

            self.map.insert(i, Some((x, y)));
            res.push(img.get_pixel(x, y).clone());
        }

        if self.reverse {
            res.reverse();
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;

    #[test]
    fn test_led_map() {
        let mut map = LedMap::init(4, 3, 4, Some(3), Some(4), false);

        let buf = vec![
            vec![1; 3],
            vec![2; 3],
            vec![3; 3],
            vec![4; 3],
            vec![10; 3],
            vec![0; 3],
            vec![0; 3],
            vec![5; 3],
            vec![9; 3],
            vec![8; 3],
            vec![7; 3],
            vec![6; 3],
        ]
        .into_iter()
        .flatten()
        .collect();

        let image = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(4, 3, buf).unwrap();

        //println!("{:?}", image.pixels().collect::<Vec<_>>());

        let mapped = map.map_rows(&image);

        let target = [8, 9, 10, 1, 2, 3, 4]
            .iter()
            .map(|&x| Rgb([x; 3]))
            .collect::<Vec<Rgb<u8>>>();

        assert_eq!(mapped, target);
    }
}
