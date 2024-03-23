use core::fmt;

use chumsky::prelude::*;

#[derive(Debug)]
pub enum Token {
    Index(usize),
    TimeSpan,
    Timecode,
    Text(String)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Index(i) => write!(f, "{}", i),
            Token::TimeSpan => write!(f, "{}"),
            Token::Timecode => write!(f, "{}"),
            Token::Text(s) => write!(f, "{}", s),

        }
    }
}