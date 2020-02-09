//extern crate diesel;
//extern crate r2d2;
//extern crate r2d2_diesel;

use actix_web::{
    HttpResponse,
    web,
};
//use diesel::MysqlConnection;
//use r2d2::Pool;
use super::super::{
    service,
    request,
    response,
};

pub fn index(
    payload: web::Query<request::design::Index>,
//    db: web::Data<Pool<MysqlConnection>>,
) -> HttpResponse {
    let (domain_designs, total) = &service::design::index(
        payload.page,
        payload.page_size,
    );
    response::design_index::response(domain_designs, &total)
}