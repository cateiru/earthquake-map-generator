use crate::error::Error;
use crate::setting::Setting;
use rs_latlon::Location;
use tiny_skia::Pixmap;

pub struct EAGenerator {
  pub zoom_level: f64,
  pub location: Location,
  pub title: String,
  pub time_zone: isize,
  pix_map: Pixmap,
}

impl EAGenerator {
  pub fn new(setting: Setting) -> Result<Self, Box<dyn std::error::Error>> {
    let location = Location::new_center(
      setting.center_lat,
      setting.center_lon,
      setting.image_width,
      setting.image_height,
      setting.zoom_level,
    )?;
    let pix_map = Pixmap::new(setting.image_width as u32, setting.image_height as u32);

    match pix_map {
      Some(_pix_map) => Ok(Self {
        zoom_level: setting.zoom_level,
        location: location,
        title: setting.title,
        time_zone: setting.time_zone,
        pix_map: _pix_map,
      }),
      None => Err(Box::new(Error::SkiaError(
        "Failed to create pix map.".to_string(),
      ))),
    }
  }

  // pub fn generate(point_data) {}
}
