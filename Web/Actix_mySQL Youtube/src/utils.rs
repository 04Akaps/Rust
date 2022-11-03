

use dotenv::dotenv;
use mysql::Pool;
use serde::{Serialize};

extern  crate log4rs;

#[derive(Serialize)]
pub struct Status {
    pub status : String
}

pub fn get_logs() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap()
}

pub fn get_db() -> mysql::PooledConn {
    dotenv().ok();

    let db_user = std::env::var("DB_USER_NAME").expect("DB_USER_NAME");
    let db_passwrd = std::env::var("DB_USER_NAME").expect("DB_USER_NAME");
    let db_ip = std::env::var("DB_IP").expect("DB_IP");
    let db_port = std::env::var("DB_PORT").expect("DB_PORT");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME");

    let db_url = format!("mysql://{}:{}@{}:{}/{}" 
            ,db_user
            ,db_passwrd
            ,db_ip
            ,db_port
            ,db_name
    );
    let pool = Pool::new(db_url.as_str()).unwrap();

    pool.get_conn().unwrap()
}



