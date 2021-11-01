mod keystore;

use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Result};
use actix_web_static_files;
use futures::{StreamExt, TryStreamExt};
use keystore::{InMemoryKeyValueStore, KeyStore};
use nanoid::nanoid;
use std::collections::HashMap;
use std::io::Write;

struct AppState {
    value_store: InMemoryKeyValueStore<String, String>,
}

#[post("/files")]
async fn upload(
    state: web::Data<AppState>,
    mut payload: Multipart,
) -> Result<web::Json<HashMap<String, String>>> {
    let mut added_files: HashMap<String, String> = HashMap::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!(
            "/home/goldsoultheory/repos/philes-rs/philes-server/tmp/{}",
            &filename
        );
        let id = nanoid!();

        let mut f = web::block(move || std::fs::File::create(&filepath))
            .await
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }

        let store = &state.value_store;
        match store
            .upsert(id.clone(), format!("/tmp/philes.rs/{}", filename))
            .await
        {
            // TODO: Actually do error handling
            Ok(_) => added_files.insert(id, format!("/tmp/philes.rs/{}", &filename)),
            Err(_) => return Err(HttpResponse::InternalServerError().into()),
        };
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
    let data = web::Data::new(AppState {
        value_store: InMemoryKeyValueStore::<String, String>::new(),
    });

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .app_data(data.clone())
            .wrap(Cors::default().allow_any_origin().allow_any_method())
            .service(upload)
            .service(get_file)
            .service(get_files)
            .service(actix_web_static_files::ResourceFiles::new("/", generated))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
