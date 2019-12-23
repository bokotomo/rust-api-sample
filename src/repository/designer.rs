use super::super::domain::{
    designer::DomainDesigner,
    user::DomainUser,
};

pub struct RepositoryDesigner {}

impl RepositoryDesigner {
    pub fn new() -> RepositoryDesigner {
        RepositoryDesigner {}
    }

    pub fn find_designers_total(&self, page_size: i32) -> i32 {
        page_size * 2
    }

    // デザイナー一覧
    pub fn find_designers(&self, page: i32, page_size: i32) -> Vec<DomainDesigner> {
        let mut designers = Vec::new();

        for i in 0..page * page_size {
            let mut post_images = Vec::new();
            post_images.push("http://localhost:3000/images/content2.jpg".to_string());
            post_images.push("http://localhost:3000/images/content2.jpg".to_string());
            post_images.push("http://localhost:3000/images/content1.jpg".to_string());
            post_images.push("http://localhost:3000/images/content1.jpg".to_string());

            let user = DomainUser::new(
                i,
                format!("{} - {}", "デザイナー二郎".to_string(), i),
                "http://localhost:3000/images/user2.png".to_string(),
            );
            designers.push(
                DomainDesigner::new(
                    i,
                    post_images,
                    user,
                )
            );
        }

        designers
    }
}