use std::fmt;
#[derive(Debug, PartialEq)]

pub struct Set {
    pub member: String,
    pub min_score: i64,
    pub max_score: i64,
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = "{ member: ".to_string() + &self.member.to_string() + &", min_score: ".to_string() + &self.min_score.to_string() + &", max_score: ".to_string() + &self.max_score.to_string() + " }";
        println!("parsing Set: {}", out);
        write!(f, "{}", out)
    }
}

pub struct Sets(pub Vec<Set>);

impl fmt::Display for Sets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, set| {
            result.and_then(|_| write!(f, "{},", set))
        })
        
    }
}

pub struct IntervalSet {
    pub sets: Sets,
}

impl fmt::Display for IntervalSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("parsing sets: [{}]", self.sets.to_string());
        write!(f, "[{}]", self.sets.to_string())
    }
}