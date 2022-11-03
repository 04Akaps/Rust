use serde::{Serialize, Deserialize};
use serde_json::{from_str};
use actix_web::{Responder, HttpResponse, web::Bytes, web};
use mysql::prelude::Queryable;
use mysql::params;

use log::{debug, error, info, trace, warn};

#[path = "utils.rs"]
mod utils;

use utils::get_db;

#[derive(Serialize)]
struct Hello {
    test : u64
}

// https://serde.rs/attr-rename.html
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TitleTable {
    id : i64,
    title : String,
}



pub async fn hello() -> impl Responder {

    debug!("Boom!");

    HttpResponse::Ok()
        .json(Hello { test : 3})
}


pub async fn delete_data(info: web::Path<i64>) -> impl Responder {
    let query_number = info.into_inner();
    let mut conn = get_db();

    let query = format!("DELETE FROM title_table WHERE id = {}", query_number);

    match conn
    .query_map(
        query, |title| TitleTable{ id : query_number, title}) {
            Ok(_) => {
                // 로그를 찍고 return
                warn!("deleted id is {:?}", query_number);

                HttpResponse::Ok()
                .json(format!("delete id = {}", query_number))
            },
            Err(err) => {
                // 로그를 찍고 return
                error!("error is : {:?}",err);

                HttpResponse::Ok()
                .json(format!("error is : {:?}",err))
            },
            }
        
}

pub async fn update_data(bytes : Bytes) -> impl Responder {
    let test = String::from_utf8(bytes.to_vec()).unwrap();
    let temp :TitleTable = from_str(&test).expect("convertError");

    let mut conn = get_db();

    let query = format!("update title_table set title = '{}' where id = {}",&temp.title, &temp.id);

    conn
    .query_map(
        query, |title| TitleTable{ id : temp.id , title}).expect("이게 안되냐??");


    HttpResponse::Ok()
    .json(temp)
}

pub async fn get_oneData(info: web::Path<i64>) -> impl Responder {
    let query_number = info.into_inner();
    let mut conn = get_db();

    let query = format!("SELECT title from title_table where id = {}", query_number);

    let selected_payments = conn
    .query_map(
        query, |title| TitleTable{ id : query_number, title}).expect("이게 안되냐??");


    HttpResponse::Ok()
    .json(selected_payments)
}

pub async fn get_all_data() -> impl Responder {
    let mut conn = get_db();

    let query = format!("SELECT * from title_table");

    let data_list = conn.query_map(query, |(id, title)| TitleTable { id , title}).expect("이것도 못하냐..?");

    HttpResponse::Ok()
    .json(data_list)
}

pub async fn post_data(bytes : Bytes) -> impl Responder {
    println!("new Data");

    let test = String::from_utf8(bytes.to_vec()).unwrap();
    let temp : TitleTable = from_str(&test).expect("Body value Error");
    let temp_test = vec![&temp];

    let mut conn = get_db();

    conn.exec_batch(
        r"INSERT INTO title_table (id, title) VALUES (:id, :title)",
        temp_test.iter().map(|p| params!{
            "id" => p.id,
            "title" => p.title.as_str()
        }) 
    ).expect("이게 뭘까");


    HttpResponse::Ok()
        .json(temp)
}
