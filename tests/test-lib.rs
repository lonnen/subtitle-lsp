use subtitle_lsp::*;

#[test]
fn test_lex() {
    assert_eq!(lexer().parse_recovery(), ());
}
