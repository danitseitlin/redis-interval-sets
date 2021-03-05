pub use crate::lib::{get_members, Set}; 

#[test]
fn test_get_members_empty() {
    let args = vec![];
    let members = get_members(args.into_iter());
    let members = members.expect("no members");
    assert_eq!(members, vec![]);
}
/*
#[test]
fn test_get_members_partial1() {
    let args = vec!["member1".to_string()];
    let members = get_members(args.into_iter());
    match members.expect_err("should fail on partial arguments") {
        RedisError::WrongArity => {}
        _ => panic!("wrong error"),
    }
}

#[test]
fn test_get_members_partial2() {
    let args = vec!["member1".to_string(), "10".to_string()];
    let members = get_members(args.into_iter());
    match members.expect_err("should fail on partial arguments") {
        RedisError::WrongArity => {}
        _ => panic!("wrong error"),
    }
}*/

#[test]
fn test_get_members_single() {
    let args = vec!["member1".to_string()];
    let members = get_members(args.into_iter());
    let members = members.expect("one member");
    assert_eq!(
        members,
        vec!["member1"]
    );
}

#[test]
fn test_get_members_multi() {
    let args = vec![
        "member1".to_string(),
        "member2".to_string(),
    ];
    let members = get_members(args.into_iter());
    let members = members.expect("multiple members");
    assert_eq!(
        members,
        vec![
            member: "member1".to_string(),
            member: "member2".to_string()
        ]
    );
}