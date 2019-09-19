extern crate diesel;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use super::super::model::design::NewDesign;
use super::super::schema::design;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLが設定されていない。");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("接続失敗{}", database_url))
}

pub fn create_design(conn: &MysqlConnection, title: &str) -> usize {
    let new_design= NewDesign { title };

    diesel::insert_into(design::table)
        .values(&new_design)
        .execute(conn)
        .expect("Error saving new post")
}