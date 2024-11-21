use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Data {
  id: String, // not sure if needed
  pub link: String,
  pub expiry: Option<i64>,
  pub slug: String
}

// Specify the fully-qualified path for the table_name attribute#[derive(Insertable)]
#[diesel(table_name = schema::human)]
pub struct NewHuman {
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub username: String,
    pub email: String,
    pub location: String,
}