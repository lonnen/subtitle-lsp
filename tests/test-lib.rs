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
    let (tokens, _) = parser().parse_recovery(SIMPLE_SRT);
    for (token, _) in tokens.unwrap() {
        assert_eq!(token, Token::Text("".to_string()));
    }
}
