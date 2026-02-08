#[repr(C)]
pub struct BitmapFileHeader {
    signature: [u8; 2],
    file_size: u32,
    reserved: u32,
}

#[repr(C)]
pub struct BitmapInfoHeader {
    size: u32,
    width: i32,
    height: u32,
    planes: u16,
    bit_per_pixel: u16,
    compression: u32,
    image_size: u32,
    x_ppm: i32,
    y_ppm: i32,
    colors_used: u32,
    colors_important: u32,
}

pub struct Bitmap {}
