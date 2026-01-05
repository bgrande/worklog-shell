use std::time::Duration;
use anyhow::{anyhow, Result as AnyResult};
use nom::branch::alt;

use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, alphanumeric1, digit0, digit1};
use nom::combinator::not;
use nom::IResult;
use nom::multi::{many0, many1, many_till, separated_list0};
use nom::sequence::{delimited, pair, separated_pair, terminated, tuple};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Learning {
    text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Impediment {
    text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct Log {
    time_start: String, // todo chrono datetime
    time_end: String, // todo chrono datetime
    summaries: Vec<Summary>,
    details: Vec<Detail>,
}

/*
impl Log {
    fn new(summaries: &str, time: (&str, &str)) -> Self {
        Self {
            summaries: summaries,
            details: details,
            time_start: match time.0.parse::<u64>() {
                Ok(num) => num,
                Err(e) => 0,
            },
            time_end: match time.1.parse::<u64>() {
                Ok(num) => num,
                Err(e) => 0,
            }
        }
    }
}
*/
#[derive(Clone, Debug, PartialEq, Eq)]
struct WorkLog {
    date_time_start: String, // todo chrono datetime
    date_time_end: String, // todo chrono datetime
    date_work_hours: String,
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
        separated_pair(
            alpha1,
            tag(": "),
            separated_pair(digit0, tag(":"), digit0)
        )(input_str);

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

pub fn parse_log(input: String) -> Option<WorkLog> {
    Some(WorkLog {
        date_work_hours: "foo".to_string(),
        date_time_start: "foo".to_string(),
        logs: Vec::new(),
        date_time_end: "too".to_string()
    })
}

/*
pub fn parse_log(input: String) -> Option<WorkLog> {
    let input_str = input.as_str();
    let res = separated_list0(
        (
            tuple(
                (digit0, tag(":"), digit0)
            ),
            many0(
                alphanumeric1/*,
                tuple((digit0, tag(":"), digit0))*/
            ),
            separated_list0(tag("; "), alphanumeric1),
        ),
        ()
    )(input_str);

    match res {
        Ok(parsed) => {
            println!("{:?}", parsed);
            let rating = WorkLog::new(parsed.1.0, parsed.1.1);
            Some(rating)
            //return Some("".to_string())
        },
        Err(e) => None
    }
}

/// todo adjust the following for parse_log:
fn parse(input: &str) -> IResult<&str, Vec<WorkLog>> {
    separated_list0(
        separator,
        entry,
    )(input)
}

fn entry(input: &str) -> IResult<&str, WorkLog> {
    let (input, title) = title(input)?;
    let (input, body_lines) = many1(body_line(title))(input)?;

    let body = body_lines.join("");
    let entry = WorkLog {
        date_work_hours: "".to_string(),
        date_time_start: "".to_string(),
        date_time_end: "".to_string(),
        logs: vec![]

    };

    //TODO: Does it have to end with a separator ?
    // If it does, either use terminated() in combination with many(), or add
    // an additional check for separator here


    IResult::Ok((input, entry))
}
*/
fn title(input: &str) -> IResult<&str, &str> {
    terminated(
        take_while(not_r_n),
        end_of_line,
    )(input)
}


pub fn time_parse(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(digit0, tag(":"), digit0)(input)
}

pub fn body_line<'i>(title: &'i str, time: &'i str) -> impl FnMut(&'i str) -> IResult<&'i str, &'i str, nom::error::Error<&'i str>>
{
    move |input: &str| {
        delimited(
            pair(tag(title), tag(" ")),
            take_while(not_r_n),
            end_of_line,
        )(input)
    }
}

fn separator(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("sep/"), // the separator is hardcoded, otherwise you have to do the same monstrosity as body_line() above
        end_of_line,
    )(input)
}

fn end_of_line(input: &str) -> IResult<&str, &str> {
    alt((
        tag("\n"),
        tag("\r\n")
    ))(input)
}

fn not_r_n(ch: char) -> bool {
    ch != '\r' && ch != '\n'
}
/// todo end

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_time_parse() {
        let to_parse = r#"09:00"#;

        println!("{:?}", time_parse(to_parse));

        assert_eq!(
            Ok(("", ("09", "00"))),
            time_parse(to_parse)
        );
    }

    #[test]
    fn test_parse_log() {
        let to_parse = r#"
08:00
reading emails; planning meeting; finalizing project X
JIRA-1: project X final steps were creating the app, committing the changes and updating dependencies.
Last step was deploying the app
12:00
12:30
creating new project Y
setting up the project in git, creating project bootstrap
17:30
        "#;

        let mut expected_logs = Vec::new();

        let expected_worklog = WorkLog {
            date_time_start: "08:00".to_string(),
            date_time_end: "17:30".to_string(),
            date_work_hours: "8.5".to_string(),
            logs: expected_logs
        };
        let expected = Some(expected_worklog);

        assert_eq!(
            expected,
            parse_log(to_parse.to_string())
        );
    }

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