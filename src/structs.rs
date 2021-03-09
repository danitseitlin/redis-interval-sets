#[derive(Debug, PartialEq)]

pub struct Set {
    pub member: String,
    pub min_score: i64,
    pub max_score: i64,
}

pub struct IntervalSet {
    pub sets: Vec<Set>,
}