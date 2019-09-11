// デザイン情報
use super::super::domain::user::{DomainUser};

pub struct DomainDesign {
    pub id: i32,
    pub title: String,
    pub thumbnail: String,
    pub user: DomainUser,
    pub good: i32,
    pub comment: i32,
}