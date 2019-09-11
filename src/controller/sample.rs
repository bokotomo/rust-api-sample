use actix_web::{
    web,
    HttpResponse,
};
use super::super::request::sample::{RequestSampleIndex};

pub fn sample_index(item: web::Json<RequestSampleIndex>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0)
}