use std::fs::File;
use std::io::{Write, BufWriter};
use crate::color::Color;
const BMP_HEADER_SIZE: usize = 40;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 24;

pub fn write_bmp_file(
    file_path: &str,
    buffer: &Vec<Color>,
    width: usize,
    height: usize,
) -> std::io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    write_bmp_header(&mut writer, width, height)?;
    write_pixel_data(&mut writer, buffer, width, height)?;
    Ok(())
}

pub fn write_bmp_header(
    file: &mut BufWriter<File>,
    width: usize,
    height: usize,
) -> std::io::Result<()> {
    // Constants:
    let file_size = 14 + 40 + width * height * 3;
    //Constructing Header:

    let mut bmp_header = Vec::new();
    bmp_header.extend(b"BM");     
    bmp_header.extend(&(file_size as u32).to_le_bytes());
    bmp_header.extend(&(0 as u32).to_le_bytes()); // Reserved
    bmp_header.extend(&(BMP_PIXEL_OFFSET as u32).to_le_bytes());
    file.write_all(&bmp_header)?;

    // DIB Header (BITMAPINFOHEADER)
    let mut dib_header = Vec::new();
    dib_header.extend(&(BMP_HEADER_SIZE as u32).to_le_bytes()); // Header size
    dib_header.extend(&(width as u32).to_le_bytes()); // Image width
    dib_header.extend(&(height as u32).to_le_bytes()); // Image height
    dib_header.extend((1 as u16).to_le_bytes()); // Planes
    dib_header.extend(&(BMP_BITS_PER_PIXEL as u16).to_le_bytes()); // Bits per pixel
    dib_header.extend((0 as u32).to_le_bytes()); // Compression
    dib_header.extend((0 as u32).to_le_bytes()); // Image size
    dib_header.extend((0 as u32).to_le_bytes()); // X pixels per meter
    dib_header.extend((0 as u32).to_le_bytes()); // Y pixels per meter
    dib_header.extend((0 as u32).to_le_bytes()); // Total colors
    dib_header.extend((0 as u32).to_le_bytes()); // Important colors
 
    file.write_all(&dib_header)?;
    Ok(())
}


pub fn write_pixel_data(
    file: &mut BufWriter<File>,
    buffer: &Vec<Color>,
    width: usize,
    height: usize,
) -> std::io::Result<()> {
    let row_padding = (4 - (width * 3 % 4)) % 4;
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = buffer[(y * width + x) as usize];
            let r = (pixel.r & 0xFF) as u8;
            let g = (pixel.g & 0xFF) as u8;
            let b = (pixel.b & 0xFF) as u8;

            file.write_all(&[b, g, r])?;
        }
        // Padding for 4-byte alignment
        for _ in 0..row_padding {
            file.write_all(&[0])?;
        }
    }
    Ok(())
}