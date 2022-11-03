mod utils;
mod api;

use utils::{Status, get_logs};
use api::{hello, post_data, get_oneData, get_all_data, delete_data, update_data};
use actix_web::{ App, HttpServer, Responder, web, HttpResponse};


async fn start() -> impl Responder {
    HttpResponse::Ok()
        .json(Status { status : "Start".to_string()})
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    get_logs();

    HttpServer::new(move || {

    
        App::new()
            .route("/", web::get().to(start))
            .route("/hello" , web::get().to(hello))
            .service(
                web::scope("/api")
                .route("/add" , web::post().to(post_data))
                .route("/get/{id}", web::get().to(get_oneData))
                .route("/getAll", web::get().to(get_all_data))
                .route("/delete/{id}", web::delete().to(delete_data))
                .route("/update", web::put().to(update_data))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
