use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpServer};
use std::sync::Mutex;

mod api;
mod utils;

use api::MyData::*;

use utils::util::*;

struct AppStateWithCounter {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/")]
async fn start(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    let app_name = &data.app_name;

    *counter += 1;

    format!("appName is : {app_name} Request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let status = web::Data::new(AppStateWithCounter {
        app_name: "my first rust server".to_string(),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .max_age(3600)
            .allowed_methods(vec!["GET", "POST"]);

        App::new()
            .wrap(cors)
            .app_data(status.clone())
            .service(start)
            .service(
                web::scope("/MyData")
                    .service(addMyData)
                    .service(select_all)
                    .service(select_myData_by_id)
                    .service(delete_myData_by_id)
                    .service(update_myData_by_id),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
