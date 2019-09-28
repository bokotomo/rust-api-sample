// サービス全体を通した企業
pub struct DomainCompany {
    id: i32,
    // 会社名
    name: String,
    // 会社ロゴ
    logo: String,
    // 会社サムネイル
    thumbnail: String,
}

impl DomainCompany {
    pub fn new(id: i32, name: String, logo: String, thumbnail: String) -> DomainCompany {
        DomainCompany {
            id,
            name,
            logo,
            thumbnail,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn logo(&self) -> &str {
        &self.logo
    }
    pub fn thumbnail(&self) -> &str {
        &self.thumbnail
    }
}
