// デザイン一覧の情報
use super::super::domain::user::DomainUser;

pub struct DomainDesigner {
    id: i32,
    // デザインタイトル
    post_images: String,
    user: DomainUser,
    // いいね総数
    good_total: i32,
}

impl DomainDesigner {
    pub fn new(id: i32, post_images: String, user: DomainUser, good_total: i32) -> DomainDesigner {
        DomainDesigner {
            id,
            post_images,
            user,
            good_total,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn post_images(&self) -> &str {
        &self.post_images
    }
    pub fn good_total(&self) -> &i32 {
        &self.good_total
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
}
