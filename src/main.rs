#![feature(proc_macro_hygiene, decl_macro)]

use std::fs::{self, File};

use nanoid::nanoid;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use rocket::{
    http::{hyper::StatusCode, ContentType},
    response::{Content, Stream},
    Data,
    get,
    post,
    routes
};

#[get("/")]
fn index() -> Result<Content<String>, StatusCode> {
    let content = get_index();

    if let Some(c) = content {
        return Ok(Content(ContentType::HTML, c));
    }

    Err(StatusCode::NotFound)
}

#[get("/file/<id>")]
fn get_file(id: String) -> Option<Stream<File>> {
    let file = File::open(format!("{}", id));

    if let Ok(f) = file {
        return Some(Stream::chunked(f, 10));
    }

    None
}

#[post("/upload", format = "image/png", data = "<data>")]
fn upload_png(data: Data) -> Result<Content<String>, std::io::Error> {
    let id = nanoid!();
    Ok(save_image_stream(data, id))
}

#[post("/upload", format = "image/jpeg", data = "<data>")]
fn upload_jpeg(data: Data) -> Result<Content<String>, std::io::Error> {
    let id = nanoid!();
    Ok(save_image_stream(data, id))
}

#[post("/upload", format = "plain", data = "<data>")]
fn upload_text(data: Data) -> Result<Content<String>, std::io::Error> {
    let id = nanoid!();
    Ok(save_image_stream(data, id))
}

fn save_image_stream(data: Data, id: String) -> Content<String> {
    let mut db = PickleDb::new(
        "data.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    );

    match data.stream_to_file(format!("{}", id)) {
        Ok(_) => {
            match db.set(&id, &id) {
                Ok(_) => {}
                Err(_) => return Content(ContentType::HTML, String::from("Failed to save image.")),
            };
            Content(ContentType::HTML, id)
        }
        Err(_) => Content(ContentType::HTML, String::from("Failed to save image.")),
    }
}

fn get_index() -> Option<String> {
    fs::read_to_string("index.html").ok()
}

fn main() {
    let routes = routes![index, get_file, upload_png, upload_jpeg, upload_text];

    rocket::ignite().mount("/", routes).launch();
}
