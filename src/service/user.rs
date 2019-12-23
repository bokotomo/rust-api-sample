use super::super::{
    domain,
    repository,
};

pub fn index(repository_user: repository::user::RepositoryUser, page: i32, page_size: i32) -> Vec<domain::user::DomainUser> {
    let domain_users = repository_user.find_users(page, page_size);
    domain_users
}

pub fn show(repository_user: repository::user::RepositoryUser, user_id: i32) -> domain::user::DomainUser {
    let domain_user = repository_user.find_user(user_id);
    domain_user
}