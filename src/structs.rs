use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
#[derive(Debug, PartialEq)]

pub struct Set {
    pub member: String,
    pub min_score: i64,
    pub max_score: i64,
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = "{'member':'".to_string() + &self.member.to_string() + &"','min_score':".to_string() + &self.min_score.to_string() + &",'max_score':".to_string() + &self.max_score.to_string() + "}";
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
        let sets: String = self.sets.to_string().chars().take(self.sets.to_string().len()-1).collect();
        println!("parsing sets: [{}]", sets);
        write!(f, "[{}]", sets)
    }
}

impl FromStr for IntervalSet {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{}", s);
        let mut sets: Sets = Sets(vec![]);
        if s != "[]" {
            //let sets: Sets = Sets(vec![]);
            let mut sets_in_string: Vec<String> = vec![];
            //Going over the string and spliting it to a Vec of js objects as string.
            let mut found_json = false;
            let mut current_set_in_string: Vec<char> = vec![];// = "".to_string();
            for i in s.chars() {
                let c = i;
                if i == '{' && !found_json {
                    current_set_in_string.push(c);
                    found_json = true;
                }
                else if i == '}' && found_json {
                    current_set_in_string.push(i);
                    found_json = false;
                    sets_in_string.push(current_set_in_string.iter().collect());
                    current_set_in_string = vec![];
                }
                else {
                    current_set_in_string.push(i);
                }
            }
            //println!("current_set_in_string: {}", sets_in_string.iter().collect());
            //Goining over the Vec of json objects as strings, creating Sets;
            for i in sets_in_string {

                //
                let mut member: String = "".to_string();
                let mut min_score: i64 = 0;
                let mut max_score: i64 = 1;
                let split_key: Vec<&str> = i.trim_matches(| p | p == '{' || p == '}').split(',').collect();
                for kv in split_key {
                    let sp: Vec<&str> = kv.split(':').collect();
                    if sp[0].contains("member")  { member = sp[1].to_string().replace("'", ""); }
                    else if sp[0].contains("min_score")  { min_score = sp[1].parse::<i64>().unwrap(); }
                    else if sp[0].contains("max_score")  { max_score = sp[1].parse::<i64>().unwrap(); }
                }
                println!("for {} found: member => {}, min => {}, max => {}", i, member, min_score, max_score);
                sets.0.push(Set { member, min_score, max_score });
            }
        }
        Ok(IntervalSet { sets })
    }
}