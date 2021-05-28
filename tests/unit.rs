use structs::{Set, IntervalSet};
use lib::{get_sets, get_members};

#[test]
fn get_sets_empty() {
    let args = vec![];
    let sets = get_sets(args.into_iter());
    let sets = sets.expect("no sets");
    assert_eq!(sets, vec![]);
}

#[test]
fn get_sets_partial1() {
    let args = vec!["member1".to_string()];
    let sets = get_sets(args.into_iter());
    match sets.expect_err("should fail on partial arguments") {
        RedisError::WrongArity => {}
        _ => panic!("wrong error"),
    }
}

#[test]
fn get_sets_partial2() {
    let args = vec!["member1".to_string(), "10".to_string()];
    let sets = get_sets(args.into_iter());
    match sets.expect_err("should fail on partial arguments") {
        RedisError::WrongArity => {}
        _ => panic!("wrong error"),
    }
}

#[test]
fn get_sets_single() {
    let args = vec!["member1".to_string(), "10".to_string(), "20".to_string()];
    let sets = get_sets(args.into_iter());
    let sets = sets.expect("one member");
    assert_eq!(
        sets,
        vec![Set {
            member: "member1".to_string(),
            min_score: 10,
            max_score: 20,
        }]
    );
}

#[test]
fn get_sets_multi() {
    let args = vec![
        "member1".to_string(),
        "10".to_string(),
        "20".to_string(),
        "member2".to_string(),
        "30".to_string(),
        "40".to_string(),
    ];
    let sets = get_sets(args.into_iter());
    let sets = sets.expect("multiple members");
    assert_eq!(
        sets,
        vec![
            Set {
                member: "member1".to_string(),
                min_score: 10,
                max_score: 20,
            },
            Set {
                member: "member2".to_string(),
                min_score: 30,
                max_score: 40,
            }
        ]
    );
}

#[test]
fn get_members_empty() {
    let args = vec![];
    let members = get_members(args.into_iter());
    let members = members.expect("no members");
    let empty_list: Vec<String> = vec![];
    assert_eq!(members, empty_list);
}

#[test]
fn get_members_single() {
    let args = vec!["member1".to_string()];
    let members = get_members(args.into_iter());
    let members = members.expect("one member");
    assert_eq!(
        members,
        vec!["member1"]
    );
}

#[test]
fn get_members_multi() {
    let args = vec![
        "member1".to_string(),
        "member2".to_string(),
    ];
    let members = get_members(args.into_iter());
    let members = members.expect("multiple members");
    assert_eq!(
        members,
        vec!["member1".to_string(), "member2".to_string()]
    );
}