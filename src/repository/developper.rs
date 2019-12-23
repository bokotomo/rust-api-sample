use super::super::domain::{
    developper::DomainDevelopper,
    user::DomainUser,
};

pub struct RepositoryDevelopper {}

impl RepositoryDevelopper {
    pub fn new() -> RepositoryDevelopper {
        RepositoryDevelopper {}
    }

    pub fn find_developper_total(&self, page_size: i32) -> i32 {
        page_size * 2
    }

    pub fn find_developpers(&self, page: i32, page_size: i32) -> Vec<DomainDevelopper> {
        let mut developpers = Vec::new();

        for i in 0..page * page_size {
            let mut post_images = Vec::new();
            post_images.push("http://localhost:3000/images/content2.jpg".to_string());
            post_images.push("http://localhost:3000/images/content2.jpg".to_string());
            post_images.push("http://localhost:3000/images/content1.jpg".to_string());
            post_images.push("http://localhost:3000/images/content1.jpg".to_string());

            let user = DomainUser::new(
                i,
                format!("{} - {}", "エンジニア太郎".to_string(), i),
                "http://localhost:3000/images/user1.jpg".to_string(),
            );
            developpers.push(
                DomainDevelopper::new(
                    i,
                    post_images,
                    user,
                )
            );
        }

        developpers
    }
}