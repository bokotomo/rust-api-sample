use super::super::domain::{
    job::DomainJob,
    company::DomainCompany,
};

pub struct RepositoryJob {}

impl RepositoryJob {
    pub fn new() -> RepositoryJob {
        RepositoryJob {}
    }

    // 仕事一覧を返す
    pub fn find_jobs(&self, page: i32, page_size: i32) -> Vec<DomainJob> {
        let mut jobs = Vec::new();
        for i in 0..5 {
            let company = DomainCompany::new(
                i,
                format!("{} - {}", "株式会社feroiav Games".to_string(), i),
                "http://localhost:3000/images/logo1.jpg".to_string(),
                "http://localhost:3000/images/company_back1.jpg".to_string(),
            );
            jobs.push(
                DomainJob::new_jobs(
                    page + page_size,
                    company,
                    "We want 3D Designers who want to create a new 3D Game".to_string(),
                    "We are making new 3D games. If you curious about it, plese contact with us.".to_string(),
                    "たぐ".to_string(),
                )
            );
        }

        jobs
    }

    // 仕事詳細を返す
    pub fn find_job(&self, job_id: i32) -> DomainJob {
        let company = DomainCompany::new(
            1,
            "株式会社feroiav Games".to_string(),
                "http://localhost:3000/images/logo1.jpg".to_string(),
                "http://localhost:3000/images/company_back1.jpg".to_string(),
        );
        DomainJob::new_job(
            job_id,
            company,
            "We want 3D Designers who want to create a new 3D Game".to_string(),
            "We are making new 3D games. If you curious about it, plese contact with us.".to_string(),
            "たぐ".to_string(),
            "Bulbasaur (Japanese: フシギダネ Fushigidane) is a dual-type Grass/Poison Pokémon introduced in Generation I.It evolves into Ivysaur starting at level 16, which evolves into Venusaur starting at level 32.Along with Charmander and Squirtle, Bulbasaur is one of three starter Pokémon of Kanto available at the beginning of Pokémon Red, Green, Blue, FireRed, and LeafGreen.".to_string(),
            "~ 2019/08/12".to_string(),
            "United States / San Francisco, CA".to_string(),
            "$72,000/year".to_string(),
            "働き方リモート".to_string(),
            12,
        )
    }
}