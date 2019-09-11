use super::super::super::repository::design::{find_designs};

#[test]
fn test_find_designs() {
    let domain_designs = find_designs(1, 1);
    assert_eq!(1, domain_designs[0].id);
}