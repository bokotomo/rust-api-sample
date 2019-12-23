use actix_web::HttpResponse;
use super::super::domain::develop::DomainDevelop;
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Develop {
    id: i32,
    title: String,
    sub_title: String,
    thumbnail: String,
    good: i32,
    comment: i32,
    user_name: String,
    user_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevelopIndex {
    trends: Vec<Develop>,
    popularities: Vec<Develop>,
}

pub fn response(domain_develop_trends: &Vec<DomainDevelop>, domain_develop_popularities: &Vec<DomainDevelop>) -> HttpResponse {
    let mut trends = Vec::with_capacity(domain_develop_trends.len());
    for domain_trend in domain_develop_trends {
        trends.push(Develop {
            id: *domain_trend.id(),
            title: domain_trend.title().to_string(),
            sub_title: domain_trend.sub_title().to_string(),
            thumbnail: domain_trend.title().to_string(),
            good: *domain_trend.good(),
            comment: *domain_trend.comment(),
            user_name: domain_trend.user_name().to_string(),
            user_image: domain_trend.user_image().to_string(),
        });
    }

    let mut popularities = Vec::with_capacity(domain_develop_popularities.len());
    for domain_popularity in domain_develop_popularities {
        popularities.push(Develop {
            id: *domain_popularity.id(),
            title: domain_popularity.title().to_string(),
            sub_title: domain_popularity.sub_title().to_string(),
            thumbnail: domain_popularity.title().to_string(),
            good: *domain_popularity.good(),
            comment: *domain_popularity.comment(),
            user_name: domain_popularity.user_name().to_string(),
            user_image: domain_popularity.user_image().to_string(),
        });
    }

    HttpResponse::Ok().json(DevelopIndex {
        trends,
        popularities,
    })
}