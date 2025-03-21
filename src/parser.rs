use core::fmt;

use nom::{
    branch::alt,
    character::char,
    character::complete::{line_ending, usize},
    multi::separated_list0,
    IResult, Parser,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Timespan {
    start: Timecode,
    end: Timecode,
}

impl fmt::Display for Timespan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} --> {}", self.start, self.end)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Timecode {
    hours: usize,
    minutes: usize,
    seconds: usize,
    milliseconds: usize,
}

impl fmt::Display for Timecode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{},{}",
            self.hours, self.minutes, self.seconds, self.milliseconds
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Index(u32),
    Timespan(Timespan),
    Text(String),
    Delimeter,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Index(i) => write!(f, "{}", i),
            Token::Timespan(t) => write!(f, "{} --> {}", t.start, t.end),
            Token::Text(s) => write!(f, "{}", s),
            Token::Delimeter => write!(f, "\n"),
        }
    }
}

fn timespan(input: &str) -> IResult<&str, Timecode> {
    let (input, (hours, _, minutes, _, seconds, _, milliseconds)) =
        (usize, char(':'), usize, char(':'), usize, char(':'), usize)
            .parse(input)
            .unwrap();

    Ok((
        input,
        Timecode {
            hours,
            minutes,
            seconds,
            milliseconds,
        },
    ))
}

fn lines(input: &str) -> IResult<&str, &str> {
    separated_list0(line_ending, any)
}

fn index(input: &str) -> IResult<&str, usize> {
    usize(input)
}

fn token(input: &str) -> Token {
    alt((timespan, index, text, delimeter))
}
