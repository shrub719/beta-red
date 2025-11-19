mod lexer {
    use crate::*;

    #[test]
    fn lex_true() {
        use crate::lexer::Token::*;

        let input = "\\x.\\y.x";
        let tokens = lexer::lex(input).unwrap();

        assert_eq!(tokens, vec![
            Lambda, 
            Identifier("x".to_string()), 
            Dot, 
            Lambda, 
            Identifier("y".to_string()), 
            Dot, 
            Identifier("x".to_string())
        ]);
    }

    #[test]
    fn lex_app() {
        use crate::lexer::Token::*;

        let input = "(\\x.x) _3";
        let tokens = lexer::lex(input).unwrap();

        assert_eq!(tokens, vec![
            LParen,
            Lambda,
            Identifier("x".to_string()),
            Dot,
            Identifier("x".to_string()),
            RParen,
            Identifier("_3".to_string())
        ]);
    }

    #[test]
    #[should_panic]
    fn lex_invalid() {
        let input = "\\x.x --";
        lexer::lex(input).unwrap();
    }
}

mod parser {
    use crate::*;
    use std::collections::VecDeque;

    #[test]
    fn parse_identifier() {
        use crate::lexer::Token::*;

        let tokens = VecDeque::from(vec![
            Identifier("x".to_string())
        ]);
        let expr = parser::parse(tokens).unwrap();
    }
}
