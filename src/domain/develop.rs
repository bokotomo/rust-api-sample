// デザイン一覧の情報
use super::super::domain::user::DomainUser;

pub struct DomainDevelop {
    id: i32,
    // 開発物タイトル
    title: String,
    user: DomainUser,
}

impl DomainDevelop {
    pub fn new(id: i32, title: String, user: DomainUser) -> DomainDevelop {
        DomainDevelop {
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
