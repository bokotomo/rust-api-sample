use super::super::domain::{
    develop::DomainDevelop,
    user::DomainUser,
};

pub struct RepositoryDevelop {}

impl RepositoryDevelop {
    pub fn new() -> RepositoryDevelop {
        RepositoryDevelop {}
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