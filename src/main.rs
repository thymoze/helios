//#![feature(test)]

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
    smoothing, LedMap,
};
use image::Rgb;
use std::{thread, time::Duration, time::Instant};

#[cfg(feature = "rpi")]
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

    let mut last_write = Instant::now();
    let mut leds = vec![Rgb([0; 3]); 128];

    loop {
        let img = dispmanx::capture();

        if counter % 500 == 0 {
            if let Some(b) = bounding_box::bounding_box(&img) {
                bounds = b;
            }
        }

        let img = bounding_box::trim(img, &bounds);
        let mapped = map.map_hash(&img);

        let factor = (Instant::now() - last_write).as_secs_f32() / 0.2;
        leds = smoothing::linear_smoothing(&leds, &mapped, factor);

        apa.write(&leds).unwrap();
        last_write = Instant::now();

        counter = counter.wrapping_add(1);

        thread::sleep(Duration::from_millis(33));
    }
}

#[cfg(not(feature = "rpi"))]
fn main() {}
