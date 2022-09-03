mod error;
mod movie;
mod server;
mod moviestore {
  include!("moviestore.rs");

  pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("movie_descriptor");
}

use crate::movie::MovieStoreImpl;
use error::Error;
use moviestore::moviestore_server::{Moviestore, MoviestoreServer};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let address = "[::1]:50051".parse()?;
  let movie_store_service = MoviestoreServer::new(MovieStoreImpl::default());
  let reflection_service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(moviestore::FILE_DESCRIPTOR_SET)
    .build()?;

  server::start(address, movie_store_service, reflection_service).await
}
