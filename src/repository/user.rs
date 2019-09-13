use super::super::domain::user::{DomainUser};

pub struct RepositoryUser {}

impl RepositoryUser {
    pub fn new() -> RepositoryUser {
        RepositoryUser {}
    }

    pub fn find_users(&self, page: i32, page_size: i32) -> Vec<DomainUser> {
        let mut users = Vec::new();
        users.push(
            DomainUser::new(
                page + page_size,
                "a太郎".to_string(),
                "http://~".to_string(),
            )
        );
        
        users
    }
}