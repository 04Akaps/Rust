use actix_web::{delete, error, get, post, put, web, Error, HttpResponse, Responder, Result};
use futures::StreamExt;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};

#[path = "../utils/util.rs"]
mod utils;
use utils::get_db_object;

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    name: String,
    address: String,
    age: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NewName {
    newName: String,
}

const MAX_SIZE: usize = 262_144;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// https://www.vultr.com/docs/building-rest-apis-in-rust-with-actix-web/

#[put("chageNameById/{id}")]
async fn update_myData_by_id(id: web::Path<u64>, req: web::Json<NewName>) -> HttpResponse {

    let newName = String::from(&req.newName);

    let mut conn = get_db_object();
    // update myData set name = "change" where id = 1
    let query = format!(
        "UPDATE myData SET name = '{}' WHERE id = {}",
        req.newName, id
    );

    let result = conn.query_iter(query).expect("쿼리 실패");

    println!("updated id : {:?} -> newName : {:?}",id, req);

    HttpResponse::Ok().body(format!("username: {:?}", req))
}

#[delete("deleteByid/{id}")]
async fn delete_myData_by_id(id: web::Path<u64>) -> Result<impl Responder> {
    let mut conn = get_db_object();

    let query = format!("DELETE FROM MyData WHERE id = {}", id);

    let result = conn
        .query_map(query, |(name, address, age)| MyData { name, address, age })
        .expect("쿼리 실패");

    Ok(web::Json(result))
}

#[get("/selectAll")]
async fn select_all() -> Result<impl Responder> {
    let mut conn = get_db_object();
    let query = "SELECT name, address, age from myData";

    let result = conn
        .query_map(query, |(name, address, age)| MyData { name, address, age })
        .expect("쿼리 실패");

    println!("get select All from MyData");

    Ok(web::Json(result))
}

#[get("/selectByid/{id}")]
async fn select_myData_by_id(id: web::Path<u64>) -> Result<impl Responder> {
    let mut conn = get_db_object();

    let query = format!("SELECT name, address, age from myData where id = {}", id);

    let result = conn
        .query_map(query, |(name, address, age)| MyData { name, address, age })
        .expect("쿼리 실패");

    Ok(web::Json(result))
}

#[post("/addMyData")]
async fn addMyData(mut payload: web::Payload) -> Result<impl Responder> {
    // https://docs.rs/actix-web/4.2.1/actix_web/web/struct.Payload.html

    println!("New POST request to create a post!");

    let mut body = web::BytesMut::new();

    //  데이터를 payload stream으로 모으는 while문
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let obj: MyData = serde_json::from_slice::<MyData>(&body).expect("실패....");

    println!("저장 될 데이터는 이와 같습니다. {:#?}", obj);

    let mut conn = get_db_object();

    let result = conn
        .exec_batch(
            "INSERT INTO myData (name, address, age) VALUES (:name, :address, :age)",
            vec![obj].iter().map(|p| {
                params! {
                    "name" => &p.name,
                    "address" => &p.address,
                    "age" => &p.age
                }
            }),
        )
        .expect("무슨 코드지 이게..");

    Ok(web::Json(result))
}
