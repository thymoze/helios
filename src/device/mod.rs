#[cfg(unix)]
use {
    linux_embedded_hal::Spidev,
    apa102_spi::Apa102 as apa102,
    linux_embedded_hal::spidev::SpidevOptions,
};

pub trait Device {
    fn write(&mut self, leds: &[u8]) -> Result<(), std::io::Error>;
}

#[cfg(unix)]
pub struct Apa102 {
    spi: Spidev,
    apa: apa102_spi::Apa102<Spidev>,
}

#[cfg(unix)]
impl Apa102 {
    pub fn init(path: impl AsRef<std::path::Path>, options: SpidevOptions) -> Result<Self, std::io::Error>
    {
        let mut spi = Spidev::open(path)?;
        spi.configure(&options);

        let apa = apa102_spi::Apa102::new(spi);

        Ok(Apa102 {
            spi,
            apa
        })
    }
}

#[cfg(unix)]
impl Device for Apa102 {
    fn write(&mut self, leds: &[u8]) -> Result<(), std::io::Error> {
        self.apa.write(leds)
    }
}