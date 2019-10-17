// 開発物一覧の情報
use super::super::domain::user::DomainUser;

pub struct DomainDevelop {
    id: i32,
    // タイトル
    title: String,
    // サブタイトル
    sub_title: String,
    // サムネイル画像
    thumbnail: String,
    // いいね数
    good: i32,
    // コメント数
    comment: i32,
    user: DomainUser,
}

impl DomainDevelop {
    pub fn new_develops(id: i32, title: String, sub_title: String, thumbnail: String, good: i32, comment: i32, user: DomainUser) -> DomainDevelop {
        DomainDevelop {
            id,
            title,
            sub_title,
            thumbnail,
            good,
            comment,
            user,
        }
    }
    pub fn new(id: i32, title: String, user: DomainUser) -> DomainDevelop {
        DomainDevelop {
            id,
            title,
            sub_title: "".to_string(),
            thumbnail: "".to_string(),
            good: 0,
            comment: 0,
            user,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn sub_title(&self) -> &str {
        &self.sub_title
    }
    pub fn thumbnail(&self) -> &str {
        &self.thumbnail
    }
    pub fn good(&self) -> &i32 {
        &self.good
    }
    pub fn comment(&self) -> &i32 {
        &self.comment
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