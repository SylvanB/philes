use nanoid::nanoid;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use rocket::{Data, get, http::ContentType, post, response::Stream};
use rocket_contrib::json::Json;
use rocket_multipart_form_data::MultipartFormDataOptions;
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataField};
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, fs::File, io::Write, path::Path};

use crate::db::{Db};

#[post("/file/upload", data = "<data>")]
pub(crate) fn multipart_upload(content_type: &ContentType, data: Data) -> Result<String, &str> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("file").size_limit(32 * 1042 * 1024),
    ]);

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options) {
        Ok(form_data) => form_data,
        Err(err) => match err {
            rocket_multipart_form_data::MultipartFormDataError::DataTooLargeError(_) => {
                return Err("File is too large")
            }
            rocket_multipart_form_data::MultipartFormDataError::DataTypeError(_) => {
                return Err("Incorrect content type");
            }
            _ => panic!("{:?}", err),
        },
    };

    let file = multipart_form_data.raw.remove("file");

    match file {
        Some(mut file) => {
            let raw = file.remove(0);
            let file_name = raw.file_name.unwrap();
            let extension = Path::new(&file_name)
                .extension()
                .and_then(OsStr::to_str)
                .unwrap();

            let id = nanoid!();
            let file_name = String::from(format!("{}.{}", id, extension));
            let data = raw.raw;

            save_file(data, &file_name, &id).or(Err("Failed to save file"))
        }
        None => Err("Please input a file"),
    }
}

#[get("/file/<id>")]
pub(crate) fn get_file(id: String) -> Option<Stream<File>> {
    let file = File::open(format!("{}", id));

    if let Ok(f) = file {
        return Some(Stream::chunked(f, 10));
    }

    None
}

#[get("/files")]
pub(crate) fn get_all_files() -> Json<Vec<FileInfo>> {
    let results = Db::new().get_all();
    Json(results)
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FileInfo {
    pub id: String,
    pub location: String,
}

fn save_file(data: Vec<u8>, file_name: &String, id: &String) -> Result<String, &'static str> {
    let mut db = PickleDb::load(
        "data.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ).or_else(|_| return Err("Failed to open DB"))?;

    let mut file = File::create(file_name).or_else(|_| return Err("Failed to create file"))?;

    match file.write_all(&data) {
        Ok(_) => {
            match db.set(&id, &file_name) {
                Ok(_) => {}
                Err(_) => return Err("Failed to store image id in database"),
            };
        }
        Err(_) => return Err("Failed to write data to disk"),
    };

    Ok(String::from(format!("localhost:8000/file/{}", file_name)))
}
