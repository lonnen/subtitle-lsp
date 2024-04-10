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
    let tokens_unwrapped = tokens.unwrap();
    assert_eq!(tokens_unwrapped.len(), 0);
    for (token, _) in tokens_unwrapped {
        assert_eq!(token, Token::Text("stuff".to_string()));
        println!("{}", token);
    }
}
