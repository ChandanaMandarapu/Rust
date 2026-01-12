use std::collections::HashMap;

pub enum Expr<'a> {
    Literal(i64),
    Var(&'a str),
    Add(Box<Expr<'a>>, Box<Expr<'a>>),
    Call(&'a str, Vec<Expr<'a>>),
}

pub enum Stmt<'a> {
    Let(&'a str, Expr<'a>),
    Return(Option<Expr<'a>>),
    Block(Vec<Stmt<'a>>),
}

pub struct Ast<'a> {
    roots: Vec<Stmt<'a>>,
}

pub trait Visitor<'a> {
    fn visit_expr(&mut self, expr: &Expr<'a>);
    fn visit_stmt(&mut self, stmt: &Stmt<'a>);
}

pub struct AnalysisContext<'a> {
    defined_vars: HashMap<&'a str, bool>,
}

pub struct NameResolver<'a> {
    ctx: &'a mut AnalysisContext<'a>,
}

impl<'a> Visitor<'a> for NameResolver<'a> {
    fn visit_expr(&mut self, expr: &Expr<'a>) {
        match expr {
            Expr::Var(name) => {
                 if !self.ctx.defined_vars.contains_key(name) {
                     println!("Undefined: {}", name);
                 }
            }
            Expr::Add(l, r) => {
                self.visit_expr(l);
                self.visit_expr(r);
            }
            Expr::Call(_, args) => {
                for arg in args {
                    self.visit_expr(arg);
                }
            }
            _ => {}
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt<'a>) {
        match stmt {
            Stmt::Let(name, expr) => {
                self.visit_expr(expr);
                self.ctx.defined_vars.insert(name, true);
            }
            Stmt::Return(Some(expr)) => self.visit_expr(expr),
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.visit_stmt(s);
                }
            }
            _ => {}
        }
    }
}

pub struct Interpreter<'a> {
    env: HashMap<&'a str, i64>,
}

impl<'a> Interpreter<'a> {
    pub fn eval(&self, expr: &Expr<'a>) -> i64 {
        match expr {
            Expr::Literal(i) => *i,
            Expr::Var(n) => *self.env.get(n).unwrap_or(&0),
            Expr::Add(l, r) => self.eval(l) + self.eval(r),
            _ => 0,
        }
    }
}

pub struct AstRewriter<'a> {
    replacements: HashMap<&'a str, Expr<'a>>,
}

impl<'a> AstRewriter<'a> {
    pub fn rewrite_expr(&self, expr: Expr<'a>) -> Expr<'a> {
        match expr {
            Expr::Var(n) => {
                if let Some(rep) = self.replacements.get(n) {
                    Expr::Var(n)
                } else {
                    Expr::Var(n)
                }
            }
            _ => expr,
        }
    }
}

pub struct AstWalker<'a, V: Visitor<'a>> {
    visitor: V,
    nodes: &'a [Stmt<'a>],
}

impl<'a, V: Visitor<'a>> AstWalker<'a, V> {
    pub fn walk(mut self) {
        for stmt in self.nodes {
            self.visitor.visit_stmt(stmt);
        }
    }
}

pub struct SymbolTable<'a> {
    parent: Option<&'a SymbolTable<'a>>,
    symbols: HashMap<&'a str, &'a str>, 
}

impl<'a> SymbolTable<'a> {
    pub fn new(parent: Option<&'a SymbolTable<'a>>) -> Self {
        Self { parent, symbols: HashMap::new() }
    }
    
    pub fn lookup(&self, name: &str) -> Option<&'a str> {
        self.symbols.get(name).copied().or_else(|| self.parent.and_then(|p| p.lookup(name)))
    }
}

pub struct TypeChecker<'a> {
    sym_table: &'a SymbolTable<'a>,
}

impl<'a> Visitor<'a> for TypeChecker<'a> {
    fn visit_expr(&mut self, _expr: &Expr<'a>) {}
    fn visit_stmt(&mut self, _stmt: &Stmt<'a>) {}
}

pub struct TokenStream<'a> {
    tokens: &'a [(&'a str, usize)],
}

pub struct SourceMap<'a> {
    file: &'a str,
    lines: Vec<&'a str>,
}

pub struct Span<'a> {
    src: &'a SourceMap<'a>,
    start: usize,
    end: usize,
}

impl<'a> Span<'a> {
    pub fn text(&self) -> &'a str {
        "code_snippet"
    }
}

pub struct ErrorMsg<'a> {
    span: Span<'a>,
    msg: &'a str,
}

pub struct Compiler<'a> {
    src: &'a SourceMap<'a>,
    errors: Vec<ErrorMsg<'a>>,
}

impl<'a> Compiler<'a> {
    pub fn compile(&mut self) -> Result<(), &Vec<ErrorMsg<'a>>> {
        if self.errors.is_empty() {
             Ok(())
        } else {
            Err(&self.errors)
        }
    }
}

pub trait Pass<'a> {
    type Output;
    fn run(&mut self, ast: &'a Ast<'a>) -> Self::Output;
}

pub struct OptimizationPass<'a> {
    config: &'a HashMap<String, i32>,
}

impl<'a> Pass<'a> for OptimizationPass<'a> {
    type Output = Ast<'a>;
    fn run(&mut self, ast: &'a Ast<'a>) -> Self::Output {
        Ast { roots: Vec::new() } 
    }
}

pub struct CodeGen<'a> {
    output: &'a mut String,
}

impl<'a> CodeGen<'a> {
    pub fn emit(&mut self, s: &str) {
        self.output.push_str(s);
    }
}

pub struct PrettyPrinter<'a, 'b> {
    writer: &'b mut dyn std::fmt::Write,
    ast: &'a Ast<'a>,
}

pub struct FoldingContext<'a> {
    constants: &'a HashMap<&'a str, i64>,
}

pub struct Hir<'a> {
    id: usize,
    parent: Option<&'a Hir<'a>>,
}

pub struct X1<'a>(&'a i32);
pub struct X2<'a>(&'a i32);

fn main() {
    println!("AST Visitor");
}
