use std::collections::HashSet;
use std::cell::RefCell;
use crate::{
    parser::Term
};

thread_local!(static DISAMBIGUATE_CTR: RefCell<u64> = RefCell::new(0));

fn disambiguate(w: &str) -> String {
    DISAMBIGUATE_CTR.with(|c| {
        let mut ctr = c.borrow_mut();
        *ctr += 1;
        format!("{}_{}", w, ctr)
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
    let new_var = disambiguate(var);
    let new_body = sub(body, var, &Term::Var(new_var.clone()));
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

pub fn reduce(expr: Term) -> Term {
    match expr {
        Term::App(left, right) => {
            let left = reduce(*left);
            match left {
                Term::Abs(_, _) => reduce(beta_red(left, *right)),
                _ => Term::App(Box::new(left), Box::new(reduce(*right)))
            }
        },
        _ => expr
    }
}
