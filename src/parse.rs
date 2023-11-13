use anyhow::{anyhow, Result as AnyResult};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::error::context;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{delimited, separated_pair, terminated, tuple};

struct Learning {
    text: String,
}

struct Impediment {
    text: String,
}

struct Summary {
    text: String,
}

struct Detail {
    text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rating {
    question: String,
    answer: String,
}

impl Rating {
    fn new(output: (&str, &str)) -> Self {
        Self {
            question: output.0.to_string(),
            answer: output.1.to_string()
        }
    }
}

struct Log {
    time_start: String, // todo chrono datetime
    time_end: String, // todo chrono datetime
    summaries: Vec<Summary>,
    details: Vec<Detail>,
}

struct WorkLog {
    date_time_start: String, // todo chrono datetime
    date_time_end: String, // todo chrono datetime
    date_work_hours: i8,
    logs: Vec<Log>,
}

pub fn parse_rating(input: String) -> Option<Vec<Rating>> {
    /*let context = context(input.as_str(), alt((
        tuple((many1(terminated(alphanumeric1, tag(": "))), alpha1)),
    )));*/

    let input_str = input.as_str();
    let res: IResult<&str, (&str, &str)> = separated_pair(alpha1, tag(": "), alpha1)(input_str);

    match res {
        Ok(parsed) => {
            let mut res = Vec::new();
            let rating = Rating::new(parsed.1);
            res.push(rating);
            
            return Some(res)
        },
        Err(e) => None
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let mut expected_rating = Vec::new();
        expected_rating.push(Rating { question: "mood".to_string(), answer: "good".to_string() });
        let expected = Some(expected_rating);
        
        assert_eq!(
            expected,
            parse_rating("mood: good".to_string())
        );
    }
}