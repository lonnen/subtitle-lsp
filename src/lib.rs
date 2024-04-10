use core::fmt;

use chumsky::{prelude::*, text::newline};

pub type Span = std::ops::Range<usize>;

#[derive(Debug)]
pub struct Timespan {
    start: Timecode,
    end: Timecode,
}

#[derive(Debug)]
pub struct Timecode {
    hours: u8,
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Index(String),
    // Timespan(Timespan),
    // Timecode(Timecode),
    Text(String),
    Delimeter,
    Card(String),
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Index(i) => write!(f, "{}", i),
            // Token::Timespan(t) => write!(f, "{} --> {}", t.start, t.end),
            // Token::Timecode(t) => write!(f, "{}", t),
            Token::Text(s) => write!(f, "{}", s),
            Token::Delimeter => write!(f, "\n"),
            Token::Card(s) => write!(f, "{}", s),
        }
    }
}

pub fn parser() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    // let delimeter = text::newline()
    //     .then(text::newline())
    //     .map(|_| Token::Delimeter);

    // // A parser for indexes
    // let index = text::int(10)
    //     .chain::<char, _, _>(just(',').chain(text::digits(10)).or_not().flatten())
    //     .collect::<String>()
    //     .map(Token::Index);

    // // A parser for timespans
    // // let timespan = ;

    // // parser for timecodes
    // // let timecode = ;

    // // parser for text
    // let text_ = just('"')
    //     .ignore_then(filter(|c| *c != '"').repeated())
    //     .then_ignore(just('"'))
    //     .collect::<String>()
    //     .map(Token::Text);

    // // parser for delimeter
    // let delimeter = newline().repeated()

    // let token = index
    //     .or(timespan)
    //     .or(text_)
    //     .or(delimeter)
    //     .recover_with(skip_then_retry_until([]));

    // let token = delimeter
    //     .or(index)
    //     .recover_with(skip_then_retry_until([]));

    let token = any()
        .repeated()
        .then_ignore(newline())
        .collect::<String>()
        .map(|characters: String| match characters.as_str() {
            _ => Token::Text(characters),
        });

    token
        .map_with_span(|tok, span| (tok, span))
        .padded()
        .repeated()
}
