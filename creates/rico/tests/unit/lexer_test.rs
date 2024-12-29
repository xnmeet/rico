use logos::Logos;
use rico::lexer::Token;

#[test]
fn test_lexer() {
    let input = r#"
        namespace rs demo
        
        struct User {
            1: string name
        }
    "#;

    let mut lexer = Token::lexer(input);

    assert_eq!(lexer.next(), Some(Ok(Token::Namespace)));
    assert_eq!(lexer.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lexer.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lexer.next(), Some(Ok(Token::Struct)));
    assert_eq!(lexer.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lexer.next(), Some(Ok(Token::LeftBrace)));
    assert_eq!(lexer.next(), Some(Ok(Token::IntegerLiteral)));
    assert_eq!(lexer.next(), Some(Ok(Token::Colon)));
    assert_eq!(lexer.next(), Some(Ok(Token::String)));
    assert_eq!(lexer.next(), Some(Ok(Token::Identifier)));
    assert_eq!(lexer.next(), Some(Ok(Token::RightBrace)));
}
