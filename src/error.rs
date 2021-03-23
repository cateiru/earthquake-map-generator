use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("variable `{0}` is not defined.")]
  NotDefineError(String),
  #[error("Skia error: {0}")]
  SkiaError(String),
}
