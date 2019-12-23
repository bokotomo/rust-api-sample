use super::super::domain::{
    design::DomainDesign,
    designer::DomainDesigner,
    user::DomainUser,
};
// use super::super::driver::db::establish_connection;
// use super::super::driver::db::CONNECTION_MYSQL;
//use super::super::schema::design::dsl::*;
//use super::super::model::design::Design;
//use diesel::prelude::*;
use super::super::driver::db::tomo;

pub struct RepositoryDesign {}

impl RepositoryDesign {
    pub fn new() -> RepositoryDesign {
        RepositoryDesign {}
    }

    pub fn find_designs_total(&self, page_size: i32) -> i32 {
        page_size * 2
    }

    pub fn find_designers_total(&self, page_size: i32) -> i32 {
        page_size * 2
    }

    // ピックアップデザイン一覧
    pub fn find_pickups(&self) -> Vec<DomainDesign> {
        let mut pickups = Vec::new();
        for i in 0..4 {
            let user = DomainUser::new(
                i,
                "a太郎".to_string(),
                "http://localhost:3000/images/user2.png".to_string(),
            );
            pickups.push(
                DomainDesign::new(
                    i,
                    "タイトル".to_string(),
                    "http://localhost:3000/images/content2.jpg".to_string(),
                    user,
                    1,
                    12,
                )
            );
        }

        pickups
    }

    // デザイン一覧
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
        print!("oooooooooooooooooooooooo");
        tomo();

        let mut designs = Vec::new();
        for i in 0..10 {
            let user = DomainUser::new(
                i,
                "a太郎".to_string(),
                "http://localhost:3000/images/user1.jpg".to_string(),
            );
            designs.push(
                DomainDesign::new(
                    i,
                    "タイトル".to_string(),
                    "http://localhost:3000/images/content1.jpg".to_string(),
                    user,
                    page,
                    page_size,
                )
            );
        }

        designs
    }

    // デザイナー一覧
    pub fn find_designers(&self, page: i32, page_size: i32) -> Vec<DomainDesigner> {
        let mut designers = Vec::new();

        for i in 0..page * page_size {
            let mut post_images = Vec::new();
            post_images.push("http://localhost:3000/images/content2.jpg".to_string());
            post_images.push("http://localhost:3000/images/content2.jpg".to_string());
            post_images.push("http://localhost:3000/images/content1.jpg".to_string());
            post_images.push("http://localhost:3000/images/content1.jpg".to_string());

            let user = DomainUser::new(
                i,
                format!("{} - {}", "デザイナー二郎".to_string(), i),
                "http://localhost:3000/images/user2.png".to_string(),
            );
            designers.push(
                DomainDesigner::new(
                    i,
                    post_images,
                    user,
                )
            );
        }

        designers
    }
}