use dh::recommended::*;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::fs::{remove_file, OpenOptions};

#[derive(Debug)]
pub enum Types {
    Rgb,
    Rgba,
    Luma,
    LumaA,
    Unsupported,
}

pub fn b2i(path: &str, alpha: bool, grayscale: bool) {
    let mut reader = dh::file::open_r(path).unwrap();
    let size = reader.size().unwrap() as u32;

    if grayscale {
        if alpha {
            let img_size = size as f64 / 2.0 + 2.0;

            let size_sqrt = img_size.sqrt();
            let width = size_sqrt.ceil() as u32;
            let height = width;

            let mut imgbuf = ImageBuffer::new(width, height);

            let size_bytes = size.to_le_bytes();
            let pixel = imgbuf.get_pixel_mut(0, 0);
            *pixel = image::LumaA([size_bytes[0], size_bytes[1]]);
            let pixel = imgbuf.get_pixel_mut(1, 0);
            *pixel = image::LumaA([size_bytes[2], size_bytes[3]]);

            for x in 2..width {
                let w = reader.read_u8().unwrap_or(0);
                let a = reader.read_u8().unwrap_or(0);
                let pixel = imgbuf.get_pixel_mut(x, 0);
                *pixel = image::LumaA([w, a]);
            }

            for y in 1..height {
                for x in 0..width {
                    let w = reader.read_u8().unwrap_or(0);
                    let a = reader.read_u8().unwrap_or(0);
                    let pixel = imgbuf.get_pixel_mut(x, y);
                    *pixel = image::LumaA([w, a]);
                }
            }

            imgbuf.save(path.to_string() + ".png").unwrap();
        } else {
            let img_size = size as f64 + 4.0;

            let size_sqrt = img_size.sqrt();
            let width = size_sqrt.ceil() as u32;
            let height = width;

            let mut imgbuf = ImageBuffer::new(width, height);

            let size_bytes = size.to_le_bytes();
            let pixel = imgbuf.get_pixel_mut(0, 0);
            *pixel = image::Luma([size_bytes[0]]);
            let pixel = imgbuf.get_pixel_mut(1, 0);
            *pixel = image::Luma([size_bytes[1]]);
            let pixel = imgbuf.get_pixel_mut(2, 0);
            *pixel = image::Luma([size_bytes[2]]);
            let pixel = imgbuf.get_pixel_mut(3, 0);
            *pixel = image::Luma([size_bytes[3]]);

            for x in 4..width {
                let w = reader.read_u8().unwrap_or(0);
                let pixel = imgbuf.get_pixel_mut(x, 0);
                *pixel = image::Luma([w]);
            }

            for y in 1..height {
                for x in 0..width {
                    let w = reader.read_u8().unwrap_or(0);
                    let pixel = imgbuf.get_pixel_mut(x, y);
                    *pixel = image::Luma([w]);
                }
            }

            imgbuf.save(path.to_string() + ".png").unwrap();
        }
    } else if alpha {
        let img_size = size as f64 / 4.0 + 1.0;

        let size_sqrt = img_size.sqrt();
        let width = size_sqrt.ceil() as u32;
        let height = width;

        let mut imgbuf = ImageBuffer::new(width, height);

        let pixel = imgbuf.get_pixel_mut(0, 0);
        *pixel = image::Rgba(size.to_le_bytes());

        for x in 1..width {
            let r = reader.read_u8().unwrap_or(0);
            let g = reader.read_u8().unwrap_or(0);
            let b = reader.read_u8().unwrap_or(0);
            let a = reader.read_u8().unwrap_or(0);
            let pixel = imgbuf.get_pixel_mut(x, 0);
            *pixel = image::Rgba([r, g, b, a]);
        }

        for y in 1..height {
            for x in 0..width {
                let r = reader.read_u8().unwrap_or(0);
                let g = reader.read_u8().unwrap_or(0);
                let b = reader.read_u8().unwrap_or(0);
                let a = reader.read_u8().unwrap_or(0);
                let pixel = imgbuf.get_pixel_mut(x, y);
                *pixel = image::Rgba([r, g, b, a]);
            }
        }

        imgbuf.save(path.to_string() + ".png").unwrap();
    } else {
        let img_size = size as f64 / 3.0 + 2.0;

        let size_sqrt = img_size.sqrt();
        let width = size_sqrt.ceil() as u32;
        let height = width;

        let mut imgbuf = ImageBuffer::new(width, height);

        let size_bytes = size.to_le_bytes();
        let pixel = imgbuf.get_pixel_mut(0, 0);
        *pixel = image::Rgb([size_bytes[0], size_bytes[1], size_bytes[2]]);
        let pixel = imgbuf.get_pixel_mut(1, 0);
        *pixel = image::Rgb([size_bytes[3], 0, 0]);

        for x in 2..width {
            let r = reader.read_u8().unwrap_or(0);
            let g = reader.read_u8().unwrap_or(0);
            let b = reader.read_u8().unwrap_or(0);
            let pixel = imgbuf.get_pixel_mut(x, 0);
            *pixel = image::Rgb([r, g, b]);
        }

        for y in 1..height {
            for x in 0..width {
                let r = reader.read_u8().unwrap_or(0);
                let g = reader.read_u8().unwrap_or(0);
                let b = reader.read_u8().unwrap_or(0);
                let pixel = imgbuf.get_pixel_mut(x, y);
                *pixel = image::Rgb([r, g, b]);
            }
        }

        imgbuf.save(path.to_string() + ".png").unwrap();
    }
}

pub fn i2b(path: &str) {
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();
    remove_file(path.to_string() + ".bin").unwrap_or_default();
    let mut writer = dh::file::open_w(path.to_string() + ".bin").unwrap();

    let format = match img {
        DynamicImage::ImageLuma8(_) => Types::Luma,
        DynamicImage::ImageLumaA8(_) => Types::LumaA,
        DynamicImage::ImageRgb8(_) => Types::Rgb,
        DynamicImage::ImageRgba8(_) => Types::Rgba,
        _ => Types::Unsupported,
    };
    let size = match format {
        Types::Luma => {
            let px = img.get_pixel(0, 0).0;
            let px2 = img.get_pixel(1, 0).0;
            let px3 = img.get_pixel(2, 0).0;
            let px4 = img.get_pixel(3, 0).0;
            u32::from_le_bytes([px[0], px2[0], px3[0], px4[0]])
        }
        Types::LumaA => {
            let px = img.get_pixel(0, 0).0;
            let px2 = img.get_pixel(1, 0).0;
            u32::from_le_bytes([px[0], px[3], px2[0], px2[3]])
        }
        Types::Rgb => {
            let px = img.get_pixel(0, 0).0;
            u32::from_le_bytes([px[0], px[1], px[2], img.get_pixel(1, 0).0[0]])
        }
        Types::Rgba => u32::from_le_bytes(img.get_pixel(0, 0).0),
        _ => 0,
    };

    for x in (match format {
        Types::Luma => 4,
        Types::LumaA => 2,
        Types::Rgb => 2,
        Types::Rgba => 1,
        _ => 0,
    })..width
    {
        let pixel = img.get_pixel(x, 0);
        match format {
            Types::Luma => {
                writer.write_u8(pixel[0]).unwrap();
            }
            Types::LumaA => {
                writer.write_u8(pixel[0]).unwrap();
                writer.write_u8(pixel[3]).unwrap();
            }
            Types::Rgb => {
                writer.write_u8(pixel[0]).unwrap();
                writer.write_u8(pixel[1]).unwrap();
                writer.write_u8(pixel[2]).unwrap();
            }
            Types::Rgba => {
                writer.write_u8(pixel[0]).unwrap();
                writer.write_u8(pixel[1]).unwrap();
                writer.write_u8(pixel[2]).unwrap();
                writer.write_u8(pixel[3]).unwrap();
            }
            _ => {}
        }
    }

    for y in 1..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            match format {
                Types::Luma => {
                    writer.write_u8(pixel[0]).unwrap();
                }
                Types::LumaA => {
                    writer.write_u8(pixel[0]).unwrap();
                    writer.write_u8(pixel[3]).unwrap();
                }
                Types::Rgb => {
                    writer.write_u8(pixel[0]).unwrap();
                    writer.write_u8(pixel[1]).unwrap();
                    writer.write_u8(pixel[2]).unwrap();
                }
                Types::Rgba => {
                    writer.write_u8(pixel[0]).unwrap();
                    writer.write_u8(pixel[1]).unwrap();
                    writer.write_u8(pixel[2]).unwrap();
                    writer.write_u8(pixel[3]).unwrap();
                }
                _ => {}
            }
        }
    }

    writer.close().unwrap();

    let writer = OpenOptions::new()
        .write(true)
        .open(path.to_string() + ".bin")
        .unwrap();
    writer.set_len(size as u64).unwrap();
}

pub fn i2d(path: &str) {
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();
    let format = match img {
        DynamicImage::ImageLuma8(_) => Types::Luma,
        DynamicImage::ImageLumaA8(_) => Types::LumaA,
        DynamicImage::ImageRgb8(_) => Types::Rgb,
        DynamicImage::ImageRgba8(_) => Types::Rgba,
        _ => Types::Unsupported,
    };
    let size = match format {
        Types::Luma => {
            let px = img.get_pixel(0, 0).0;
            let px2 = img.get_pixel(1, 0).0;
            let px3 = img.get_pixel(2, 0).0;
            let px4 = img.get_pixel(3, 0).0;
            u32::from_le_bytes([px[0], px2[0], px3[0], px4[0]])
        }
        Types::LumaA => {
            let px = img.get_pixel(0, 0).0;
            let px2 = img.get_pixel(1, 0).0;
            u32::from_le_bytes([px[0], px[1], px2[0], px2[1]])
        }
        Types::Rgb => {
            let px = img.get_pixel(0, 0).0;
            u32::from_le_bytes([px[0], px[1], px[2], img.get_pixel(1, 0).0[0]])
        }
        Types::Rgba => u32::from_le_bytes(img.get_pixel(0, 0).0),
        _ => 0,
    };

    println!("Binary size: {}", size);
    println!("Image size: {}x{}", width, height);
    println!(
        "Additional bytes at the end: {}",
        (width
            * height
            * match format {
                Types::Luma => 1,
                Types::LumaA => 2,
                Types::Rgb => 3,
                Types::Rgba => 4,
                _ => 0,
            })
            - size,
    );
    println!(
        "Type: {}",
        match format {
            Types::Rgb => {
                "RGB"
            }
            Types::Rgba => {
                "RGBA"
            }
            Types::Luma => {
                "Luma"
            }
            Types::LumaA => {
                "LumaA"
            }
            _ => {
                "Unknown"
            }
        }
    );
}
