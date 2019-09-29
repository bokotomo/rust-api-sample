use super::super::domain::user::DomainUser;
use super::super::repository::user::RepositoryUser;

pub fn service_user_index(repository_user: RepositoryUser, page: i32, page_size: i32) -> Vec<DomainUser> {
    let domain_users = repository_user.find_users(page, page_size);
    domain_users
}
pub fn service_user_show(repository_user: RepositoryUser, user_id: i32) -> DomainUser {
    let domain_user = repository_user.find_user(user_id);
    domain_user
}