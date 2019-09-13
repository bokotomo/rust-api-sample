// デザイン情報
use super::super::domain::user::{DomainUser};

pub struct DomainDesign {
    id: i32,
    title: String,
    thumbnail: String,
    user: DomainUser,
    good: i32,
    comment: i32,
}

impl DomainDesign {
    pub fn new(id: i32, title: String, thumbnail: String, user: DomainUser, good: i32, comment: i32) -> DomainDesign {
        DomainDesign {
            id,
            title,
            thumbnail,
            user,
            good,
            comment,
        }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn title(&self) -> String {
        self.title.to_string()
    }
    pub fn thumbnail(&self) -> String {
        self.thumbnail.to_string()
    }
    pub fn good(&self) -> i32 {
        self.good
    }
    pub fn comment(&self) -> i32 {
        self.comment
    }
    pub fn user_id(&self) -> i32 {
        self.user.id()
    }
    pub fn user_name(&self) -> String {
        self.user.name()
    }
    pub fn user_image(&self) -> String {
        self.user.image()
    }
}
