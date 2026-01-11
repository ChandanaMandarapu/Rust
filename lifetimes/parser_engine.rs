use std::borrow::Cow;
use std::collections::HashMap;
use std::marker::PhantomData;

pub enum Token<'a> {
    Identifier(&'a str),
    StringLiteral(Cow<'a, str>),
    Number(f64),
    Operator(&'a str),
    Keyword(&'a str),
    EndOfFile,
}

pub struct ParsingContext<'a> {
    input: &'a str,
    cursor: usize,
    memo: HashMap<usize, Token<'a>>,
}

pub struct Parser<'a> {
    context: &'a mut ParsingContext<'a>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> {
    pub fn new(context: &'a mut ParsingContext<'a>) -> Self {
        Self { context, _marker: PhantomData }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.context.input[self.context.cursor..].chars().next()
    }

    pub fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.context.cursor += c.len_utf8();
        }
    }

    pub fn parse_token(&mut self) -> Result<Token<'a>, String> {
        self.skip_whitespace();
        if self.is_eof() {
            return Ok(Token::EndOfFile);
        }

        let start = self.context.cursor;
        let c = self.peek().unwrap();

        if c.is_alphabetic() {
            self.parse_identifier(start)
        } else if c.is_numeric() {
            self.parse_number()
        } else if c == '"' {
            self.parse_string()
        } else {
            self.advance();
            Ok(Token::Operator(&self.context.input[start..self.context.cursor]))
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn is_eof(&self) -> bool {
        self.context.cursor >= self.context.input.len()
    }

    fn parse_identifier(&mut self, start: usize) -> Result<Token<'a>, String> {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        Ok(Token::Identifier(&self.context.input[start..self.context.cursor]))
    }

    fn parse_number(&mut self) -> Result<Token<'a>, String> {
        let start = self.context.cursor;
        while let Some(c) = self.peek() {
            if c.is_numeric() || c == '.' {
                self.advance();
            } else {
                break;
            }
        }
        let slice = &self.context.input[start..self.context.cursor];
        match slice.parse::<f64>() {
            Ok(n) => Ok(Token::Number(n)),
            Err(_) => Err(format!("Invalid number: {}", slice)),
        }
    }

    fn parse_string(&mut self) -> Result<Token<'a>, String> {
        self.advance(); 
        let start = self.context.cursor;
        loop {
            if self.is_eof() {
                return Err("Unterminated string".to_string());
            }
            if let Some(c) = self.peek() {
                if c == '"' {
                    let slice = &self.context.input[start..self.context.cursor];
                    self.advance(); 
                    return Ok(Token::StringLiteral(Cow::Borrowed(slice)));
                } else if c == '\\' {
                    return self.parse_escaped_string(start);
                }
                self.advance();
            }
        }
    }

    fn parse_escaped_string(&mut self, start_content: usize) -> Result<Token<'a>, String> {
        let mut s = String::from(&self.context.input[start_content..self.context.cursor]);
        
        loop {
            if self.is_eof() {
                return Err("Unterminated string".to_string());
            }
            if let Some(c) = self.peek() {
                 if c == '"' {
                    self.advance();
                    return Ok(Token::StringLiteral(Cow::Owned(s)));
                } else if c == '\\' {
                    self.advance();
                    if let Some(escaped) = self.peek() {
                         s.push(escaped);
                         self.advance();
                    }
                } else {
                    s.push(c);
                    self.advance();
                }
            }
        }
    }
}

pub struct AstNode<'a> {
    token: Token<'a>,
    children: Vec<AstNode<'a>>,
    span: &'a str,
}

pub trait AstVisitor<'a> {
    fn visit(&mut self, node: &'a AstNode<'a>);
}

pub struct RecursiveParser<'a> {
    tokens: &'a [Token<'a>],
    pos: usize,
}

impl<'a> RecursiveParser<'a> {
    pub fn parse_expression(&mut self) -> Option<AstNode<'a>> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        self.pos += 1;
        Some(AstNode {
            token: Token::Identifier("dummy"),
            children: Vec::new(),
            span: "dummy",
        })
    }
}

pub struct ErrorReporter<'a> {
    source: &'a str,
    errors: Vec<(&'a str, String)>,
}

impl<'a> ErrorReporter<'a> {
    pub fn report(&mut self, span: &'a str, msg: String) {
        let start = span.as_ptr() as usize;
        let source_start = self.source.as_ptr() as usize;
        if start >= source_start && start < source_start + self.source.len() {
            self.errors.push((span, msg));
        }
    }
}

pub struct SymbolTable<'a> {
    parent: Option<&'a SymbolTable<'a>>,
    symbols: HashMap<&'a str, Token<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn lookup(&self, name: &str) -> Option<&Token<'a>> {
        if let Some(t) = self.symbols.get(name) {
            Some(t)
        } else if let Some(p) = self.parent {
            p.lookup(name)
        } else {
            None
        }
    }
}

pub struct InputStream<'a> {
    chunks: Vec<&'a str>,
}

impl<'a> InputStream<'a> {
    pub fn iter_chars(&'a self) -> impl Iterator<Item = char> + 'a {
        self.chunks.iter().flat_map(|s| s.chars())
    }
}

pub struct ZeroCopyBuffer<'a> {
    data: &'a [u8],
}

impl<'a> ZeroCopyBuffer<'a> {
    pub fn read_u32(&self, offset: usize) -> Option<u32> {
        if offset + 4 <= self.data.len() {
            let bytes = &self.data[offset..offset+4];
            Some(u32::from_le_bytes(bytes.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn get_slice(&self, start: usize, len: usize) -> Option<&'a [u8]> {
        if start + len <= self.data.len() {
            Some(&self.data[start..start+len])
        } else {
            None
        }
    }
}

pub struct DoubleLife<'a, 'b> {
    x: &'a str,
    y: &'b str,
}

pub trait OutputBound<'a> {
    type Out: 'a;
    fn produce(&self) -> Self::Out;
}

pub struct StrProducer<'a> {
    s: &'a str,
}

impl<'a> OutputBound<'a> for StrProducer<'a> {
    type Out = &'a str;
    fn produce(&self) -> Self::Out {
        self.s
    }
}

pub fn filter_lifetime<'a, F>(items: &'a [String], predicate: F) -> Vec<&'a String>
where F: Fn(&'a String) -> bool
{
    items.iter().filter(|s| predicate(s)).collect()
}

pub struct ComplexParserState<'a, 'b> 
where 'a: 'b 
{
    global_config: &'a HashMap<String, String>,
    local_overrides: &'b HashMap<String, String>,
}

impl<'a, 'b> ComplexParserState<'a, 'b> {
    pub fn get_config(&self, key: &str) -> Option<&'a String> {
        self.global_config.get(key)
    }

    pub fn get_local(&self, key: &str) -> Option<&'b String> {
        self.local_overrides.get(key)
    }
}

fn main() {
    let input = "x = 10";
    let mut ctx = ParsingContext {
        input,
        cursor: 0,
        memo: HashMap::new(),
    };
    let mut parser = Parser::new(&mut ctx);
    println!("Parser initialized");
}
