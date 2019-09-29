use super::super::domain::{
    user::DomainUser,
    user_creation::DomainUserCreation,
    user_article::DomainUserArticle,
    user_experience::DomainUserExperience,
};

pub struct RepositoryUser {}

impl RepositoryUser {
    pub fn new() -> RepositoryUser {
        RepositoryUser {}
    }

    pub fn find_users(&self, page: i32, page_size: i32) -> Vec<DomainUser> {
        let mut users = Vec::new();
        for _ in 0..10 {
            users.push(
                DomainUser::new(
                    page + page_size,
                    "a太郎".to_string(),
                    "http://localhost:3000/images/user1.jpg".to_string(),
                )
            );
        }

        users
    }

    pub fn find_user(&self, user_id: i32) -> DomainUser {
        let mut tags = Vec::new();
        let mut user_creation = Vec::new();
        let mut user_article = Vec::new();
        let mut user_experience = Vec::new();

        tags.push("UI/UX".to_string());
        tags.push("3D graphics".to_string());
        tags.push("Sound effect".to_string());

        user_creation.push(
            DomainUserCreation::new(
                user_id,
                "3D RPG".to_string(),
                "http://localhost:3000/images/content1.jpg".to_string(),
            )
        );
        user_creation.push(
            DomainUserCreation::new(
                2,
                "VR Shooting".to_string(),
                "http://localhost:3000/images/content1.jpg".to_string(),
            )
        );
        user_creation.push(
            DomainUserCreation::new(
                3,
                "Dragon".to_string(),
                "http://localhost:3000/images/content1.jpg".to_string(),
            )
        );

        user_article.push(
            DomainUserArticle::new(
                1,
                "Google".to_string(),
                "https://www.google.com".to_string(),
            )
        );
        user_article.push(
            DomainUserArticle::new(
                2,
                "Yahoo".to_string(),
                "https://www.google.com".to_string(),
            )
        );
        user_article.push(
            DomainUserArticle::new(
                3,
                "Yandex".to_string(),
                "https://www.google.com".to_string(),
            )
        );

        user_experience.push(
            DomainUserExperience::new(
                1,
                "A Games".to_string(),
                "2017-2019".to_string(),
                "Developed web services.".to_string(),
            )
        );
        user_experience.push(
            DomainUserExperience::new(
                2,
                "B Games".to_string(),
                "2017-2019".to_string(),
                "Developed web services.".to_string(),
            )
        );
        user_experience.push(
            DomainUserExperience::new(
                3,
                "C Games".to_string(),
                "2017-2019".to_string(),
                "Developed web services.".to_string(),
            )
        );

        DomainUser::new_user(
            "nameId".to_string(),
            "a太郎".to_string(),
            "http://localhost:3000/images/user1.jpg".to_string(),
            "The pace with which his followers increase ensures that he will not be late in reaching the three figures that his mother has, 107k. His consecration in the street style has meant struggling in fashion weeks around the world with Zinko, posing for photographers and religiously updating his Instagram account. At his age he has already been flashed by paparazzis from Monaco, Miami, Moscow or Los Angeles, the latter where he resides and, of course, his favorite.".to_string(),
            "http://localhost:3000/images/user1.jpg".to_string(),
            tags,
            user_creation,
            user_article,
            user_experience,
        )
    }
}