// ユーザの記事
pub struct DomainUserArticle {
    id: i32,
    // 記事タイトル
    title: String,
    // 記事URL
    url: String,
}

impl DomainUserArticle {
    pub fn new(id: i32, title: String, url: String) -> DomainUserArticle {
        DomainUserArticle {
            id,
            title,
            url,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn url(&self) -> &str {
        &self.url
    }
}