use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("")]
  Error(),
  #[error("variable `{0}` is not defined.")]
  NotDefineError(String),
}
