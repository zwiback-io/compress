#[macro_use] extern crate rocket;

use rocket::{http::Status, response::Redirect, serde::json::Json};

use data::structs::*;

mod data;


#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![new_shortened, get_link])
}

#[post("/new", data = "<data>")]
fn new_shortened(data: Json<Data>) -> Status {
  Status::ImATeapot
}


#[get("/<id>")]
fn get_link(id: &str) -> Redirect {
  
  
  
  todo!()
}

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