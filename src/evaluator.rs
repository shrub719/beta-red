use std::collections::HashSet;
use std::cell::RefCell;
use crate::{
    parser::Term,
    parser::parse,
    lexer::lex
};

thread_local!(static DISAMBIGUATE_CTR: RefCell<u64> = RefCell::new(0));

fn disambiguate(w: &str) -> String {
    DISAMBIGUATE_CTR.with(|c| {
        let mut ctr = c.borrow_mut();
        *ctr += 1;
        format!("{}_{}", w, ctr)
    })
}

pub fn reset_disambiguation() {
    DISAMBIGUATE_CTR.with(|c| {
        let mut ctr = c.borrow_mut();
        *ctr = 0;
    })
}

fn upd_free_vars(expr: &Term, free_vars: &mut HashSet<String>, bound_vars: &mut HashSet<String>) {
    match expr {
        Term::Var(name) => {
            if !bound_vars.contains(name) {
                free_vars.insert(name.to_string());
            }
        },
        Term::Abs(name, body) => {
            let already_there = bound_vars.insert(name.to_string());
            upd_free_vars(body, free_vars, bound_vars);
            if already_there { bound_vars.remove(name); }
        },
        Term::App(left, right) => {
            upd_free_vars(left, free_vars, bound_vars);
            upd_free_vars(right, free_vars, bound_vars);
        }
    }
}

fn get_free_vars(expr: &Term) -> HashSet<String> {
    let mut free_vars = HashSet::new();
    let mut bound_vars = HashSet::new();

    upd_free_vars(expr, &mut free_vars, &mut bound_vars);
    free_vars
}

fn alpha_convert(var: &str, body: Term) -> (String, Term) {
    let new_var = disambiguate(var); let new_body = sub(body, var, &Term::Var(new_var.clone()));
    (new_var, new_body)
}

fn beta_red(abs: Term, val: Term) -> Term {
    match abs {
        Term::Abs(var, body) => {
            sub(*body, &var, &val)
        },
        _ => panic!("can't beta reduce non-abstraction")
    }
}

fn sub(root: Term, var: &str, val: &Term) -> Term {
    match root {
        Term::Var(curr_var) => {
            if curr_var == var {
                val.clone()
            } else {
                Term::Var(curr_var)
            }
        },
        Term::App(left, right) => {
            Term::App(
                Box::new(sub(*left, var, val)), Box::new(sub(*right, var, val))
            )
        },
        Term::Abs(curr_var, body) => {
            if curr_var == var {
                Term::Abs(curr_var, body)
            } else if get_free_vars(val).contains(&curr_var) {
                let (new_var, new_body) = alpha_convert(&curr_var, *body);
                Term::Abs(new_var, Box::new(sub(new_body, var, val)))
            } else {
                Term::Abs(curr_var, Box::new(sub(*body, var, val)))
            }
        }
    }
}

fn church_num_inner(n: usize) -> Term {
    match n {
        0 => Term::Var("x".to_string()),
        _ => Term::App(
            Box::new(Term::Var("f".to_string())),
            Box::new(church_num_inner(n-1))
        )
    }
}

fn church_num(n: usize) -> Term {
    Term::Abs(
        "f".to_string(),
        Box::new(Term::Abs(
            "x".to_string(),
            Box::new(church_num_inner(n))
        ))
    )
}

fn quick_parse(expr: &str) -> Option<Term> {
    Some(parse(&lex(&mut expr.chars()).unwrap()).unwrap())
}

fn get_builtin(name: &String) -> Option<Term> {
    match name.as_str() {
        "pred" => quick_parse("\\n.\\f.\\x. n (\\g.\\h.h (g f)) (\\u.x) (\\u.u)"),
        "plus" => quick_parse("\\m.\\n.\\f.\\x.m f (n f x) "),
        "multiply" => quick_parse("\\m.\\n.\\f.\\x.m (n f) x "),
        "if" => quick_parse("\\c.\\t.\\f.c t f"),
        "true" => quick_parse("\\t.\\f.t "),
        "false" => quick_parse("\\t.\\f.f "),
        _ => None
    }
}

pub fn reduce(expr: Term) -> Term {
    match expr {
        Term::App(left, right) => {
            let left = reduce(*left);
            match left {
                Term::Abs(_, _) => reduce(beta_red(left, *right)),
                _ => Term::App(Box::new(left), Box::new(reduce(*right)))
            }
        },
        Term::Var(name) => {
            if let Some(builtin) = get_builtin(&name) {
                builtin.clone()
            } else if name.chars().all(char::is_numeric) {
                church_num(name.parse().unwrap())
            } else {
                Term::Var(name)
            }
        },
        Term::Abs(func, arg) => Term::Abs(func, Box::new(reduce(*arg))),
    }
}

pub fn evaluate(expr: Term) -> Term {
    reset_disambiguation();     // TODO: can do this better
    reduce(expr)
}
