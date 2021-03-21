use crate::error::Error;

pub struct Pointer {
  pub lat: f64,
  pub lon: f64,
  // if true, epicenter version.
  pub is_epicenter: bool,
  // if true, seismic_intensity is "0", "1", "2", "3", "4", "5_low", "5_up", "6_low", "6_up", "7"
  // if false, this None.
  pub seismic_intensity: Option<String>,
}

impl Pointer {
  pub fn new(
    lat: f64,
    lon: f64,
    is_epicenter: bool,
    seismic_intensity: Option<String>,
  ) -> Result<Self, Error> {
    if is_epicenter {
      match seismic_intensity {
        Some(i) => Ok(Self {
          lat: lat,
          lon: lon,
          is_epicenter: is_epicenter,
          seismic_intensity: Some(i),
        }),
        None => Err(Error::NotDefineError("seismic_intensity".to_string())),
      }
    } else {
      Ok(Self {
        lat: lat,
        lon: lon,
        is_epicenter: is_epicenter,
        seismic_intensity: seismic_intensity,
      })
    }
  }
}
