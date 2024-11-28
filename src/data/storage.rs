use std::{net::IpAddr, sync::Mutex, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};

use rand::RngCore;
use rocket::http::Status;

use crate::{constants::*, Label, Record, Thing};



pub struct Storage {
  pub id: String,
  things: Mutex<Vec<Record>>,
}


impl Storage {
  pub fn init() -> Self {
    Storage {
    id: Label::new(None),
    things: Mutex::new(Vec::new()),
    }
  }

  pub fn get(&self, id: String) -> Option<Record> {
    let res = self.things.lock().unwrap().iter().find(|e| e.data.slug == Some(id.clone())).cloned();

    res
  }

  pub fn set(&self, mut thing: Thing, owner: IpAddr) -> Result<String, Status> {
    let slug = Some(Label::new(Some(6)));
    thing.slug = slug.clone();

    let record = Record {
      owner,
      data: thing.clone(),
      last_write: Instant::now()
    };

    // todo: filter for owner & check time

    let elmnt = self.things.lock().unwrap().iter().find(|e| e.data.slug == slug /* || e.link == thing.link */).cloned();

    if elmnt.is_some() {
      return Ok(elmnt.unwrap().data.link);
    }

    self.things.lock().unwrap().push(record);

    Ok(thing.slug.expect("Unga bunga"))
  }

  pub fn del(&self, slug: String) -> Status {
    self.things.lock().unwrap().retain(|c| c.data.slug != Some(slug.clone()));

    Status::Ok
  }
}



impl Label {
  pub fn new(len: Option<usize>) -> String {
    let mut str =  format!("{:x}", Self::rand());
    str.truncate(len.unwrap_or(DEFAULT_LEN));
    str
  }

  fn rand() -> u128 {
    let start = SystemTime::now();
    let stamp = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_millis();

    let mut rand = rand::thread_rng();
    let n = rand.next_u64() as u128;

    (stamp + n) * SEED
  }
}
