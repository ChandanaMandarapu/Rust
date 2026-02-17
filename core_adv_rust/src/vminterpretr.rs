// today practice session 4

// Implemented a full mini language from scratch including character level lexer, recursive descent parser with operator precedence, AST representation, and runtime interpreter with scoped variables and function support. Added conditionals, while loops, recursion, user defined functions, built in math and string operations, and print handling. Included error reporting for lexing parsing and runtime stages. Main demonstrates arithmetic control flow recursion fibonacci string handling and built in functions to validate the full pipeline end to end.

use std::collections::HashMap;
use std::fmt;

// ─── Lexer ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Literals
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Ident(String),
    // Operators
    Plus, Minus, Star, Slash, Percent,
    Eq, NotEq, Lt, Gt, LtEq, GtEq,
    And, Or, Not,
    Assign,
    // Delimiters
    LParen, RParen, LBrace, RBrace, Semicolon, Comma,
    // Keywords
    Let, If, Else, While, Fn, Return, Print,
    // Special
    Eof,
}

#[derive(Debug)]
struct LexError { msg: String, pos: usize }
impl fmt::Display for LexError { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "LexError at {}: {}", self.pos, self.msg) }}

struct Lexer { input: Vec<char>, pos: usize }

impl Lexer {
    fn new(src: &str) -> Self { Lexer { input: src.chars().collect(), pos: 0 } }

    fn peek(&self) -> Option<char>    { self.input.get(self.pos).copied() }
    fn advance(&mut self) -> Option<char> {
        let c = self.input.get(self.pos).copied();
        self.pos += 1;
        c
    }
    fn skip_ws(&mut self) {
        while self.peek().map_or(false, |c| c.is_whitespace()) { self.advance(); }
    }

    fn read_while<F: Fn(char) -> bool>(&mut self, pred: F) -> String {
        let mut s = String::new();
        while self.peek().map_or(false, |c| pred(c)) {
            s.push(self.advance().unwrap());
        }
        s
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = vec![];
        loop {
            self.skip_ws();
            let pos = self.pos;
            match self.peek() {
                None => { tokens.push(Token::Eof); break; }
                Some(c) => {
                    let tok = match c {
                        '+' => { self.advance(); Token::Plus }
                        '-' => { self.advance(); Token::Minus }
                        '*' => { self.advance(); Token::Star }
                        '/' => {
                            self.advance();
                            if self.peek() == Some('/') {
                                self.read_while(|c| c != '\n');
                                continue;
                            }
                            Token::Slash
                        }
                        '%' => { self.advance(); Token::Percent }
                        '(' => { self.advance(); Token::LParen }
                        ')' => { self.advance(); Token::RParen }
                        '{' => { self.advance(); Token::LBrace }
                        '}' => { self.advance(); Token::RBrace }
                        ';' => { self.advance(); Token::Semicolon }
                        ',' => { self.advance(); Token::Comma }
                        '=' => {
                            self.advance();
                            if self.peek() == Some('=') { self.advance(); Token::Eq }
                            else { Token::Assign }
                        }
                        '!' => {
                            self.advance();
                            if self.peek() == Some('=') { self.advance(); Token::NotEq }
                            else { Token::Not }
                        }
                        '<' => {
                            self.advance();
                            if self.peek() == Some('=') { self.advance(); Token::LtEq }
                            else { Token::Lt }
                        }
                        '>' => {
                            self.advance();
                            if self.peek() == Some('=') { self.advance(); Token::GtEq }
                            else { Token::Gt }
                        }
                        '&' => {
                            self.advance();
                            if self.peek() == Some('&') { self.advance(); Token::And }
                            else { return Err(LexError { msg: "expected &&".to_string(), pos }); }
                        }
                        '|' => {
                            self.advance();
                            if self.peek() == Some('|') { self.advance(); Token::Or }
                            else { return Err(LexError { msg: "expected ||".to_string(), pos }); }
                        }
                        '"' => {
                            self.advance();
                            let s = self.read_while(|c| c != '"');
                            self.advance(); // closing "
                            Token::Str(s)
                        }
                        c if c.is_ascii_digit() => {
                            let s = self.read_while(|c| c.is_ascii_digit() || c == '.');
                            if s.contains('.') {
                                Token::Float(s.parse().map_err(|_| LexError { msg: format!("bad float {}", s), pos })?)
                            } else {
                                Token::Int(s.parse().map_err(|_| LexError { msg: format!("bad int {}", s), pos })?)
                            }
                        }
                        c if c.is_alphabetic() || c == '_' => {
                            let s = self.read_while(|c| c.is_alphanumeric() || c == '_');
                            match s.as_str() {
                                "let"    => Token::Let,
                                "if"     => Token::If,
                                "else"   => Token::Else,
                                "while"  => Token::While,
                                "fn"     => Token::Fn,
                                "return" => Token::Return,
                                "print"  => Token::Print,
                                "true"   => Token::Bool(true),
                                "false"  => Token::Bool(false),
                                _        => Token::Ident(s),
                            }
                        }
                        c => return Err(LexError { msg: format!("unexpected char '{}'", c), pos }),
                    };
                    tokens.push(tok);
                }
            }
        }
        Ok(tokens)
    }
}

// ─── AST ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Expr {
    Int(i64), Float(f64), Bool(bool), Str(String),
    Ident(String),
    BinOp { op: BinOp, left: Box<Expr>, right: Box<Expr> },
    UnOp  { op: UnOp, expr: Box<Expr> },
    Call  { name: String, args: Vec<Expr> },
    Assign { name: String, value: Box<Expr> },
}

#[derive(Debug, Clone, Copy)]
enum BinOp { Add, Sub, Mul, Div, Mod, Eq, NotEq, Lt, Gt, LtEq, GtEq, And, Or }

#[derive(Debug, Clone, Copy)]
enum UnOp { Neg, Not }

#[derive(Debug, Clone)]
enum Stmt {
    Expr(Expr),
    Let { name: String, value: Expr },
    If  { cond: Expr, then: Vec<Stmt>, else_: Option<Vec<Stmt>> },
    While { cond: Expr, body: Vec<Stmt> },
    Fn  { name: String, params: Vec<String>, body: Vec<Stmt> },
    Return(Expr),
    Print(Expr),
    Block(Vec<Stmt>),
}

// ─── Parser ───────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct ParseError { msg: String }
impl fmt::Display for ParseError { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ParseError: {}", self.msg) }}

struct Parser { tokens: Vec<Token>, pos: usize }

impl Parser {
    fn new(tokens: Vec<Token>) -> Self { Parser { tokens, pos: 0 } }
    fn peek(&self) -> &Token { &self.tokens[self.pos] }
    fn advance(&mut self) -> &Token { let t = &self.tokens[self.pos]; self.pos += 1; t }
    fn expect(&mut self, t: &Token) -> Result<(), ParseError> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(t) {
            self.advance(); Ok(())
        } else {
            Err(ParseError { msg: format!("expected {:?}, got {:?}", t, self.peek()) })
        }
    }
    fn check(&self, t: &Token) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(t)
    }

    fn parse_program(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = vec![];
        while !self.check(&Token::Eof) { stmts.push(self.parse_stmt()?); }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek().clone() {
            Token::Let => {
                self.advance();
                let name = match self.advance().clone() { Token::Ident(n) => n, t => return Err(ParseError { msg: format!("expected ident, got {:?}", t) }) };
                self.expect(&Token::Assign)?;
                let value = self.parse_expr()?;
                self.expect(&Token::Semicolon)?;
                Ok(Stmt::Let { name, value })
            }
            Token::If => {
                self.advance();
                let cond = self.parse_expr()?;
                let then = self.parse_block()?;
                let else_ = if self.check(&Token::Else) { self.advance(); Some(self.parse_block()?) } else { None };
                Ok(Stmt::If { cond, then, else_ })
            }
            Token::While => {
                self.advance();
                let cond = self.parse_expr()?;
                let body = self.parse_block()?;
                Ok(Stmt::While { cond, body })
            }
            Token::Fn => {
                self.advance();
                let name = match self.advance().clone() { Token::Ident(n) => n, _ => return Err(ParseError { msg: "expected fn name".to_string() }) };
                self.expect(&Token::LParen)?;
                let mut params = vec![];
                while !self.check(&Token::RParen) {
                    if let Token::Ident(p) = self.advance().clone() { params.push(p); }
                    if self.check(&Token::Comma) { self.advance(); }
                }
                self.expect(&Token::RParen)?;
                let body = self.parse_block()?;
                Ok(Stmt::Fn { name, params, body })
            }
            Token::Return => { self.advance(); let e = self.parse_expr()?; self.expect(&Token::Semicolon)?; Ok(Stmt::Return(e)) }
            Token::Print  => { self.advance(); let e = self.parse_expr()?; self.expect(&Token::Semicolon)?; Ok(Stmt::Print(e)) }
            _ => {
                let e = self.parse_expr()?;
                self.expect(&Token::Semicolon)?;
                Ok(Stmt::Expr(e))
            }
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.expect(&Token::LBrace)?;
        let mut stmts = vec![];
        while !self.check(&Token::RBrace) && !self.check(&Token::Eof) { stmts.push(self.parse_stmt()?); }
        self.expect(&Token::RBrace)?;
        Ok(stmts)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> { self.parse_or() }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_and()?;
        while self.check(&Token::Or) { self.advance(); let r = self.parse_and()?; l = Expr::BinOp { op: BinOp::Or, left: Box::new(l), right: Box::new(r) }; }
        Ok(l)
    }
    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_eq()?;
        while self.check(&Token::And) { self.advance(); let r = self.parse_eq()?; l = Expr::BinOp { op: BinOp::And, left: Box::new(l), right: Box::new(r) }; }
        Ok(l)
    }
    fn parse_eq(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_cmp()?;
        loop { match self.peek() {
            Token::Eq    => { self.advance(); let r = self.parse_cmp()?; l = Expr::BinOp { op: BinOp::Eq,    left: Box::new(l), right: Box::new(r) }; }
            Token::NotEq => { self.advance(); let r = self.parse_cmp()?; l = Expr::BinOp { op: BinOp::NotEq, left: Box::new(l), right: Box::new(r) }; }
            _ => break,
        }}
        Ok(l)
    }
    fn parse_cmp(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_add()?;
        loop { let op = match self.peek() {
            Token::Lt   => BinOp::Lt,  Token::Gt   => BinOp::Gt,
            Token::LtEq => BinOp::LtEq, Token::GtEq => BinOp::GtEq, _ => break,
        }; self.advance(); let r = self.parse_add()?; l = Expr::BinOp { op, left: Box::new(l), right: Box::new(r) }; }
        Ok(l)
    }
    fn parse_add(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_mul()?;
        loop { let op = match self.peek() { Token::Plus => BinOp::Add, Token::Minus => BinOp::Sub, _ => break };
            self.advance(); let r = self.parse_mul()?; l = Expr::BinOp { op, left: Box::new(l), right: Box::new(r) }; }
        Ok(l)
    }
    fn parse_mul(&mut self) -> Result<Expr, ParseError> {
        let mut l = self.parse_unary()?;
        loop { let op = match self.peek() { Token::Star => BinOp::Mul, Token::Slash => BinOp::Div, Token::Percent => BinOp::Mod, _ => break };
            self.advance(); let r = self.parse_unary()?; l = Expr::BinOp { op, left: Box::new(l), right: Box::new(r) }; }
        Ok(l)
    }
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().clone() {
            Token::Minus => { self.advance(); Ok(Expr::UnOp { op: UnOp::Neg, expr: Box::new(self.parse_primary()?) }) }
            Token::Not   => { self.advance(); Ok(Expr::UnOp { op: UnOp::Not, expr: Box::new(self.parse_primary()?) }) }
            _ => self.parse_primary(),
        }
    }
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.advance().clone() {
            Token::Int(n)    => Ok(Expr::Int(n)),
            Token::Float(f)  => Ok(Expr::Float(f)),
            Token::Bool(b)   => Ok(Expr::Bool(b)),
            Token::Str(s)    => Ok(Expr::Str(s)),
            Token::Ident(name) => {
                if self.check(&Token::LParen) {
                    self.advance();
                    let mut args = vec![];
                    while !self.check(&Token::RParen) {
                        args.push(self.parse_expr()?);
                        if self.check(&Token::Comma) { self.advance(); }
                    }
                    self.expect(&Token::RParen)?;
                    Ok(Expr::Call { name, args })
                } else if self.check(&Token::Assign) {
                    self.advance();
                    Ok(Expr::Assign { name, value: Box::new(self.parse_expr()?) })
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            Token::LParen => { let e = self.parse_expr()?; self.expect(&Token::RParen)?; Ok(e) }
            t => Err(ParseError { msg: format!("unexpected token {:?}", t) }),
        }
    }
}

// ─── Interpreter ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Value { Int(i64), Float(f64), Bool(bool), Str(String), Nil }

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(n)   => write!(f, "{}", n),
            Value::Float(v) => write!(f, "{}", v),
            Value::Bool(b)  => write!(f, "{}", b),
            Value::Str(s)   => write!(f, "{}", s),
            Value::Nil      => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
struct RuntimeError { msg: String }
impl fmt::Display for RuntimeError { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "RuntimeError: {}", self.msg) }}

enum Control { Return(Value), Continue }

struct Interpreter {
    globals:   HashMap<String, Value>,
    functions: HashMap<String, (Vec<String>, Vec<Stmt>)>,
    output:    Vec<String>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter { globals: HashMap::new(), functions: HashMap::new(), output: vec![] }
    }

    fn eval(&mut self, expr: &Expr, locals: &mut HashMap<String, Value>) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Int(n)    => Ok(Value::Int(*n)),
            Expr::Float(f)  => Ok(Value::Float(*f)),
            Expr::Bool(b)   => Ok(Value::Bool(*b)),
            Expr::Str(s)    => Ok(Value::Str(s.clone())),
            Expr::Ident(name) => {
                locals.get(name).or_else(|| self.globals.get(name)).cloned()
                    .ok_or_else(|| RuntimeError { msg: format!("undefined: {}", name) })
            }
            Expr::Assign { name, value } => {
                let v = self.eval(value, locals)?;
                if locals.contains_key(name) { locals.insert(name.clone(), v.clone()); }
                else { self.globals.insert(name.clone(), v.clone()); }
                Ok(v)
            }
            Expr::UnOp { op, expr } => {
                let v = self.eval(expr, locals)?;
                match (op, v) {
                    (UnOp::Neg, Value::Int(n))   => Ok(Value::Int(-n)),
                    (UnOp::Neg, Value::Float(f)) => Ok(Value::Float(-f)),
                    (UnOp::Not, Value::Bool(b))  => Ok(Value::Bool(!b)),
                    (op, v) => Err(RuntimeError { msg: format!("bad unop {:?} on {}", op, v) }),
                }
            }
            Expr::BinOp { op, left, right } => {
                let l = self.eval(left, locals)?;
                let r = self.eval(right, locals)?;
                match (op, l, r) {
                    (BinOp::Add, Value::Int(a),   Value::Int(b))   => Ok(Value::Int(a + b)),
                    (BinOp::Add, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                    (BinOp::Add, Value::Str(a),   Value::Str(b))   => Ok(Value::Str(a + &b)),
                    (BinOp::Sub, Value::Int(a),   Value::Int(b))   => Ok(Value::Int(a - b)),
                    (BinOp::Sub, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                    (BinOp::Mul, Value::Int(a),   Value::Int(b))   => Ok(Value::Int(a * b)),
                    (BinOp::Mul, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                    (BinOp::Div, Value::Int(a),   Value::Int(b))   => {
                        if b == 0 { return Err(RuntimeError { msg: "division by zero".to_string() }); }
                        Ok(Value::Int(a / b))
                    }
                    (BinOp::Div, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                    (BinOp::Mod, Value::Int(a),   Value::Int(b))   => Ok(Value::Int(a % b)),
                    (BinOp::Eq,  a, b) => Ok(Value::Bool(format!("{}", a) == format!("{}", b))),
                    (BinOp::NotEq, a, b) => Ok(Value::Bool(format!("{}", a) != format!("{}", b))),
                    (BinOp::Lt,   Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a < b)),
                    (BinOp::Gt,   Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a > b)),
                    (BinOp::LtEq, Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a <= b)),
                    (BinOp::GtEq, Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a >= b)),
                    (BinOp::And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
                    (BinOp::Or,  Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
                    (op, l, r) => Err(RuntimeError { msg: format!("bad binop {:?} on {} and {}", op, l, r) }),
                }
            }
            Expr::Call { name, args } => {
                let eval_args: Result<Vec<Value>, _> = args.iter().map(|a| self.eval(a, locals)).collect();
                let eval_args = eval_args?;

                // Built-in functions
                match name.as_str() {
                    "sqrt"  => { if let Value::Float(f) = &eval_args[0] { return Ok(Value::Float(f.sqrt())); } }
                    "abs"   => { if let Value::Int(n)   = &eval_args[0] { return Ok(Value::Int(n.abs())); } }
                    "len"   => { if let Value::Str(s)   = &eval_args[0] { return Ok(Value::Int(s.len() as i64)); } }
                    "str"   => { return Ok(Value::Str(format!("{}", eval_args[0]))); }
                    _ => {}
                }

                let (params, body) = self.functions.get(name)
                    .ok_or_else(|| RuntimeError { msg: format!("undefined fn: {}", name) })?.clone();

                let mut fn_locals: HashMap<String, Value> = params.iter().zip(eval_args.iter())
                    .map(|(p, v)| (p.clone(), v.clone())).collect();

                match self.run_block(&body, &mut fn_locals)? {
                    Control::Return(v) => Ok(v),
                    Control::Continue  => Ok(Value::Nil),
                }
            }
        }
    }

    fn run_block(&mut self, stmts: &[Stmt], locals: &mut HashMap<String, Value>) -> Result<Control, RuntimeError> {
        for stmt in stmts {
            if let Control::Return(v) = self.run_stmt(stmt, locals)? {
                return Ok(Control::Return(v));
            }
        }
        Ok(Control::Continue)
    }

    fn run_stmt(&mut self, stmt: &Stmt, locals: &mut HashMap<String, Value>) -> Result<Control, RuntimeError> {
        match stmt {
            Stmt::Expr(e) => { self.eval(e, locals)?; }
            Stmt::Let { name, value } => {
                let v = self.eval(value, locals)?;
                locals.insert(name.clone(), v);
            }
            Stmt::If { cond, then, else_ } => {
                let c = self.eval(cond, locals)?;
                let branch = if let Value::Bool(true) = c { Some(then) } else { else_.as_ref() };
                if let Some(b) = branch {
                    if let Control::Return(v) = self.run_block(b, locals)? {
                        return Ok(Control::Return(v));
                    }
                }
            }
            Stmt::While { cond, body } => {
                loop {
                    let c = self.eval(cond, locals)?;
                    if let Value::Bool(false) | Value::Nil = c { break; }
                    if let Control::Return(v) = self.run_block(body, locals)? {
                        return Ok(Control::Return(v));
                    }
                }
            }
            Stmt::Fn { name, params, body } => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
            }
            Stmt::Return(e) => { return Ok(Control::Return(self.eval(e, locals)?)); }
            Stmt::Print(e)  => {
                let v = self.eval(e, locals)?;
                self.output.push(format!("{}", v));
                println!("  >> {}", v);
            }
            Stmt::Block(b) => {
                if let Control::Return(v) = self.run_block(b, locals)? {
                    return Ok(Control::Return(v));
                }
            }
        }
        Ok(Control::Continue)
    }

    fn run(&mut self, src: &str) -> Result<(), String> {
        let tokens = Lexer::new(src).tokenize().map_err(|e| e.to_string())?;
        let ast    = Parser::new(tokens).parse_program().map_err(|e| e.to_string())?;
        let mut locals = HashMap::new();
        self.run_block(&ast, &mut locals).map(|_| ()).map_err(|e| e.to_string())
    }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== Bytecode VM & Interpreter ===\n");

    let mut vm = Interpreter::new();

    let programs = vec![
        ("Basic arithmetic", r#"
            let x = 10;
            let y = 3;
            print x + y;
            print x * y;
            print x - y;
            print x / y;
            print x % y;
        "#),
        ("Conditionals", r#"
            let n = 42;
            if n > 40 {
                print "n is greater than 40";
            } else {
                print "n is small";
            }
            if n == 42 {
                print "the answer!";
            }
        "#),
        ("While loop + accumulator", r#"
            let i = 1;
            let sum = 0;
            while i <= 10 {
                sum = sum + i;
                i = i + 1;
            }
            print sum;
        "#),
        ("Functions + recursion", r#"
            fn factorial(n) {
                if n <= 1 {
                    return 1;
                }
                return n * factorial(n - 1);
            }
            print factorial(5);
            print factorial(10);
        "#),
        ("Fibonacci", r#"
            fn fib(n) {
                if n <= 1 { return n; }
                return fib(n - 1) + fib(n - 2);
            }
            let i = 0;
            while i <= 9 {
                print fib(i);
                i = i + 1;
            }
        "#),
        ("String operations", r#"
            let greeting = "Hello";
            let name     = "Rustacean";
            print greeting + ", " + name + "!";
            print len(greeting);
            print str(42) + " is the answer";
        "#),
        ("Built-in math", r#"
            print sqrt(2.0);
            print abs(-99);
        "#),
    ];

    for (title, src) in programs {
        println!("── {} ──", title);
        match vm.run(src) {
            Ok(())  => {}
            Err(e)  => println!("  ERROR: {}", e),
        }
        println!();
    }

    println!("=== Done ===");
}