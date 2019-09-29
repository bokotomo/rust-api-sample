use super::super::domain::user::DomainUser;

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
}