use std::str::Chars;

pub enum Token {
    Lambda(usize),
    LParen(usize),
    RParen(usize),
    Identifier(usize, String)
}

pub fn lex(input: &mut Chars) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut pos: usize = 0;
    let mut running_id = String::new();

    while let Some(ch) = input.next() {
        pos += 1;
        let mut next_token = None;

        match ch {
            'λ' | '\\' => next_token = Some(Token::Lambda(pos)),
            '(' => next_token = Some(Token::LParen(pos)),
            ')' => next_token = Some(Token::RParen(pos)),
            c if c.is_alphanumeric() || c == '_' => {
                running_id.push(c);
                continue;
            },
            c if c.is_whitespace() || c == '.' => (),
            _ => panic!("invalid character")
        };

        if running_id.len() > 0 {
            tokens.push(Token::Identifier(pos, running_id.clone()));
            running_id.clear();
        }

        match next_token {
            Some(token) => tokens.push(token),
            _ => ()
        };
    }

    tokens
}
