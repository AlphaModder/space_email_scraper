use space_email_api::SpaceEmail;
use chrono::NaiveDateTime;
use regex::Regex;

pub enum Filter {
    Or(Box<Filter>, Box<Filter>),
    And(Box<Filter>, Box<Filter>),
    Not(Box<Filter>),
    MatchSender(Regex),
    MatchSubject(Regex),
    MatchBody(Regex),
    BeforeTime(NaiveDateTime),
    AfterTime(NaiveDateTime),
    Empty,
}

impl Filter {
    fn matches(&self, email: &SpaceEmail) -> bool {
        match *self {
            Filter::Or(a, b) => a.matches(email) || b.matches(email),
            Filter::And(a, b) => a.matches(email) && b.matches(email),
            Filter::Not(f) => !f.matches(email),
            Filter::MatchSender(r) => r.is_match(&email.contents().sender),
            Filter::MatchSubject(r) => r.is_match(&email.contents().subject),
            Filter::MatchBody(r) => r.is_match(&email.contents().body),
            Filter::BeforeTime(t) => email.timestamp().le(&t),
            Filter::AfterTime(t) => email.timestamp().ge(&t),
            Filter::Empty => true
        }
    }
}

mod parser {
    use super::Filter;
    use nom::{
        IResult, branch::alt, combinator::map_res,
        bytes::complete::{
            tag, take_while, take_until
        }
    };
    
    fn parenthesized_regex(input: &str) -> IResult<&str, regex::Regex> {
        let (input, _) = tag("(")(input)?;
        let (input, open) = alt((tag("'"), tag("\"")))(input)?;
        let (input, hashes) = take_while(|c| c == '#')(input)?;
        let close = open.to_string() + hashes + ")";
        let (input, regex) = map_res(take_until(&*close), regex::Regex::new)(input)?;
        let (input, _) = tag(&*close)(input)?;
        Ok((input, regex))
    }

    fn sender(input: &str) -> IResult<&str, Filter> {
        let (input, _) = tag("sender")(input)?; 
        let (input, regex) = parenthesized_regex(input)?;
        Ok((input, Filter::MatchSender(regex)))
    }

    fn subject(input: &str) -> IResult<&str, Filter> {
        let (input, _) = tag("subject")(input)?; 
        let (input, regex) = parenthesized_regex(input)?;
        Ok((input, Filter::MatchSubject(regex)))
    }

    fn body(input: &str) -> IResult<&str, Filter> {
        let (input, _) = tag("body")(input)?; 
        let (input, regex) = parenthesized_regex(input)?;
        Ok((input, Filter::MatchBody(regex)))
    }

    fn date(input: &str) -> chrono::ParseResult<chrono::NaiveDateTime> {
        chrono::NaiveDateTime::parse_from_str(input, "%m/%d/%y %H:%M")
    }

    fn before(input: &str) -> IResult<&str, Filter> {

    }

    fn after(input: &str) -> IResult<&str, Filter> {
        
    }

    fn filter(input: &str) -> IResult<&str, Filter> {

    }
}

