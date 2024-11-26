use std::net::IpAddr;
use rocket::request::{FromRequest, Outcome, Request};


#[derive(Debug)]
pub enum APIError { }

pub struct RateLimit {
  pub ip: IpAddr
}



#[rocket::async_trait]
impl<'r> FromRequest<'r> for RateLimit {
    type Error = APIError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
      Outcome::Success(RateLimit {
        ip: request.client_ip().expect("No IP? Ghost?")
      })
    }
}