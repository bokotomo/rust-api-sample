use super::super::domain::{
    developper::DomainDevelopper,
    user::DomainUser,
};
pub struct RepositoryDevelop {}

impl RepositoryDevelop {
    pub fn new() -> RepositoryDevelop {
        RepositoryDevelop {}
    }

    pub fn find_developper_total(&self, page_size: i32) -> i32 {
        page_size * 2
    }

    pub fn find_developpers(&self, page: i32, page_size: i32) -> Vec<DomainDevelopper> {
        let mut developpers = Vec::new();
        for i in 0..10 {
            let post_images = "http://localhost:3000/images/content2.jpg".to_string();
            let user = DomainUser::new(
                i,
                "a太郎".to_string(),
                "http://localhost:3000/images/user1.jpg".to_string(),
            );
            developpers.push(
                DomainDevelopper::new(
                    i,
                    post_images,
                    user,
                    page*page_size,
                )
            );
        }

        developpers
    }
}