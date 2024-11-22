use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Thing {
  pub slug: Option<String>,
  pub link: String,
  pub expiry: Option<i64> // not implemented yet
}

// "id"
pub struct Label { }