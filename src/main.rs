mod capture;
mod device;
mod imageprocessing;

#[cfg(unix)]
use {device::apa102::Apa102, linux_embedded_hal::spidev};

#[cfg(feature = "rpi")]
use crate::capture::dispmanx;

use crate::device::Device;
use crate::imageprocessing::{
    blackborder::{bounding_box, Bounds},
    LedMap,
};
use image::{ImageBuffer, RgbImage};

#[cfg(unix)]
fn main() {
    let mut spi_options = spidev::SpidevOptions::new();
    spi_options.max_speed_hz(4_000_000);
    let mut apa = Apa102::init("/dev/spidev0.0", spi_options).unwrap();

    let mut map = LedMap::init(26, 14, 46, Some(14), Some(46), true);
    let mut bounds = Bounds {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let mut counter = 0u32;

    loop {
        let img = dispmanx::capture();

        if counter % 30 == 0 {
            if let Some(b) = bounding_box::bounding_box(&img) {
                bounds = b;
            }
        }

        let img = bounding_box::trim(img, &bounds);

        apa.write(&map.map(img));

        counter += 1;
    }

    let slice = vec![image::Rgb([0u8; 3]); 128];
    apa.write(&slice);
}

#[cfg(windows)]
fn main() {
    println!("Windows currently not ready.");
}
