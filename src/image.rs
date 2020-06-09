use crate::Color;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

impl Color {
    /// Returns a byte array of the data
    pub fn get_bytes(&self) -> Vec<u8> {
        vec![
            (self.red.sqrt().min(1.).max(0.) * 255.999) as u8,
            (self.green.sqrt().min(1.).max(0.) * 255.999) as u8,
            (self.blue.sqrt().min(1.).max(0.) * 255.999) as u8,
        ]
    }
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Vec<Color>>,
}

impl Image {
    /// Creates a new `width`x`height` white image.
    pub fn new(width: u32, height: u32) -> Self {
        let data = vec![vec![Color::default(); width as usize]; height as usize];
        Image {
            width,
            height,
            data,
        }
    }

    /// Creates a new test image with colors relative to their offset
    pub fn new_test(width: u32, height: u32) -> Self {
        let data = (0..height)
            .map(|cur_height| {
                let h_float = cur_height as f64;
                (0..width)
                    .map(|cur_width| {
                        let w_float = cur_width as f64;
                        let r_float = w_float / (width - 1) as f64;
                        let g_float = (height as f64 - 1_f64 - h_float) / (height - 1) as f64;
                        let b_float = 0.25;
                        Color {
                            red: r_float,
                            green: g_float,
                            blue: b_float,
                        }
                    })
                    .collect()
            })
            .collect();
        Image {
            width,
            height,
            data,
        }
    }

    /// Writes the image to a file in ppm format
    pub fn write_ppm<P: AsRef<Path>>(self, path: P) {
        // Create file
        let file = File::create(path).expect("Error creating file");
        let mut w = BufWriter::new(file);

        // Write header
        writeln!(w, "P3").unwrap();
        writeln!(w, "{} {}", self.width, self.height).unwrap();
        writeln!(w, "255").unwrap();

        // Write data
        self.data.iter().for_each(|line| {
            line.iter().for_each(|color| {
                writeln!(
                    w,
                    "{} {} {}",
                    color.red as u8, color.green as u8, color.blue as u8
                )
                .unwrap();
            })
        });
    }

    /// Writes the image to a file in png format
    pub fn write_png<P: AsRef<Path>>(self, path: P) {
        // Convert to png data
        let data: Vec<u8> = self
            .data
            .iter()
            .flat_map(|line| line.iter().flat_map(|color| color.get_bytes()))
            .collect();

        // Write data
        // if path.as_ref().exists() {
        //     std::fs::remove_file(&path).unwrap();
        // }
        image::save_buffer_with_format(
            path,
            &data,
            self.width,
            self.height,
            image::ColorType::Rgb8,
            image::ImageFormat::Png,
        )
        .unwrap()
    }
}
