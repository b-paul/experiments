#![feature(stmt_expr_attributes)]
#![feature(exact_div)]

use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Expr {
    Const(f64),
    Neg(Box<Expr>),
    Sqrt(Box<Expr>),
    Floor(Box<Expr>),
    Ceil(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Const(n) => write!(f, "{n}"),
            Expr::Neg(e) => write!(f, "-({e})"),
            Expr::Sqrt(e) => write!(f, "sqrt({e})"),
            Expr::Floor(e) => write!(f, "floor({e})"),
            Expr::Ceil(e) => write!(f, "ceil({e})"),
            Expr::Add(e1, e2) => write!(f, "({e1}) + ({e2})"),
            Expr::Mul(e1, e2) => write!(f, "({e1}) * ({e2})"),
            Expr::Div(e1, e2) => write!(f, "({e1}) / ({e2})"),
            Expr::Pow(e1, e2) => write!(f, "({e1}) ^ ({e2})"),
        }
    }
}

impl Expr {
    /// Evaluate an expression, returning None if an integer overflow or non-integer division is
    /// performed.
    pub fn eval(&self) -> Option<f64> {
        match self {
            Expr::Const(n) => Some(*n),
            Expr::Neg(e) => e.eval().map(|n| -n),
            Expr::Sqrt(e) => e.eval().map(|n| n.sqrt()),
            Expr::Floor(e) => e.eval().map(|n| n.floor()),
            Expr::Ceil(e) => e.eval().map(|n| n.ceil()),
            Expr::Add(e1, e2) => e1.eval().zip(e2.eval()).map(|(a, b)| a + b),
            Expr::Mul(e1, e2) => e1.eval().zip(e2.eval()).map(|(a, b)| a * b),
            Expr::Div(e1, e2) => e1.eval().zip(e2.eval()).map(|(a, b)| a / b),
            Expr::Pow(e1, e2) => e1.eval().zip(e2.eval()).map(|(a, b)| a.powf(b)),
        }
    }
}

fn all_uns(a: &Expr) -> Vec<Expr> {
    vec![
        a.clone(),
        Expr::Neg(Box::new(a.clone())),
        Expr::Sqrt(Box::new(a.clone())),
        Expr::Floor(Box::new(a.clone())),
        Expr::Ceil(Box::new(a.clone())),
        Expr::Floor(Box::new(Expr::Sqrt(Box::new(a.clone())))),
        Expr::Ceil(Box::new(Expr::Sqrt(Box::new(a.clone())))),
    ]
}

fn all_pairs(a: &Expr, b: &Expr) -> Vec<Expr> {
    let mut res = vec![];
    for a in all_uns(a) {
        for b in all_uns(b) {
            res.push(Expr::Add(Box::new(a.clone()), Box::new(b.clone())));
            res.push(Expr::Mul(Box::new(a.clone()), Box::new(b.clone())));
            res.push(Expr::Div(Box::new(a.clone()), Box::new(b.clone())));
            res.push(Expr::Pow(Box::new(a.clone()), Box::new(b.clone())));
        }
    }
    res
}

fn all_exprs(base: Vec<Expr>, goal: f64) -> Vec<Expr> {
    if base.len() == 1 {
        let e = base[0].clone();
        if e.eval() == Some(goal) {
            println!("{e}");
            return base;
        } else {
            return vec![];
        }
    }
    (0..base.len() - 1)
        .flat_map(|i| {
            let mut base = base.clone();
            let a = base.remove(i);
            let b = base.remove(i);
            all_pairs(&a, &b).into_iter().flat_map(move |e| {
                let mut base = base.clone();
                base.insert(i, e);
                all_exprs(base, goal)
            })
        })
        .collect()
}

fn main() {
    let mut m = HashMap::<i64, Vec<Expr>>::new();
    for (n, e) in all_exprs(
        vec![
            Expr::Const(100.0),
            Expr::Const(10.0),
            Expr::Const(5.0),
            Expr::Const(2.0),
        ],
        452.,
    )
    .into_iter()
    .flat_map(|e| {
        e.eval()
            .and_then(|n| (n == n.round()).then_some(n as i64))
            .map(|n| (n, e))
    }) {
        m.entry(n).or_insert(Vec::new()).push(e);
    }
    for e in m[&452].iter() {
        println!("{e}");
    }
}
