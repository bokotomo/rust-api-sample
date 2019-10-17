use super::super::domain::{
    developper::DomainDevelopper,
    develop::DomainDevelop,
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

    // トレンド開発物一覧
    pub fn find_trends(&self) -> Vec<DomainDevelop> {
        let mut trends = Vec::new();
        for i in 0..4 {
            let user = DomainUser::new(
                i,
                "a太郎".to_string(),
                "http://localhost:3000/images/user2.png".to_string(),
            );
            trends.push(
                DomainDevelop::new_develops(
                    i,
                    "タイトル".to_string(),
                    "サブタイトル".to_string(),
                    "http://localhost:3000/images/content2.jpg".to_string(),
                    1,
                    12,
                    user,
                )
            );
        }

        trends
    }

    // 人気開発物一覧
    pub fn find_popularities(&self) -> Vec<DomainDevelop> {
        let mut popularities = Vec::new();
        for i in 0..4 {
            let user = DomainUser::new(
                i,
                "a太郎".to_string(),
                "http://localhost:3000/images/user2.png".to_string(),
            );
            popularities.push(
                DomainDevelop::new_develops(
                    i,
                    "タイトル".to_string(),
                    "http://localhost:3000/images/content2.jpg".to_string(),
                    "サブタイトル".to_string(),
                    1,
                    12,
                    user,
                )
            );
        }

        popularities
    }
}