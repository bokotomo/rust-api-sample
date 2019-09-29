// デザイン一覧の情報
use super::super::domain::user::DomainUser;

pub struct DomainDevelopper {
    id: i32,
    // デザインタイトル
    post_images: Vec<String>,
    user: DomainUser,
}

impl DomainDevelopper {
    pub fn new(id: i32, post_images: Vec<String>, user: DomainUser) -> DomainDevelopper {
        DomainDevelopper {
            id,
            post_images,
            user,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn post_images(&self) -> &Vec<String> {
        &self.post_images
    }
    pub fn user_good_total(&self) -> &i32 {
        self.user.good_total()
    }
    pub fn user_id(&self) -> &i32 {
        self.user.id()
    }
    pub fn user_name(&self) -> &str {
        self.user.name()
    }
    pub fn user_image(&self) -> &str {
        self.user.image()
    }
    pub fn user_location(&self) -> &str {
        self.user.location()
    }
}
