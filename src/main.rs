#[macro_use] extern crate rocket;
extern crate diesel;

use rocket::{http::Status, response::Redirect, serde::json::Json};

use data::structs::*;
use data::db::*;

mod data;


#[launch]
fn rocket() -> _ {
  let mut connection = connect();
  let new_human = NewHuman {
      first_name: String::from("John"),
      last_name: String::from("Doe"),
      age: 30,
      username: String::from("johndoe"),
      email: String::from("john.doe@example.com"),
      location: String::from("New York"),
  };
  diesel::insert_into(structs::::table).values(&new_human).execute(&mut connection).expect("Error saving new human");


  rocket::build().mount("/", routes![new, get_link])
}

#[post("/new", data = "<data>")]
fn new(data: Json<Data>) -> Status {
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