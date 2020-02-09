use actix_web::HttpResponse;
use super::super::response;

pub fn index() -> HttpResponse {
    response::health::response()
}