mod keystore;
mod files;

use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Result, Responder};
use actix_web_static_files;
use futures::{TryStreamExt};
use keystore::{InMemoryKeyValueStore, KeyStore};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use crate::files::save_file_to_disk;

struct AppState {
    value_store: InMemoryKeyValueStore<String, String>,
}

#[post("/files")]
async fn upload(
    state: web::Data<AppState>,
    mut payload: Multipart,
) -> Result<web::Json<HashMap<String, String>>> {
    let mut added_files: HashMap<String, String> = HashMap::new();

    let kv_store = &state.value_store;
    while let Ok(Some(mut field)) = payload.try_next().await {
        if let Err(_) = save_file_to_disk(kv_store, &mut added_files, &mut field).await {

        }
    }

    Ok(web::Json(added_files).into())
}

#[get("/files")]
async fn get_files(state: web::Data<AppState>) -> Result<web::Json<HashMap<String, String>>> {
    let store = &state.value_store;
    Ok(web::Json(store.get_all().await))
}

#[get("/files/{file_id}")]
async fn get_file(
    state: web::Data<AppState>,
    file_id: web::Path<String>,
) -> Result<HttpResponse> {
    let store = &state.value_store;
    if let Some(filepath) = store.get(file_id.to_string()).await {
        let path = Path::new(&filepath);
        match File::open(&path).await {
            Ok(mut file) => {
                let mut file_data = Vec::new();

                // Must be a better way of doing this without writing an entire file
                // to memory. Can we stream it from disk instad?
                if file.read_to_end(&mut file_data).await.is_ok() {
                    Ok(HttpResponse::Ok()
                        .content_type("application/octet-stream")
                        .append_header(("Content-Disposition", format!("attachment; filename={}", path.file_name().unwrap().to_string_lossy())))
                        .body(file_data))
                } else {
                    Ok(HttpResponse::InternalServerError().body("Failed to read the file"))
                }
            }
            Err(_) => Ok(HttpResponse::NotFound().body("File not found")),
        }
    } else {
        Ok(HttpResponse::NotFound().into())
    }
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let data = web::Data::new(AppState {
        value_store: InMemoryKeyValueStore::<String, String>::new(),
    });

    let bind_target = "127.0.0.1:8000";

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .app_data(data.clone())
            // .wrap(Cors::default().allow_any_origin().allow_any_method())
            .service(upload)
            .service(get_file)
            .service(get_files)
            .service(actix_web_static_files::ResourceFiles::new("/", generated).resolve_not_found_to_root())
    })
    .bind(bind_target)?
    .run()
    .await
}
