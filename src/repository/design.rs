use super::super::domain::{
    design::DomainDesign,
    user::DomainUser,
};
// use super::super::driver::db::establish_connection;
// use super::super::driver::db::CONNECTION_MYSQL;
use super::super::schema::design::dsl::*;
use super::super::model::design::Design;

use diesel::prelude::*;

pub struct RepositoryDesign {}

impl RepositoryDesign {
    pub fn new() -> RepositoryDesign {
        RepositoryDesign {}
    }

    pub fn find_designs_total(&self, page_size: i32) -> i32 {
        page_size * 2
    }

    pub fn find_designs(&self, page: i32, page_size: i32) -> Vec<DomainDesign> {
        // establish_connection();
        // let results = design.filter(title.eq("OK"))
        //     .limit(5)
        //     .load::<Design>(&CONNECTION_MYSQL)
        //     .expect("Error loading design");

        // println!("Displaying {} posts", results.len());
        // for res in results {
        //     println!("{}", res.title);
        //     println!("----------\n");
        // }

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