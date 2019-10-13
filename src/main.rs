mod capture;
mod blackborder;
mod imageprocessing;

use linux_embedded_hal as linux_hal;

use smart_leds::SmartLedsWrite;
use smart_leds::colors;
use apa102_spi::Apa102;

use crate::capture::dispmanx;
use crate::imageprocessing::LedMap;
use crate::blackborder::{Bounds, bounding_box};
use image::{ImageBuffer, RgbImage};

fn main() {
    let mut spi = linux_hal::Spidev::open("/dev/spidev0.0").unwrap();
    let mut spi_options = linux_hal::spidev::SpidevOptions::new();
    spi_options.max_speed_hz(4_000_000);
    spi.configure(&spi_options);

    let mut apa = Apa102::new(spi);
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
