// ユーザ情報
use super::super::domain::{
    user_creation::DomainUserCreation,
    user_article::DomainUserArticle,
    user_experience::DomainUserExperience,
};

pub struct DomainUser {
    id: i32,
    // 表示されるユーザID
    id_display: String,
    // ユーザ名
    name: String,
    // ユーザ画像
    image: String,
    // ユーザ背景画像
    image_background: String,
    // 詳細
    description: String,
    // ユーザ住んでる場所
    location: String,
    // いいね総数
    good_total: i32,
    // ユーザタグ
    tags: Vec<String>,
    // 制作物一覧
    creations: Vec<DomainUserCreation>,
    // 記事一覧
    articles: Vec<DomainUserArticle>,
    // 職歴一覧
    experiences: Vec<DomainUserExperience>,
}

impl DomainUser {
    pub fn new(id: i32, name: String, image: String) -> DomainUser {
        DomainUser {
            id,
            id_display: "id_display".to_string(),
            name,
            image,
            image_background: "image_background".to_string(),
            description: "description".to_string(),
            location: "tokyo".to_string(),
            good_total: 122,
            tags: Vec::new(),
            creations: Vec::new(),
            articles: Vec::new(),
            experiences: Vec::new(),
        }
    }
    pub fn new_user(id_display: String, name: String, image: String, description: String, image_background: String, tags: Vec<String>, creations: Vec<DomainUserCreation>, articles: Vec<DomainUserArticle>, experiences: Vec<DomainUserExperience>) -> DomainUser {
        DomainUser {
            id: 1,
            id_display,
            name,
            image,
            image_background,
            description,
            location: "tokyo".to_string(),
            good_total: 122,
            tags,
            creations,
            articles,
            experiences,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn id_display(&self) -> &str {
        &self.id_display
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn image(&self) -> &str {
        &self.image
    }
    pub fn image_background(&self) -> &str {
        &self.image_background
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn location(&self) -> &str {
        &self.location
    }
    pub fn good_total(&self) -> &i32 {
        &self.good_total
    }
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
    pub fn creations(&self) -> &Vec<DomainUserCreation> {
        &self.creations
    }
    pub fn articles(&self) -> &Vec<DomainUserArticle> {
        &self.articles
    }
    pub fn experiences(&self) -> &Vec<DomainUserExperience> {
        &self.experiences
    }
}