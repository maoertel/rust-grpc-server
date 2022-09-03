use crate::Error;
use crate::Moviestore;
use crate::MoviestoreServer;
use tonic::transport::Server;
use tonic_reflection::server::ServerReflection;
use tonic_reflection::server::ServerReflectionServer;

pub(crate) async fn start<T>(
  address: std::net::SocketAddr,
  movie_store_service: MoviestoreServer<T>,
  reflection_service: ServerReflectionServer<impl ServerReflection>,
) -> Result<(), Error>
where
  T: Moviestore,
{
  println!("Start server listening on {address}");
  Ok(
    Server::builder()
      .add_service(movie_store_service)
      .add_service(reflection_service)
      .serve(address)
      .await?,
  )
}
