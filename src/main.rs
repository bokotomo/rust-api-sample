extern crate gcs_api;
extern crate json;
extern crate serde_json;

use gcs_api::controller::health::{health_index};
use gcs_api::controller::design::{design_index};
use gcs_api::controller::sample::{sample_index};
use gcs_api::controller::user::{user_index};
use gcs_api::controller::job::{job_index, job_show};

use actix_web::{
    middleware,
    web,
    App,
    HttpServer,
};
use dotenv;

fn setup() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}

fn main() -> std::io::Result<()> {
    setup();
    let url = dotenv::var("HOST").unwrap().to_string() + ":" + &dotenv::var("PORT").unwrap();

    HttpServer::new(|| {
        App::new()
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