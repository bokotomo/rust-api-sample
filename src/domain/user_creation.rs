// ユーザの投稿物（一時）
pub struct DomainUserCreation {
    id: i32,
    // タイトル
    title: String,
    // 画像
    image: String,
}

impl DomainUserCreation {
    pub fn new(id: i32, title: String, image: String) -> DomainUserCreation {
        DomainUserCreation {
            id,
            title,
            image,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn image(&self) -> &str {
        &self.image
    }
}