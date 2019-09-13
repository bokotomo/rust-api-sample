use actix_web::{HttpResponse};
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseHealth {
    message: String,
}

pub fn response_health() -> HttpResponse {
    let response_health = ResponseHealth {
        message: "Success!".to_string(),
    };

    HttpResponse::Ok().json(response_health)
}