use chumsky::Parser;
use subtitle_lsp::*;

const SIMPLE_SRT: &str = "
1
00:05:00,400 --> 00:05:15,300
This is an example of
a subtitle.
";

#[test]
fn test_lex() {
    let (tokens, mut errs) = parser().parse_recovery(SIMPLE_SRT);
}
