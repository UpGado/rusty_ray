pub type Pixel = (u8, u8, u8);
pub type Img = Vec<Vec<Pixel>>; // row-major order

pub enum FileFormat {
    PPM,
}

pub fn save_img(img: &Img, format: FileFormat) -> () {
    match format {
        PPM => {
            let height = img.len();
            let width = img[0].len();
            let max_val = 255;
            println!("P3\n{} {}\n{}", width, height, max_val);
            for row in img {
                for pixel in row {
                    println!("{} {} {}", pixel.0, pixel.1, pixel.2);
                }
            }
        }
    }
}
