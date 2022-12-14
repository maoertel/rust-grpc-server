use crate::moviestore::{moviestore_server::Moviestore, GetMovieRequest, GetMovieResponse};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct MovieStoreImpl;

#[tonic::async_trait]
impl Moviestore for MovieStoreImpl {
  async fn get_movie(&self, request: Request<GetMovieRequest>) -> Result<Response<GetMovieResponse>, Status> {
    println!("Request from {:?}", request.remote_addr());

    let response = GetMovieResponse {
      id: request.into_inner().id,
      director: "Peter Jackson".to_string(),
      title: "Lord of the Rings".to_string(),
      year: 2000,
    };
    Ok(Response::new(response))
  }
}
