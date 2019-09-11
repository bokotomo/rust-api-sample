use super::super::domain::user::{DomainUser};

pub fn find_users(page: i32, page_size: i32) -> Vec<DomainUser> {
    let mut users = Vec::new();
    users.push(DomainUser {
        id: page + page_size,
        name: "a太郎".to_string(),
        image: "http://~".to_string(),
    });
    
    users
}