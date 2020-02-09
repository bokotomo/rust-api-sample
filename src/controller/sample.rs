use actix_web::{
    web,
    HttpResponse,
};
use super::super::request;

pub fn index(item: web::Query<request::sample::Index>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0)
}