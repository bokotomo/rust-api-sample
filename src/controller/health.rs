use actix_web::HttpResponse;
use super::super::response;

pub fn health_index() -> HttpResponse {
    response::health::response()
}