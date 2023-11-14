use std::time::Duration;
use anyhow::{anyhow, Result as AnyResult};
use nom::branch::alt;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, digit0, digit1};
use nom::IResult;
use nom::multi::{many1, many_till, separated_list0};
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Detail {
    text: String
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Break {
    name: String,
    hours: u64,
    minutes: u64,
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

impl Break {
    fn new(name: &str, time: (&str, &str)) -> Self {
        Self {
            name: name.to_string(),
            hours: match time.0.parse::<u64>() {
                Ok(num) => num,
                Err(e) => 0,
            },
            minutes: match time.1.parse::<u64>() {
                Ok(num) => num,
                Err(e) => 0,
            }
        }
    }

    fn get_duration(&self) -> Duration {
        let in_seconds = (&self.minutes * 60) + (&self.hours * 60 * 60);
        Duration::from_secs(in_seconds)
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
    let input_str = input.as_str();

    let res: IResult<&str, (&str, &str)> =
        separated_pair(alpha1, tag(": "), alpha1)(input_str);

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

pub fn parse_break(input: String) -> Option<Vec<Break>> {
    let input_str = input.as_str();

    let res: IResult<&str, (&str, (&str, &str))> =
        separated_pair(alpha1, tag(": "), separated_pair(digit0, tag(":"), digit0))(input_str);

    match res {
        Ok(parsed) => {
            let mut res = Vec::new();
            let rating = Break::new(parsed.1.0, parsed.1.1);
            res.push(rating);

            return Some(res)
        },
        Err(e) => None
    }
}

/*
pub fn parse_log(input: String) -> Option<Log> {
    let input_str = input.as_str();
    let res = separated_list0(
        tag("\n"),
        alt((
            tuple(
                (digit0, tag(":"), digit0)
            ),
            many_till(
                alphanumeric1,
                tuple((digit0, tag(":"), digit0))
            ),
            separated_list0(tag("; "), alphanumeric1)
        ))
    );

    res
}
*/

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_rating() {
        let mut expected_rating = Vec::new();
        expected_rating.push(Rating { question: "mood".to_string(), answer: "good".to_string() });
        let expected = Some(expected_rating);

        assert_eq!(
            expected,
            parse_rating("mood: good".to_string())
        );
    }

    #[test]
    fn test_breaks() {
        let mut expected_break_list = Vec::new();
        let expected_break = Break { name: "lunch".to_string(), hours: 1, minutes: 15 };
        expected_break_list.push(expected_break.clone());
        let expected = Some(expected_break_list);

        assert_eq!(
            expected,
            parse_break("lunch: 01:15".to_string())
        );

        assert_eq!(
            expected_break.get_duration(),
            Duration::from_secs(60*60+60*15)
        )
    }
}