/// Represents an image in memory
/// 
/// This struct stores the raw pixel data of an image.

pub struct Image {
    pub width: u32,
    pub height: u32,
    /// Vector of bytes representing the raw pixel data.
    pub data: Vec<u8>
}