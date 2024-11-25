use std::sync::PoisonError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Doesn't exist: {0}")]
  IoNotFound(std::io::Error),

  #[error("Lacking permissions: {0}")]
  IoNoPermission(std::io::Error),

  #[error("Io error: {0:?}")]
  IoOther(std::io::Error),

  #[error("Error with lock: {0}")]
  Poison(String),

  #[error("Error with Arc: {0}")]
  Arc(String),

  #[error("Generic error: {0}")]
  Other(&'static str),
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
     match err.kind() {
       std::io::ErrorKind::NotFound => Error::IoNotFound(err),
       std::io::ErrorKind::PermissionDenied => Error::IoNoPermission(err),
       _ => Error::IoOther(err),
     }
  }
}

impl<T> From<PoisonError<T>> for Error {
  fn from(err: PoisonError<T>) -> Error {
    Error::Poison(err.to_string())
  }
}
