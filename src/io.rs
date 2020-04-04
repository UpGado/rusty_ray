extern crate image;
use std::fs::File;
use std::io::Write;

pub type Pixel = (u8, u8, u8);
pub type Img = Vec<Vec<Pixel>>; // row-major order

pub enum FileFormat {
    PPM,
    PNG,
}

pub fn save_img(img: &Img, format: FileFormat, filename: &str) -> () {
    let height = img.len();
    let width = img[0].len();
    match format {
        FileFormat::PPM => {
            let filename = [filename, ".ppm"].concat();
            let mut file = File::create(filename).unwrap();
            let max_val = 255;
            file.write_all(format!("P3\n{} {}\n{}\n", width, height, max_val).as_bytes())
                .unwrap();
            for row in img {
                for pixel in row {
                    file.write_all(format!("{} {} {}\n", pixel.0, pixel.1, pixel.2).as_bytes())
                        .unwrap();
                }
            }
        }
        FileFormat::PNG => {
            let filename = [filename, ".png"].concat();
            let mut buffer: Vec<u8> = Vec::with_capacity(width * height);
            for row in img {
                for pixel in row {
                    buffer.push(pixel.0);
                    buffer.push(pixel.1);
                    buffer.push(pixel.2);
                }
            }
            image::save_buffer(
                filename,
                &buffer,
                width as u32,
                height as u32,
                image::ColorType::Rgb8,
            )
            .unwrap();
        }
    }
}
