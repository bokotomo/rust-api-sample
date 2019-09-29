// ユーザの記事
pub struct DomainUserExperience {
    id: i32,
    // 記事タイトル
    title: String,
    // 期間
    period: String,
    // 詳細
    description: String,
}

impl DomainUserExperience {
    pub fn new(id: i32, title: String, period: String, description: String) -> DomainUserExperience {
        DomainUserExperience {
            id,
            title,
            period,
            description,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn period(&self) -> &str {
        &self.period
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}