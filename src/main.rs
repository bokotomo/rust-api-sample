extern crate gcs_api;
extern crate json;
extern crate serde_json;

use dotenv;
use actix_cors::Cors;
use actix_web::{
    http,
    middleware,
    web,
    App,
    HttpServer,
};
use gcs_api::controller::{
    health::health_index,
    design::{design_index, pickup_index},
    sample::sample_index,
    user::user_index,
    job::{job_index, job_show},
};

fn setup() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}

fn main() -> std::io::Result<()> {
    setup();
    let url = dotenv::var("HOST").unwrap().to_string() + ":" + &dotenv::var("PORT").unwrap();
    // let url_allowed_origin = "http://localhost:3000";

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS", "HEAD", "DELETE", "PUT"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
                    .max_age(3600),
            )
            // ログ有効
            .wrap(middleware::Logger::default())
            // 制限
            .data(web::JsonConfig::default().limit(4096))
            // ルーティング
            .service(
                web::resource("/health")
                    .route(web::get()
                        .to_async(health_index)),
            )
            .service(
                web::resource("/sample")
                    .route(web::get()
                        .to_async(sample_index)),
            )
            .service(
                web::resource("/designs")
                    .route(web::get()
                        .to_async(design_index)),
            )
            .service(
                web::resource("/pickups")
                    .route(web::get()
                        .to_async(pickup_index)),
            )
            .service(
                web::resource("/users")
                    .route(web::get()
                        .to_async(user_index)),
            )
            .service(
                web::resource("/jobs")
                    .route(web::get()
                        .to_async(job_index)),
            )
            .service(
                web::resource("/job")
                    .route(web::get()
                        .to_async(job_show)),
            )
    })
        .bind(url)?
        .run()
}