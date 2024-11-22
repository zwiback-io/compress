#[macro_use] extern crate rocket;

use constants::HOST;
use rocket::{http::Status, response::Redirect, serde::json::Json, State};
use data::{storage::Storage, structs::*};

mod data;
mod constants;

/*
--- CONCEPT ---

POST /new -> URL
- create shortened form
- create db entry
- delete time? 

GET /<id> -> REDIRECT
- lookup db
- entry found
  - redirect

*/

#[launch]
fn rocket() -> _ {
  let storage = Storage::init();
  println!("STORAGE ID: {}", storage.id);

  rocket::build()
  .manage(storage)
  .mount("/", routes![new, get_thing, del_thing])
}

#[post("/new", data = "<data>")]
fn new(
  strg: &State<Storage>,
  data: Json<Thing>
) -> Result<String, Status> {
  let res = strg.set(data.into_inner());

  match res {
      Ok(r) => Ok(format!("{host}/{id}",host = HOST ,id = r.to_string())),
      Err(e) => Err(e)
  }
}


#[get("/<id>")]
fn get_thing(
  strg: &State<Storage>,
  id: &str
) -> Result<Redirect, Status> {
  let res = strg.get(id.to_string());
  
  match res {
    Some(r) => Ok(Redirect::to(r.link)),
    None => Err(Status::ImATeapot),
  }
}


#[delete("/<id>")]
fn del_thing(
  strg: &State<Storage>,
  id: &str
) -> Status {
  strg.del(id.to_string())
}