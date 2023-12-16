use std::path::Path;

use image::Rgb;

use super::Grid2D;

impl<T: Clone + Into<Rgb<u8>>> Grid2D<T> {
    /// Saves the grid as a PNG image.
    pub fn save_png(&self, path: &Path) -> Result<(), image::ImageError> {
        let mut image = image::ImageBuffer::new(self.width() as u32, self.height() as u32);

        for (coord, value) in self.indexed_iter() {
            image.put_pixel(coord.x() as u32, coord.y() as u32, value.clone().into());
        }

        image.save(path)?;

        Ok(())
    }
}
