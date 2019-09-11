use super::super::domain::design::{DomainDesign};
use super::super::domain::user::{DomainUser};

pub fn find_designs(page: i32, page_size: i32) -> Vec<DomainDesign> {
    let mut designs = Vec::new();
    designs.push(DomainDesign {
        id: 1,
        title: "タイトル".to_string(),
        thumbnail: "http://~".to_string(),
        user: DomainUser {
            id: 1,
            name: "a太郎".to_string(),
            image: "http://~".to_string(),
        },
        good: page,
        comment: page_size
    });
    
    designs
}