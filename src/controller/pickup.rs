use actix_web::{
    HttpResponse,
};
use super::super::{
    service,
    response,
};

pub fn index() -> HttpResponse {
    let domain_designs = &service::pickup::index();
    response::pickup_index::response(domain_designs)
}
