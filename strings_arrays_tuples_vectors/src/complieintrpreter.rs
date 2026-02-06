use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Number(f64),
    Identifier(String),
    String(String),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Semicolon,
    Colon,
    Arrow,
    Let,
    Fn,
    If,
    Else,
    While,
    For,
    Return,
    True,
    False,
    Nil,
    And,
    Or,
    Not,
    EOF,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    fn new(input: String) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_numeric() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        num_str.parse().unwrap_or(0.0)
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        ident
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.advance();

        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance();
                break;
            }
            if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char {
                    string.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        _ => escaped,
                    });
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }

        string
    }

    fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();

        match self.current_char {
            None => TokenType::EOF,
            Some(ch) => {
                if ch.is_numeric() {
                    return TokenType::Number(self.read_number());
                }

                if ch.is_alphabetic() || ch == '_' {
                    let ident = self.read_identifier();
                    return match ident.as_str() {
                        "let" => TokenType::Let,
                        "fn" => TokenType::Fn,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "while" => TokenType::While,
                        "for" => TokenType::For,
                        "return" => TokenType::Return,
                        "true" => TokenType::True,
                        "false" => TokenType::False,
                        "nil" => TokenType::Nil,
                        "and" => TokenType::And,
                        "or" => TokenType::Or,
                        "not" => TokenType::Not,
                        _ => TokenType::Identifier(ident),
                    };
                }

                if ch == '"' {
                    return TokenType::String(self.read_string());
                }

                let token = match ch {
                    '+' => TokenType::Plus,
                    '*' => TokenType::Star,
                    '/' => TokenType::Slash,
                    '%' => TokenType::Percent,
                    '(' => TokenType::LeftParen,
                    ')' => TokenType::RightParen,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    '[' => TokenType::LeftBracket,
                    ']' => TokenType::RightBracket,
                    ',' => TokenType::Comma,
                    '.' => TokenType::Dot,
                    ';' => TokenType::Semicolon,
                    ':' => TokenType::Colon,
                    '-' => {
                        if self.peek(1) == Some('>') {
                            self.advance();
                            TokenType::Arrow
                        } else {
                            TokenType::Minus
                        }
                    }
                    '=' => {
                        if self.peek(1) == Some('=') {
                            self.advance();
                            TokenType::EqualEqual
                        } else {
                            TokenType::Equal
                        }
                    }
                    '!' => {
                        if self.peek(1) == Some('=') {
                            self.advance();
                            TokenType::BangEqual
                        } else {
                            TokenType::Not
                        }
                    }
                    '<' => {
                        if self.peek(1) == Some('=') {
                            self.advance();
                            TokenType::LessEqual
                        } else {
                            TokenType::Less
                        }
                    }
                    '>' => {
                        if self.peek(1) == Some('=') {
                            self.advance();
                            TokenType::GreaterEqual
                        } else {
                            TokenType::Greater
                        }
                    }
                    _ => TokenType::EOF,
                };

                self.advance();
                token
            }
        }
    }

    fn tokenize(&mut self) -> Vec<TokenType> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            if token == TokenType::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        tokens
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Array(Vec<Expr>),
    Object(HashMap<String, Expr>),
}

#[derive(Debug, Clone)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone)]
enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
enum Stmt {
    Expression(Expr),
    Let { name: String, value: Expr },
    Function { name: String, params: Vec<String>, body: Vec<Stmt> },
    Return(Option<Expr>),
    If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>> },
    While { condition: Expr, body: Vec<Stmt> },
    Block(Vec<Stmt>),
}

struct Parser {
    tokens: Vec<TokenType>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<TokenType>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn current(&self) -> &TokenType {
        self.tokens.get(self.position).unwrap_or(&TokenType::EOF)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn expect(&mut self, expected: TokenType) -> Result<(), String> {
        if std::mem::discriminant(self.current()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.current()))
        }
    }

    fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while *self.current() != TokenType::EOF {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.current().clone() {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Fn => self.parse_function(),
            TokenType::Return => self.parse_return(),
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::LeftBrace => self.parse_block(),
            _ => {
                let expr = self.parse_expression()?;
                self.expect(TokenType::Semicolon)?;
                Ok(Stmt::Expression(expr))
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<Stmt, String> {
        self.advance();
        
        let name = match self.current() {
            TokenType::Identifier(n) => n.clone(),
            _ => return Err("Expected identifier".to_string()),
        };
        self.advance();

        self.expect(TokenType::Equal)?;
        let value = self.parse_expression()?;
        self.expect(TokenType::Semicolon)?;

        Ok(Stmt::Let { name, value })
    }

    fn parse_function(&mut self) -> Result<Stmt, String> {
        self.advance();

        let name = match self.current() {
            TokenType::Identifier(n) => n.clone(),
            _ => return Err("Expected function name".to_string()),
        };
        self.advance();

        self.expect(TokenType::LeftParen)?;
        let mut params = Vec::new();

        while *self.current() != TokenType::RightParen {
            if let TokenType::Identifier(param) = self.current() {
                params.push(param.clone());
                self.advance();

                if *self.current() == TokenType::Comma {
                    self.advance();
                }
            } else {
                break;
            }
        }

        self.expect(TokenType::RightParen)?;
        self.expect(TokenType::LeftBrace)?;

        let mut body = Vec::new();
        while *self.current() != TokenType::RightBrace {
            body.push(self.parse_statement()?);
        }

        self.expect(TokenType::RightBrace)?;

        Ok(Stmt::Function { name, params, body })
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.advance();

        let value = if *self.current() == TokenType::Semicolon {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.expect(TokenType::Semicolon)?;
        Ok(Stmt::Return(value))
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.expect(TokenType::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(TokenType::RightParen)?;

        self.expect(TokenType::LeftBrace)?;
        let mut then_branch = Vec::new();
        while *self.current() != TokenType::RightBrace {
            then_branch.push(self.parse_statement()?);
        }
        self.expect(TokenType::RightBrace)?;

        let else_branch = if *self.current() == TokenType::Else {
            self.advance();
            self.expect(TokenType::LeftBrace)?;
            let mut else_stmts = Vec::new();
            while *self.current() != TokenType::RightBrace {
                else_stmts.push(self.parse_statement()?);
            }
            self.expect(TokenType::RightBrace)?;
            Some(else_stmts)
        } else {
            None
        };

        Ok(Stmt::If { condition, then_branch, else_branch })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance();
        self.expect(TokenType::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(TokenType::RightParen)?;

        self.expect(TokenType::LeftBrace)?;
        let mut body = Vec::new();
        while *self.current() != TokenType::RightBrace {
            body.push(self.parse_statement()?);
        }
        self.expect(TokenType::RightBrace)?;

        Ok(Stmt::While { condition, body })
    }

    fn parse_block(&mut self) -> Result<Stmt, String> {
        self.expect(TokenType::LeftBrace)?;
        let mut statements = Vec::new();

        while *self.current() != TokenType::RightBrace {
            statements.push(self.parse_statement()?);
        }

        self.expect(TokenType::RightBrace)?;
        Ok(Stmt::Block(statements))
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;

        while *self.current() == TokenType::Or {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;

        while *self.current() == TokenType::And {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;

        while matches!(self.current(), TokenType::EqualEqual | TokenType::BangEqual) {
            let op = match self.current() {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        while matches!(self.current(), TokenType::Less | TokenType::LessEqual | 
                      TokenType::Greater | TokenType::GreaterEqual) {
            let op = match self.current() {
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        while matches!(self.current(), TokenType::Plus | TokenType::Minus) {
            let op = match self.current() {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        while matches!(self.current(), TokenType::Star | TokenType::Slash | TokenType::Percent) {
            let op = match self.current() {
                TokenType::Star => BinaryOp::Mul,
                TokenType::Slash => BinaryOp::Div,
                TokenType::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.current() {
            TokenType::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                })
            }
            TokenType::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                })
            }
            _ => self.parse_call(),
        }
    }

    fn parse_call(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.current() {
                TokenType::LeftParen => {
                    self.advance();
                    let mut args = Vec::new();

                    while *self.current() != TokenType::RightParen {
                        args.push(self.parse_expression()?);
                        if *self.current() == TokenType::Comma {
                            self.advance();
                        }
                    }

                    self.expect(TokenType::RightParen)?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                TokenType::LeftBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(TokenType::RightBracket)?;
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current().clone() {
            TokenType::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            TokenType::String(s) => {
                self.advance();
                Ok(Expr::String(s))
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expr::Nil)
            }
            TokenType::Identifier(name) => {
                self.advance();
                Ok(Expr::Identifier(name))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenType::RightParen)?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();

                while *self.current() != TokenType::RightBracket {
                    elements.push(self.parse_expression()?);
                    if *self.current() == TokenType::Comma {
                        self.advance();
                    }
                }

                self.expect(TokenType::RightBracket)?;
                Ok(Expr::Array(elements))
            }
            _ => Err(format!("Unexpected token: {:?}", self.current())),
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Function { params: Vec<String>, body: Vec<Stmt> },
    Array(Vec<Value>),
}

struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}

struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    fn interpret(&mut self, program: Vec<Stmt>) -> Result<(), String> {
        for stmt in program {
            self.execute_statement(stmt)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, stmt: Stmt) -> Result<Option<Value>, String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expression(expr)?;
                Ok(None)
            }
            Stmt::Let { name, value } => {
                let val = self.evaluate_expression(value)?;
                self.environment.define(name, val);
                Ok(None)
            }
            Stmt::Function { name, params, body } => {
                let func = Value::Function { params, body };
                self.environment.define(name, func);
                Ok(None)
            }
            Stmt::Return(expr) => {
                let value = if let Some(e) = expr {
                    self.evaluate_expression(e)?
                } else {
                    Value::Nil
                };
                Ok(Some(value))
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let cond_val = self.evaluate_expression(condition)?;
                
                if self.is_truthy(&cond_val) {
                    for stmt in then_branch {
                        if let Some(ret) = self.execute_statement(stmt)? {
                            return Ok(Some(ret));
                        }
                    }
                } else if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        if let Some(ret) = self.execute_statement(stmt)? {
                            return Ok(Some(ret));
                        }
                    }
                }
                Ok(None)
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond_val = self.evaluate_expression(condition.clone())?;
                    if !self.is_truthy(&cond_val) {
                        break;
                    }
                    
                    for stmt in &body {
                        if let Some(ret) = self.execute_statement(stmt.clone())? {
                            return Ok(Some(ret));
                        }
                    }
                }
                Ok(None)
            }
            Stmt::Block(stmts) => {
                self.environment.push_scope();
                let mut result = None;
                
                for stmt in stmts {
                    if let Some(ret) = self.execute_statement(stmt)? {
                        result = Some(ret);
                        break;
                    }
                }
                
                self.environment.pop_scope();
                Ok(result)
            }
        }
    }

    fn evaluate_expression(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(n)),
            Expr::String(s) => Ok(Value::String(s)),
            Expr::Bool(b) => Ok(Value::Bool(b)),
            Expr::Nil => Ok(Value::Nil),
            Expr::Identifier(name) => {
                self.environment.get(&name)
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            Expr::Binary { left, op, right } => {
                let left_val = self.evaluate_expression(*left)?;
                let right_val = self.evaluate_expression(*right)?;
                self.apply_binary_op(left_val, op, right_val)
            }
            Expr::Unary { op, expr } => {
                let val = self.evaluate_expression(*expr)?;
                self.apply_unary_op(op, val)
            }
            Expr::Call { callee, args } => {
                let func = self.evaluate_expression(*callee)?;
                let arg_vals: Result<Vec<_>, _> = args.into_iter()
                    .map(|arg| self.evaluate_expression(arg))
                    .collect();
                self.call_function(func, arg_vals?)
            }
            Expr::Array(elements) => {
                let vals: Result<Vec<_>, _> = elements.into_iter()
                    .map(|e| self.evaluate_expression(e))
                    .collect();
                Ok(Value::Array(vals?))
            }
            _ => Err("Unsupported expression".to_string()),
        }
    }

    fn apply_binary_op(&self, left: Value, op: BinaryOp, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                Ok(match op {
                    BinaryOp::Add => Value::Number(l + r),
                    BinaryOp::Sub => Value::Number(l - r),
                    BinaryOp::Mul => Value::Number(l * r),
                    BinaryOp::Div => Value::Number(l / r),
                    BinaryOp::Mod => Value::Number(l % r),
                    BinaryOp::Equal => Value::Bool(l == r),
                    BinaryOp::NotEqual => Value::Bool(l != r),
                    BinaryOp::Less => Value::Bool(l < r),
                    BinaryOp::LessEqual => Value::Bool(l <= r),
                    BinaryOp::Greater => Value::Bool(l > r),
                    BinaryOp::GreaterEqual => Value::Bool(l >= r),
                    _ => return Err("Invalid operation".to_string()),
                })
            }
            _ => Err("Type error in binary operation".to_string()),
        }
    }

    fn apply_unary_op(&self, op: UnaryOp, val: Value) -> Result<Value, String> {
        match (op, val) {
            (UnaryOp::Neg, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOp::Not, v) => Ok(Value::Bool(!self.is_truthy(&v))),
            _ => Err("Type error in unary operation".to_string()),
        }
    }

    fn call_function(&mut self, func: Value, args: Vec<Value>) -> Result<Value, String> {
        match func {
            Value::Function { params, body } => {
                if params.len() != args.len() {
                    return Err("Argument count mismatch".to_string());
                }

                self.environment.push_scope();

                for (param, arg) in params.iter().zip(args.iter()) {
                    self.environment.define(param.clone(), arg.clone());
                }

                let mut result = Value::Nil;
                for stmt in body {
                    if let Some(ret) = self.execute_statement(stmt)? {
                        result = ret;
                        break;
                    }
                }

                self.environment.pop_scope();
                Ok(result)
            }
            _ => Err("Not a function".to_string()),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }
}