use super::super::domain::{
    design::{DomainDesign},
    user::{DomainUser},
};

pub struct RepositoryDesign {}

impl RepositoryDesign {
    pub fn new() -> RepositoryDesign {
        RepositoryDesign {}
    }

    pub fn find_designs_total(&self, page_size: i32) -> i32 {
        page_size*2
    }

    pub fn find_designs(&self, page: i32, page_size: i32) -> Vec<DomainDesign> {
        let mut designs = Vec::new();
        let user = DomainUser::new(
            1,
            "a太郎".to_string(),
            "http://~".to_string(),
        );
        designs.push(
            DomainDesign::new(
                1,
                "タイトル".to_string(),
                "http://thumb".to_string(),
                user,
                page,
                page_size,
            )
        );

        designs
    }
}