// デザイン一覧の情報
use super::super::domain::user::DomainUser;

pub struct DomainDevelopper {
    id: i32,
    // デザインタイトル
    title: String,
    user: DomainUser,
}

impl DomainDevelopper {
    pub fn new(id: i32, title: String, thumbnail: String, user: DomainUser, good: i32, comment: i32) -> DomainDevelopper {
        DomainDevelopper {
            id,
            title,
            user,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn title(&self) -> &str {
        &self.title
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
