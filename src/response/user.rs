use actix_web::HttpResponse;
use super::super::domain::user::DomainUser;
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserIndex {
    id: i32,
    title: String,
    good: i32,
    comment: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Creation {
    id: i32,
    title: String,
    image: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Article {
    id: i32,
    title: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Experience {
    id: i32,
    title: String,
    period: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserShow {
    id_display: String,
    name: String,
    location: String,
    image: String,
    image_background: String,
    good_total: i32,
    description: String,
    tags: Vec<String>,
    creations: Vec<Creation>,
    articles: Vec<Article>,
    experiences: Vec<Experience>,
}

pub fn response_user_index(domain_users: &Vec<DomainUser>) -> HttpResponse {
    let mut response_users = Vec::new();
    for domain_user in domain_users {
        response_users.push(UserIndex {
            id: *domain_user.id(),
            title: domain_user.name().to_string(),
            good: *domain_user.id(),
            comment: *domain_user.id(),
        });
    }

    HttpResponse::Ok().json(response_users)
}

pub fn response_user_show(domain_user: &DomainUser) -> HttpResponse {
    let mut creations = Vec::new();
    let mut articles = Vec::new();
    let mut experiences = Vec::new();

    for domain_creation in domain_user.creations() {
        creations.push(
            Creation {
                id: *domain_creation.id(),
                title: domain_creation.title().to_string(),
                image: domain_creation.image().to_string(),
            }
        )
    }
    for domain_article in domain_user.articles() {
        articles.push(
            Article {
                id: *domain_article.id(),
                title: domain_article.title().to_string(),
                url: domain_article.url().to_string(),
            }
        )
    }
    for domain_experience in domain_user.experiences() {
        experiences.push(
            Experience {
                id: *domain_experience.id(),
                title: domain_experience.title().to_string(),
                period: domain_experience.period().to_string(),
                description: domain_experience.description().to_string(),
            }
        )
    }

    HttpResponse::Ok().json(
        UserShow {
            id_display: domain_user.id_display().to_string(),
            name: domain_user.name().to_string(),
            location: domain_user.location().to_string(),
            image: domain_user.image().to_string(),
            image_background: domain_user.image_background().to_string(),
            good_total: *domain_user.good_total(),
            description: domain_user.description().to_string(),
            tags: domain_user.tags().to_vec(),
            creations,
            articles,
            experiences,
        }
    )
}