use core::fmt;

use chumsky::prelude::*;

pub type Span = std::ops::Range<usize>;

#[derive(Debug)]
pub struct Timespan {
    start: Timecode,
    end: Timecode
}

#[derive(Debug)]
pub struct Timecode {
    hours: u8,
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

#[derive(Debug)]
pub enum Token {
    Index(usize),
    Timespan(Timespan),
    Timecode(Timecode),
    Text(String)
}

impl fmt::Display for Timecode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{}.{}.{},{}", self.hours, self.minutes, self.seconds, self.milliseconds)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Index(i) => write!(f, "{}", i),
            Token::Timespan(t) => write!(f, "{} --> {}", t.start, t.end),
            Token::Timecode(t) => write!(f, "{}", t),
            Token::Text(s) => write!(f, "{}", s),
        }
    }
}

fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    // parser for indexes
    // parser for timecodes
    // parser for timespans
    // parser for text
    
    let token = index
        .or(timespan)
        .or(text_)
        .recover_with(skip_then_retry_until([]));

    token
        .map_with_span(|tok, span|(tok, span))
        .padded()
        .repeated()
}