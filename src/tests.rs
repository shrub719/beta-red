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
    fn parse_id() {
        use crate::lexer::Token::*;
        use crate::parser::Term;

        let tokens = VecDeque::from(vec![
            Identifier("x".to_string())
        ]);
        let expr = parser::parse(tokens).unwrap();

        assert_eq!(expr, Term::id("x"));
    }

    #[test]
    fn parse_true() {
        use crate::lexer::Token::*;
        use crate::parser::Term;

        let tokens = VecDeque::from(vec![
            LParen,

            Lambda,
            Identifier("x".to_string()),
            Dot,
            Lambda,
            Identifier("y".to_string()),
            Dot,
            Identifier("x".to_string()),

            RParen,

            Identifier("a".to_string()),
            Identifier("b".to_string())
        ]);
        let expr = parser::parse(tokens).unwrap();

        assert_eq!(expr, Term::app(
            Term::app(
                Term::abs(
                    "x",
                    Term::abs(
                        "y",
                        Term::id("x")
                    )
                ),
                Term::id("a")
            ),
            Term::id("b")
        ));
    }

    #[test]
    #[should_panic]
    fn parse_invalid_trailing_characters() {
        use crate::lexer::Token::*;

        let tokens = VecDeque::from(vec![
            Identifier("x".to_string()),
            Dot,
            Dot,
            Identifier("ehjgfkhldsfg".to_string())
        ]);
        
        parser::parse(tokens).unwrap();
    }
}
