pub mod blackborder;

use image::{self, imageops};
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
            reverse: reverse,
            map: HashMap::with_capacity((2 * w + 2 * h) as usize),
        }
    }

    pub fn map(&mut self, img: image::RgbImage) -> Vec<image::Rgb<u8>> {
        let img = imageops::resize(&img, self.width, self.height, image::FilterType::Nearest);

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
            } else {
                if i >= self.skip_offset && i < (self.skip_offset + self.skip) {
                    self.map.insert(i, None);
                    continue;
                }
            }

            let mut x;
            let mut y;
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

        //println!("{:?}", res);

        res
    }
}
