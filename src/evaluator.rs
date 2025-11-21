use crate::parser::Term;

#[derive(Debug)]
pub struct Evaluator {
    expr: Term
}

impl Evaluator {
    pub fn new(expr: Term) -> Self {
        Self {
            expr
        }
    }

    fn is_value(term: Term) -> bool {
        matches!(term, Term::Abstraction)
    }

    pub fn evaluate(&mut self) -> Term {
        // realistically should not throw an error
        // unless called manually on something that wasn't returned by parse()
        loop {
            match Term {
                Term::Application { func, arg } => {
                    todo!()
                },
                Term::Identifier(name) => {
                    todo!()
                },
                _ => return self.expr
            }
        }
    }
}
