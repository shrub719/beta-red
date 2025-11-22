use std::collections::HashMap;
use crate::parser::Term;

fn is_value(term: Term) -> bool {
    matches!(term, Term::Abstraction { param: _, body: _ })
}

pub fn eval(expr: Term, context: HashMap<String, Term>) -> Term {
    // realistically should not throw an error
    // unless called manually on something that wasn't returned by parse()
    loop {
        match expr {
            Term::Application { func, arg } => {
                match *func {
                    Term::Abstraction { param, body } => {
                        match *arg {
                            Term::Abstraction { param: _, body: _ } => {
                                context.insert(param, *arg);
                                expr = eval(*body, context);
                            },
                            eval_term => {
                                *arg = eval(*arg, context.clone());
                            }
                        };
                    },
                    eval_term => {
                        *func = eval(*func, context);
                    }
                };
            },
            Term::Identifier(name) => {
                todo!()
            },
            _ => return expr
        };
    }
}

pub fn evaluate(expr: Term) -> Term {
    let context = HashMap::new();
    eval(expr, context)
}
