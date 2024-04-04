use subtitle_lsp::*;

#[test]
fn test_lex() {
    assert_eq!(parser().parse_recovery(), ());
}
