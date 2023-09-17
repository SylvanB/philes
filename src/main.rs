mod keystore;
mod files;

use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Result};
use actix_web_static_files;
use futures::{TryStreamExt};
use keystore::{InMemoryKeyValueStore, KeyStore};
use std::collections::HashMap;
use std::env;
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
            return Err(HttpResponse::InternalServerError().into())
        }
    }

    Ok(web::Json(added_files))
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
) -> Result<HttpResponse, Error> {
    let store = &state.value_store;
    match store.get(file_id.0).await {
        Some(path) => Ok(HttpResponse::Ok().body(path)),
        None => Ok(HttpResponse::NotFound().into()),
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
            .wrap(Cors::default().allow_any_origin().allow_any_method())
            .service(upload)
            .service(get_file)
            .service(get_files)
            .service(
                actix_web_static_files::ResourceFiles::new("/", generated)
                    .resolve_not_found_to_root(),
            )
    })
    .bind(bind_target)?
    .run()
    .await
}
