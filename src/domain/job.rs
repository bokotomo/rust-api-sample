// 求人情報
use super::super::domain::company::{DomainCompany};

pub struct DomainJob {
    pub id: i32,
    pub company: DomainCompany,
    pub title: String,
    pub title_sub: String,
    pub tag: String,
    pub description: String,
    pub recruiment_period: String,
    pub location: String,
    pub salary: String,
    pub type_of_work: String,
    pub employees: i32,
}