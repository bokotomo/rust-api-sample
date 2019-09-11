use super::super::super::repository::job::{find_job};

#[test]
fn test_find_job() {
    let domain_job = find_job(1, 1);
    assert_eq!(2, domain_job.id);
}