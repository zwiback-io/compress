use std::{net::IpAddr, time::Instant};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Thing {
  pub slug: Option<String>,
  pub link: String
}

#[derive(Clone, Debug)]
pub struct Record {
  pub owner: IpAddr,
  pub data: Thing,
  pub last_write: Instant // not implemented yet
}

// "id"
pub struct Label { }