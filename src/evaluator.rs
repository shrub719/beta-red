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
    let new_body = substitute(body, var, &Term::Var(new_var.clone()));
    (new_var, new_body)
}

fn substitute(root: Term, var: &str, val: &Term) -> Term {
    match root {
        _ => todo!()
    }
}
