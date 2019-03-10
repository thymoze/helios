use embedded_hal as hal;
use linux_embedded_hal as linux_hal;

use smart_leds::SmartLedsWrite;
use smart_leds::colors;
use apa102_spi::Apa102;

fn main() {
    let mut spi = linux_hal::Spidev::open("/dev/spidev0.0").unwrap();
    let mut spi_options = linux_hal::spidev::SpidevOptions::new();
    spi_options.max_speed_hz(4_000_000);
    spi.configure(&spi_options);

    let mut apa = Apa102::new(spi);
    let color = colors::AZURE;
    let slice = vec![color; 64];
    apa.write(slice.into_iter());
}
