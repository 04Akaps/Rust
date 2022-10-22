use mysql::*;

pub const DB_USER_NAME: &'static str = "root";
pub const DB_PASSWORD: &'static str = "root";
pub const DB_IP: &'static str = "localhost";
pub const DB_PORT: u16 = 3306_u16;
pub const DB_NAME: &'static str = "test";

// CREATE TABLE myData (
//     id        INT NOT NULL AUTO_INCREMENT,
//     name     VARCHAR(20),
//     address    VARCHAR(20),
//     age   VARCHAR(50),
//      PRIMARY KEY(id)
//    ) ENGINE=MYISAM CHARSET=utf8;

// -> 이게 Test Table

pub fn get_db_object() -> mysql::PooledConn {
    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        DB_USER_NAME, DB_PASSWORD, DB_IP, DB_PORT, DB_NAME
    );

    let pool = Pool::new(db_url.as_str()).unwrap();
    let conn = pool.get_conn().unwrap();

    return conn;
}
