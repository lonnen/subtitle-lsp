use chumsky::{prelude::*, text::newline};

use core::fmt;

pub type Span = std::ops::Range<usize>;

#[derive(Debug, PartialEq)]
pub struct Timespan {
    start: Timecode,
    end: Timecode,
}

impl fmt::Display for Timespan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} --> {}", self.start, self.end)
    }
}

#[derive(Debug, PartialEq)]
pub struct Timecode {
    hours: u32,
    minutes: u32,
    seconds: u32,
    milliseconds: u32,
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

#[derive(Debug, PartialEq)]
pub struct Card {
    index: u32,
    timespan: Timespan,
    text: String,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}\n{}\n\n", self.index, self.timespan, self.text)
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Index(u32),
    Timespan(Timespan),
    Text(String),
    Delimeter,
    Card(Card),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Index(i) => write!(f, "{}", i),
            Token::Timespan(t) => write!(f, "{} --> {}", t.start, t.end),
            Token::Text(s) => write!(f, "{}", s),
            Token::Delimeter => write!(f, "\n"),
            Token::Card(s) => write!(f, "{}", s),
        }
    }
}

pub fn parser() -> impl Parser<char, Vec<Card>, Error = Simple<char>> {
    // helper parser for the integer portion of timecodes
    let unpadded_int = text::int(10).map(|s: String| s.parse::<u32>().unwrap());

    // A parser for indexes
    let index = unpadded_int
        .then_ignore(just("\n"))
        .map(|id| Token::Index(id));

    // parser for timecodes
    let timecode = unpadded_int
        .then_ignore(just(':'))
        .then(unpadded_int)
        .then_ignore(just(':'))
        .then(unpadded_int)
        .then_ignore(just(','))
        .then(unpadded_int)
        .map(|(((hours, minutes), seconds), milliseconds)| Timecode {
            hours,
            minutes,
            seconds,
            milliseconds,
        });

    // A parser for timespans
    let timespan = timecode
        .then_ignore(just("-->").padded())
        .then(timecode)
        .map(|(start, end)| Token::Timespan(Timespan { start, end }));

    // parser for text
    let text_ = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::Text);

    // delimeter
    let delimeter = choice((
        newline()
            .repeated()
            .exactly(2)
            .map(|_| return Token::Delimeter),
        end().map(|_| return Token::Delimeter),
    ));

    let card = index.then(timespan).then(text_).then(delimeter);

    card.repeated()
}
