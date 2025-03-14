use chumsky::{prelude::*, text::newline};

use core::fmt;

pub type Span = std::ops::Range<usize>;

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

pub fn parser() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
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
        .map(|text_| Token::Text(text_));

    // delimeter
    let delimeter = choice((
        newline()
            .repeated()
            .exactly(2)
            .to(Token::Delimeter),
        end().to(Token::Delimeter),
    ));

    // A token can be one of the following
    choice((index, timespan, text_, delimeter))
        .recover_with(skip_then_retry_until([]))
        .map_with_span(|t, span| (t, span))
        .repeated()
        .then_ignore(end())
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parse() {
        const SIMPLE_SRT: &str = "
        1
        00:05:00,400 --> 00:05:15,300
        This is an example of
        a subtitle.

        ";

        let expected = vec![
            Token::Index(32),
            Token::Timespan(Timespan {
                start: Timecode {
                    hours: 0,
                    minutes: 5,
                    seconds: 0,
                    milliseconds: 400,
                },
                end: Timecode {
                    hours: 0,
                    minutes: 5,
                    seconds: 0,
                    milliseconds: 400,
                },
            }),
            Token::Text("This is an example of\na subtitle.".to_string()),
            Token::Delimeter,
        ];

        let result = parser().parse(SIMPLE_SRT).unwrap();
        
        assert!(expected.iter()
            .zip(result.iter())
            .all(|(e, r)| *e == r.0));
    }
}
