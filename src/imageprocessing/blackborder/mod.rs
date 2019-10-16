pub mod bounding_box;

#[derive(Debug, Clone)]
pub struct Bounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
