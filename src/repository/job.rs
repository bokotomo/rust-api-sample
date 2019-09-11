use super::super::domain::job::{DomainJob};
use super::super::domain::company::{DomainCompany};

pub fn find_jobs(page: i32, page_size: i32) -> Vec<DomainJob> {
    let mut jobs = Vec::new();
    jobs.push(
        DomainJob {
            id: page + page_size,
            company: DomainCompany {
                id: 1,
                name: "会社名".to_string(),
                logo: "http://".to_string(),
                thumbnail: "http://".to_string(),
            },
            title: "タイトル".to_string(),
            title_sub: "サブ詳細".to_string(),
            tag: "たぐ".to_string(),
            description: "".to_string(),
            recruiment_period: "".to_string(),
            location: "".to_string(),
            salary: "".to_string(),
            type_of_work: "".to_string(),
            employees: 0,
        }
    );
    
    jobs
}

pub fn find_job(page: i32, page_size: i32) -> DomainJob {
    DomainJob {
        id: page + page_size,
        company: DomainCompany {
            id: 1,
            name: "会社名".to_string(),
            logo: "http://".to_string(),
            thumbnail: "http://".to_string(),
        },
        title: "タイトル".to_string(),
        title_sub: "サブ詳細".to_string(),
        tag: "たぐ".to_string(),
        description: "詳細".to_string(),
        recruiment_period: "~ 2019/08/12".to_string(),
        location: "United States / San Francisco, CA".to_string(),
        salary: "$72,000/year".to_string(),
        type_of_work: "働き方".to_string(),
        employees: 12,
    }
}