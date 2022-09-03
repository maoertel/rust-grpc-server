use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
  TonicTransport(tonic::transport::Error),
  TonicReflection(tonic_reflection::server::Error),
  ParseAddress(std::net::AddrParseError),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::TonicTransport(e) => std::fmt::Display::fmt(e, f),
      Error::TonicReflection(e) => std::fmt::Display::fmt(e, f),
      Error::ParseAddress(e) => std::fmt::Display::fmt(e, f),
    }
  }
}

impl std::error::Error for Error {}

impl From<tonic::transport::Error> for Error {
  fn from(error: tonic::transport::Error) -> Self {
    Error::TonicTransport(error)
  }
}

impl From<std::net::AddrParseError> for Error {
  fn from(error: std::net::AddrParseError) -> Self {
    Error::ParseAddress(error)
  }
}

impl From<tonic_reflection::server::Error> for Error {
  fn from(error: tonic_reflection::server::Error) -> Self {
    Error::TonicReflection(error)
  }
}
