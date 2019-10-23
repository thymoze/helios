use super::Device;
use image::Rgb;
use linux_embedded_hal::{spidev::SpidevOptions, Spidev};
use smart_leds::SmartLedsWrite;
use std::io::Error;
use std::path::Path;

pub struct Apa102(apa102_spi::Apa102<Spidev>);

impl Apa102 {
    pub fn init(path: impl AsRef<Path>, options: SpidevOptions) -> Result<Self, Error> {
        let mut spi = Spidev::open(path)?;
        spi.configure(&options)?;

        let apa = apa102_spi::Apa102::new(spi);

        Ok(Apa102(apa))
    }
}

impl Device for Apa102 {
    fn write(&mut self, led_values: &[Rgb<u8>]) -> Result<(), Error> {
        self.0.write(led_values.iter().map(|v| v.0))
    }
}
