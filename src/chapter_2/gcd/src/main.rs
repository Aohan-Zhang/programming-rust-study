use crate::controller::{get_index, post_gcd};
use actix_web::{web, App, HttpServer};

mod gcd;
mod model;
mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}

async fn start_server() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd2", web::post().to(post_gcd))
    });
    println!("server listening on port 3000");
    server
        .bind("127.0.0.1:3000")
        .expect("Can't bind to port 3000")
        .run()
        .await
}

