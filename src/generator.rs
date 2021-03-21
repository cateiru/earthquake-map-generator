use crate::error::Error;

pub struct EAGenerator {
  pub image_width: usize,
  pub image_height: usize,
  pub zoom_level: usize,
}

impl EAGenerator {
  pub fn new(image_width: usize, image_height: usize, zoom_level: usize) -> Result<Self, Error> {
    Ok(Self {
      image_width: image_width,
      image_height: image_height,
      zoom_level: zoom_level,
    })
  }

  // pub fn generate(point_data) {}
}
