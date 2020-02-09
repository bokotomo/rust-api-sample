extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::prelude::*;
//use dotenv::dotenv;
//use std::env;
use super::super::model::design::NewDesign;
use super::super::schema::design;

//use diesel::MysqlConnection;
//use r2d2_diesel::ConnectionManager;
//use self::r2d2::ManageConnection;

pub fn tomo() {
//    dotenv().ok();
//    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLが設定されていない。");
//    print!("{}", database_url);
//
//    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
//    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
//    let p = pool.clone();
//    let connection = p.get();
//
//    assert!(connection.is_ok());

}

//pub fn connection() {
//    let p = pool.clone();
//    let connection = p.get();
//}

pub fn create_design(conn: &MysqlConnection, title: &str) -> usize {
    let new_design = NewDesign { title };

    diesel::insert_into(design::table)
        .values(&new_design)
        .execute(conn)
        .expect("Error saving new post")
}