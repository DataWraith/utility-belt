use std::hash::Hash;
use std::path::Path;

use ahash::HashMap;
use hsv::hsv_to_rgb;
use image::{ImageBuffer, Rgb};

use super::Grid2D;

impl<T: Clone + Into<Rgb<u8>>> Grid2D<T> {
    /// Saves the grid as a PNG image.
    pub fn save_png(&self, path: &Path) -> Result<(), image::ImageError> {
        let mut image = image::ImageBuffer::new(self.width() as u32, self.height() as u32);

        for (coord, value) in self.iter() {
            image.put_pixel(coord.x() as u32, coord.y() as u32, value.clone().into());
        }

        image.save(path)?;

        Ok(())
    }
}

impl<T: Clone + Eq + Hash> Grid2D<T> {
    /// Saves the grid as a PNG image, where each unique value is assigned a random (but fixed) color.
    pub fn save_png_random(&self, path: &Path) -> Result<(), image::ImageError> {
        let mut image: ImageBuffer<Rgb<u8>, _> =
            image::ImageBuffer::new(self.width() as u32, self.height() as u32);

        let mut colors = HashMap::default();
        let golden_ratio_conjugate = 0.618033988749895;
        let mut h = 1.0 / 3.1415926535;

        for (coord, value) in self.iter() {
            let color: Rgb<u8> = *colors.entry(value.clone()).or_insert_with(|| {
                h += golden_ratio_conjugate;
                h %= 1.0;

                let (r, g, b) = hsv_to_rgb(h * 360.0, 0.5, 0.95);
                Rgb([r, g, b])
            });

            image.put_pixel(coord.x() as u32, coord.y() as u32, color);
        }

        image.save(path)?;

        Ok(())
    }
}
