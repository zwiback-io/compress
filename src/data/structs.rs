use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
  id: String, // not sure if needed
  pub link: String,
  pub expiry: Option<i64>,
  pub slug: String
}