mod error;
mod movie;
mod server;
mod moviestore {
  include!("moviestore.rs");

  pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("movie_descriptor");
}

use crate::movie::MovieStoreImpl;
use error::Error;
use moviestore::moviestore_server::MoviestoreServer;
use tonic::transport::Server;
use tonic_reflection::server::{ServerReflection, ServerReflectionServer};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let address = "[::1]:50051".parse()?;
  let moviestore: MovieStoreImpl = MovieStoreImpl::default();

  let reflection_service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(moviestore::FILE_DESCRIPTOR_SET)
    .build()?;

  start(address, moviestore, reflection_service).await
}

async fn start(
  address: std::net::SocketAddr,
  movie_store: MovieStoreImpl,
  reflection_service: ServerReflectionServer<impl ServerReflection>,
) -> Result<(), Error> {
  println!("Start server listening on {address}");
  Ok(
    Server::builder()
      .add_service(MoviestoreServer::new(movie_store))
      .add_service(reflection_service)
      .serve(address)
      .await?,
  )
}
