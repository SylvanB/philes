#![feature(proc_macro_hygiene, decl_macro)]

mod uploads;

use rocket::{
    get,
    http::{hyper::StatusCode, ContentType},
    response::Content,
    routes,
};
use std::fs;

#[get("/")]
fn index() -> Result<Content<String>, StatusCode> {
    let content = get_index();

    if let Some(c) = content {
        return Ok(Content(ContentType::HTML, c));
    }

    Err(StatusCode::NotFound)
}

fn get_index() -> Option<String> {
    fs::read_to_string("index.html").ok()
}

fn main() {
    let routes = routes![index, uploads::get_file, uploads::multipart_upload];

    rocket::ignite().mount("/", routes).launch();
}
