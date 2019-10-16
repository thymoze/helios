use image::Rgb;

#[cfg(unix)]
pub mod apa102;

pub trait Device {
    fn write(&mut self, led_values: &[Rgb<u8>]) -> Result<(), std::io::Error>;
}
