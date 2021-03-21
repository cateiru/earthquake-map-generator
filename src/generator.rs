use crate::error::Error;
use rs_latlon::Location;

pub struct EAGenerator {
  pub image_width: usize,
  pub image_height: usize,
  pub zoom_level: f64,
  pub location: Location,
}

impl EAGenerator {
  pub fn new(
    image_width: usize,
    image_height: usize,
    zoom_level: f64,
    center_lat: f64,
    center_lon: f64,
  ) -> Result<Self, Box<dyn std::error::Error>> {
    let location = Location::new_center(
      center_lat,
      center_lon,
      image_width,
      image_height,
      zoom_level,
    )?;
    Ok(Self {
      image_width: image_width,
      image_height: image_height,
      zoom_level: zoom_level,
      location: location,
    })
  }

  // pub fn generate(point_data) {}
}
