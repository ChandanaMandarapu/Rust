use std::io::{self, Write};

enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Vec<(String, Expr)>, Box<Expr>),
    Var(String),
}

fn eval(e: &Expr, env: &mut Vec<(String, i64)>) -> Option<i64> {
    match e {
        Expr::Num(n) => Some(*n),
        Expr::Add(l, r) => Some(eval(l, env)? + eval(r, env)?),
        Expr::Sub(l, r) => Some(eval(l, env)? - eval(r, env)?),
        Expr::Mul(l, r) => Some(eval(l, env)? * eval(r, env)?),
        Expr::Div(l, r) => {
            let a = eval(l, env)?;
            let b = eval(r, env)?;
            if b == 0 { None } else { Some(a / b) }
        }
        Expr::Mod(l, r) => {
            let a = eval(l, env)?;
            let b = eval(r, env)?;
            if b == 0 { None } else { Some(a % b) }
        }
        Expr::Neg(x) => Some(-eval(x, env)?),
        Expr::And(l, r) => Some((eval(l, env)? != 0 && eval(r, env)? != 0) as i64),
        Expr::Or(l, r) => Some((eval(l, env)? != 0 || eval(r, env)? != 0) as i64),
        Expr::Not(x) => Some((eval(x, env)? == 0) as i64),
        Expr::Eq(l, r) => Some((eval(l, env)? == eval(r, env)?) as i64),
        Expr::Lt(l, r) => Some((eval(l, env)? < eval(r, env)?) as i64),
        Expr::If(cond, t, f) => {
            if eval(cond, env)? != 0 {
                eval(t, env)
            } else {
                eval(f, env)
            }
        }
        Expr::Var(name) => env.iter().rev().find(|(k, _)| k == name).map(|&(_, v)| v),
        Expr::Let(bindings, body) => {
            let mut frame = Vec::new();
            for (name, expr) in bindings {
                frame.push((name.clone(), eval(expr, env)?));
            }
            env.extend(frame);
            let res = eval(body, env);
            for _ in bindings {
                env.pop();
            }
            res
        }
    }
}

fn parse_toks(toks: &[&str], pos: &mut usize) -> Option<Expr> {
    match toks.get(*pos)? {
        &"let" => {
            *pos += 1;
            let mut binds = Vec::new();
            loop {
                let name = *toks.get(*pos)?;
                *pos += 1;
                if toks.get(*pos) != Some(&"=") { return None; }
                *pos += 1;
                let expr = parse_toks(toks, pos)?;
                binds.push((name.to_string(), expr));
                match toks.get(*pos) {
                    Some(&"in") => { *pos += 1; break; }
                    Some(&",") => { *pos += 1; continue; }
                    _ => return None,
                }
            }
            let body = parse_toks(toks, pos)?;
            Some(Expr::Let(binds, Box::new(body)))
        }
        &"if" => {
            *pos += 1;
            let cond = parse_toks(toks, pos)?;
            if toks.get(*pos) != Some(&"then") { return None; }
            *pos += 1;
            let t = parse_toks(toks, pos)?;
            if toks.get(*pos) != Some(&"else") { return None; }
            *pos += 1;
            let f = parse_toks(toks, pos)?;
            Some(Expr::If(Box::new(cond), Box::new(t), Box::new(f)))
        }
        &"not" => {
            *pos += 1;
            let x = parse_toks(toks, pos)?;
            Some(Expr::Not(Box::new(x)))
        }
        &"-" if toks.get(*pos + 1).map(|s| s.chars().next().unwrap().is_numeric()).unwrap_or(false) => {
            *pos += 1;
            let n: i64 = toks.get(*pos)?.parse().ok()?;
            *pos += 1;
            Some(Expr::Num(-n))
        }
        s if s.chars().next().unwrap().is_numeric() => {
            let n: i64 = s.parse().ok()?;
            *pos += 1;
            Some(Expr::Num(n))
        }
        var => {
            let name = *var;
            *pos += 1;
            let mut left = Expr::Var(name.to_string());
            while let Some(op) = toks.get(*pos) {
                match *op {
                    "+" | "-" | "*" | "/" | "%" | "&&" | "||" | "==" | "<" => {
                        *pos += 1;
                        let right = parse_toks(toks, pos)?;
                        left = match op {
                            &"+" => Expr::Add(Box::new(left), Box::new(right)),
                            &"-" => Expr::Sub(Box::new(left), Box::new(right)),
                            &"*" => Expr::Mul(Box::new(left), Box::new(right)),
                            &"/" => Expr::Div(Box::new(left), Box::new(right)),
                            &"%" => Expr::Mod(Box::new(left), Box::new(right)),
                            &"&&" => Expr::And(Box::new(left), Box::new(right)),
                            &"||" => Expr::Or(Box::new(left), Box::new(right)),
                            &"==" => Expr::Eq(Box::new(left), Box::new(right)),
                            &"<" => Expr::Lt(Box::new(left), Box::new(right)),
                            _ => unreachable!(),
                        };
                    }
                    _ => break,
                }
            }
            Some(left)
        }
    }
}

fn main() {
    let code = "let x = 5, y = 10 in if x < y then x + y else y - x";
    let toks: Vec<_> = code.split_whitespace().collect();
    let mut pos = 0;
    let expr = parse_toks(&toks, &mut pos).unwrap();
    let mut env = Vec::new();
    let res = eval(&expr, &mut env).unwrap_or(-1);
    io::stdout().write_all(format!("{}\n", res).as_bytes()).unwrap();
}