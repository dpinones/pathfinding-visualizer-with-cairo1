use std::fs::File;
use std::io::prelude::*;

use actix_files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use clap::Parser;

mod run_cairo;
use run_cairo::run_cairo;

static SERVE_PATH: &str = "./static";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = match std::env::var("CREATIVE_CAIRO_PORT") {
        Ok(val) => val.parse::<u16>().unwrap(),
        Err(_e) => 8080,
    };
    println!("Serving port {}", port);
    HttpServer::new(|| {
        App::new()
            .route("/cairo", web::post().to(handle_connection))
            .service(actix_files::Files::new("/", SERVE_PATH).show_files_listing())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

fn write_cairo_to_file(cairo: String) -> String {
    let mut file = File::open("sketch/template.cairo").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let updated_contents = contents
        .replace("$1", "2")
        .replace("$2", "2")
        .replace("$3", "13")
        .replace("$4", "13");

    let mut file = File::create("sketch/lib.cairo").unwrap();
    file.write_all(updated_contents.as_bytes()).unwrap();
    String::from("sketch")
}

fn handle_cairo_run(cairo: String) -> String {
    println!("Handling cairo run: {:#?}", cairo);
    let path = write_cairo_to_file(cairo);
    // let path = "sketch".to_string();
    match run_cairo(&path) {
        Ok(v) => v,
        Err(e) => e.to_string(),
    }
}

async fn handle_connection(req_body: String) -> impl Responder {
    let run_response = handle_cairo_run(req_body);
    println!("{:#?}", run_response);
    HttpResponse::Ok().body(run_response)
}
