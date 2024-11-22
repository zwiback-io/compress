use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Thing {
  pub slug: Option<String>, // not sure if needed
  pub link: String,
  pub expiry: Option<i64>
}

// "id"
pub struct Label { }