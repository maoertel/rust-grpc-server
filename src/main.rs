mod movie;

mod moviestore {
  include!("moviestore.rs");

  pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("greeter_descriptor");
}

use crate::movie::MovieStoreImpl;
use moviestore::moviestore_server::MoviestoreServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let address = "[::1]:50051".parse().unwrap();
  let moviestore = MovieStoreImpl::default();

  let reflection_service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(moviestore::FILE_DESCRIPTOR_SET)
    .build()
    .unwrap();

  println!("Moviestore server listening on {address}");

  Server::builder()
    .add_service(MoviestoreServer::new(moviestore))
    .add_service(reflection_service)
    .serve(address)
    .await?;

  Ok(())
}
