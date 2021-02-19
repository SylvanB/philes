#![feature(proc_macro_hygiene, decl_macro)]

mod db;
mod uploads;

use rocket::routes;
use rocket_contrib::serve::StaticFiles;

fn main() {
    let routes = routes![
        // index,
        uploads::get_file,
        uploads::get_all_files,
        uploads::multipart_upload,
    ];

    rocket::ignite()
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .mount("/", routes)
        .launch();
}
