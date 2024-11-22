use std::{sync::Mutex, time::{SystemTime, UNIX_EPOCH}};

use rand::RngCore;
use rocket::http::Status;

use crate::{constants::*, Label, Thing};



pub struct Storage {
  pub id: String,
  things: Mutex<Vec<Thing>>,
}

impl Storage {
  pub fn init() -> Self {
    Storage {
    id: Label::new(None),
    things: Mutex::new(Vec::new()),
    }
  }

  pub fn get(&self, slug: String) -> Option<Thing> {
    let res = self.things.lock().unwrap().iter().find(|e| e.slug == Some(slug.clone())).cloned();

    res
  }

  pub fn set(&self, mut thing: Thing) -> Result<String, Status> {
    thing.slug = Some(Label::new(Some(6)));

    let elmnt = self.things.lock().unwrap().iter().find(|e| e.slug == thing.slug /* || e.link == thing.link */).cloned();
    println!("{:?}", elmnt);
    if elmnt.is_some() {
    return Err(Status::ImATeapot);
  }

    self.things.lock().unwrap().push(thing.clone());

    Ok(thing.slug.expect("Unga bunga"))
  }

  pub fn del(&self, slug: String) -> Status {
    self.things.lock().unwrap().retain(|c| c.slug != Some(slug.clone()));

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
