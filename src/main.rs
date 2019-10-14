mod capture;
mod imageprocessing;
mod device;

#[cfg(target_os = "linux")]
use {
    linux_embedded_hal as linux_hal,
    smart_leds::SmartLedsWrite,
    smart_leds::colors,

    device::Apa102,
};

#[cfg(feature = "rpi")]
use crate::capture::dispmanx;

use crate::imageprocessing::{
    LedMap,
    blackborder::{Bounds, bounding_box},
};
use image::{ImageBuffer, RgbImage};
use crate::device::Device;

#[cfg(unix)]
fn main() {
    let mut spi_options = linux_hal::spidev::SpidevOptions::new();
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
        break;
        let img = dispmanx::capture();

        if counter % 30 == 0 {
            if let Some(b) = bounding_box::bounding_box(&img) {
                bounds = b;
            }
        }

        let img = bounding_box::trim(img, &bounds);

        //println!("{:?}", img);
        apa.write(map.map(img).iter().map(|rgb| rgb.0));

        counter += 1;
    }

    let color = colors::BLACK;
    let slice = vec![color; 128];
    apa.write(slice.into_iter());
}

#[cfg(windows)]
fn main() {
    println!("Windows currently not ready.");
}