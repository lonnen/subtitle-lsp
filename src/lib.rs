use core::fmt;

use chumsky::prelude::*;

#[derive(Debug)]
pub enum Token {
    Index(usize),
    Timespan(String),
    Timecode(String),
    Text(String)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Index(i) => write!(f, "{}", i),
            Token::Timespan(t) => write!(f, "{}", t),
            Token::Timecode(t) => write!(f, "{}", t),
            Token::Text(s) => write!(f, "{}", s),
        }
    }
}