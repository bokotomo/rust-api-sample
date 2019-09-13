use super::super::domain::job::{DomainJob};
use super::super::domain::company::{DomainCompany};
pub struct RepositoryJob {}

impl RepositoryJob {
    pub fn new() -> RepositoryJob {
        RepositoryJob {}
    }

    pub fn find_jobs(&self, page: i32, page_size: i32) -> Vec<DomainJob> {
        let mut jobs = Vec::new();
        let company = DomainCompany::new(
            1,
            "会社名".to_string(),
            "http://".to_string(),
            "http://".to_string(),
        );
        jobs.push(
            DomainJob::new(
                page + page_size,
                company,
                "タイトル".to_string(),
                "サブ詳細".to_string(),
                "たぐ".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                0,
            )
        );
        
        jobs
    }

    pub fn find_job(&self, page: i32, page_size: i32) -> DomainJob {
        let company = DomainCompany::new(
            1,
            "会社名".to_string(),
            "http://".to_string(),
            "http://".to_string(),
        );

        DomainJob::new(
            page + page_size,
            company,
            "タイトル".to_string(),
            "サブ詳細".to_string(),
            "たぐ".to_string(),
            "詳細".to_string(),
            "~ 2019/08/12".to_string(),
            "United States / San Francisco, CA".to_string(),
            "$72,000/year".to_string(),
            "働き方".to_string(),
            12,
        )
    }
}