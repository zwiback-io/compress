use std::{fs::read_to_string, io::Error};

use rocket::{response::content::RawHtml, tokio::fs::File};

use crate::constants::NOTFOUND_TEXT;

#[get("/favicon.ico")]
pub async fn favicon() -> File {
  File::open(&"./src/docs/favicon.ico").await.ok().unwrap()
}

#[get("/")]
pub fn docs_index() -> RawHtml<String> {
  let res = fetch_doc_page("index.html");

  match res {
    Ok(r) => RawHtml(r),
    Err(_) => RawHtml(NOTFOUND_TEXT.to_string())
  }

}

fn fetch_doc_page(path: &str) -> Result<String, Error> {
  let full_path = format!("./src/docs/{}", path);
  read_to_string(full_path)
}  