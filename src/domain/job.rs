// 求人の情報
use super::super::domain::company::DomainCompany;

pub struct DomainJob {
    id: i32,
    company: DomainCompany,
    // 仕事名
    title: String,
    // サブタイトル
    title_sub: String,
    // タグ
    tag: String,
    // 仕事詳細
    description: String,
    // 期限
    recruiment_period: String,
    // 仕事場所
    location: String,
    // 給与
    salary: String,
    // 仕事の形式
    type_of_work: String,
    // 従業員数
    employees: i32,
}

impl DomainJob {
    pub fn new_jobs(id: i32, company: DomainCompany, title: String, title_sub: String, tag: String, description: String, recruiment_period: String, location: String, salary: String, type_of_work: String, employees: i32) -> DomainJob {
        DomainJob {
            id,
            company,
            title,
            title_sub,
            tag,
            description,
            recruiment_period,
            location,
            salary,
            type_of_work,
            employees,
        }
    }
    pub fn new_job(id: i32, company: DomainCompany, title: String, title_sub: String, tag: String) -> DomainJob {
        DomainJob {
            id,
            company,
            title,
            title_sub,
            tag,
            description: "".to_string(),
            recruiment_period: "".to_string(),
            location: "".to_string(),
            salary: "".to_string(),
            type_of_work: "".to_string(),
            employees: 0,
        }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn title(&self) -> String {
        self.title.to_string()
    }
    pub fn title_sub(&self) -> String {
        self.title_sub.to_string()
    }
    pub fn tag(&self) -> String {
        self.tag.to_string()
    }
    pub fn description(&self) -> String {
        self.description.to_string()
    }
    pub fn recruitment_period(&self) -> String {
        self.recruiment_period.to_string()
    }
    pub fn location(&self) -> String {
        self.location.to_string()
    }
    pub fn salary(&self) -> String {
        self.salary.to_string()
    }
    pub fn type_of_work(&self) -> String {
        self.type_of_work.to_string()
    }
    pub fn employees(&self) -> i32 {
        self.employees
    }
    pub fn company_name(&self) -> String {
        self.company.name()
    }
    pub fn company_logo(&self) -> String {
        self.company.logo()
    }
    pub fn company_thumbnail(&self) -> String {
        self.company.thumbnail()
    }
}