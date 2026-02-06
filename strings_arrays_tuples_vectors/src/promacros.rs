use std::collections::HashMap;

struct TokenStream {
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Clone, Debug)]
enum Token {
    Ident(String),
    Literal(Literal),
    Punct(char),
    Group(Delimiter, Vec<Token>),
}

#[derive(Clone, Debug)]
enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Char(char),
}

#[derive(Clone, Debug)]
enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
}

impl TokenStream {
    fn new(tokens: Vec<Token>) -> Self {
        TokenStream { tokens, position: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn next(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match self.next() {
            Some(Token::Ident(name)) => Ok(name),
            _ => Err("Expected identifier".to_string()),
        }
    }

    fn expect_punct(&mut self, expected: char) -> Result<(), String> {
        match self.next() {
            Some(Token::Punct(c)) if c == expected => Ok(()),
            _ => Err(format!("Expected '{}'", expected)),
        }
    }
}

struct DeriveInput {
    name: String,
    generics: Vec<String>,
    fields: Vec<Field>,
    attrs: Vec<Attribute>,
}

struct Field {
    name: String,
    ty: Type,
    attrs: Vec<Attribute>,
}

#[derive(Clone)]
struct Type {
    path: Vec<String>,
    generics: Vec<Type>,
}

struct Attribute {
    path: Vec<String>,
    tokens: Vec<Token>,
}

struct MacroParser {
    stream: TokenStream,
}

impl MacroParser {
    fn new(stream: TokenStream) -> Self {
        MacroParser { stream }
    }

    fn parse_derive_input(&mut self) -> Result<DeriveInput, String> {
        let attrs = self.parse_attributes()?;
        
        self.stream.expect_ident()?;
        let name = self.stream.expect_ident()?;
        
        let generics = if let Some(Token::Punct('<')) = self.stream.peek() {
            self.parse_generics()?
        } else {
            Vec::new()
        };

        let fields = if let Some(Token::Group(Delimiter::Brace, _)) = self.stream.peek() {
            self.parse_struct_fields()?
        } else {
            Vec::new()
        };

        Ok(DeriveInput {
            name,
            generics,
            fields,
            attrs,
        })
    }

    fn parse_attributes(&mut self) -> Result<Vec<Attribute>, String> {
        let mut attrs = Vec::new();
        
        while let Some(Token::Punct('#')) = self.stream.peek() {
            self.stream.next();
            
            if let Some(Token::Group(Delimiter::Bracket, tokens)) = self.stream.next() {
                let mut attr_stream = TokenStream::new(tokens);
                let path = self.parse_path(&mut attr_stream)?;
                
                attrs.push(Attribute {
                    path,
                    tokens: attr_stream.tokens[attr_stream.position..].to_vec(),
                });
            }
        }
        
        Ok(attrs)
    }

    fn parse_path(&self, stream: &mut TokenStream) -> Result<Vec<String>, String> {
        let mut path = Vec::new();
        
        loop {
            path.push(stream.expect_ident()?);
            
            if let Some(Token::Punct(':')) = stream.peek() {
                stream.next();
                stream.expect_punct(':')?;
            } else {
                break;
            }
        }
        
        Ok(path)
    }

    fn parse_generics(&mut self) -> Result<Vec<String>, String> {
        let mut generics = Vec::new();
        self.stream.expect_punct('<')?;
        
        loop {
            generics.push(self.stream.expect_ident()?);
            
            match self.stream.peek() {
                Some(Token::Punct(',')) => {
                    self.stream.next();
                }
                Some(Token::Punct('>')) => {
                    self.stream.next();
                    break;
                }
                _ => return Err("Expected ',' or '>'".to_string()),
            }
        }
        
        Ok(generics)
    }

    fn parse_struct_fields(&mut self) -> Result<Vec<Field>, String> {
        let mut fields = Vec::new();
        
        if let Some(Token::Group(Delimiter::Brace, tokens)) = self.stream.next() {
            let mut field_stream = TokenStream::new(tokens);
            
            while field_stream.peek().is_some() {
                let attrs = self.parse_field_attributes(&mut field_stream)?;
                let name = field_stream.expect_ident()?;
                field_stream.expect_punct(':')?;
                let ty = self.parse_type(&mut field_stream)?;
                
                fields.push(Field { name, ty, attrs });
                
                if let Some(Token::Punct(',')) = field_stream.peek() {
                    field_stream.next();
                }
            }
        }
        
        Ok(fields)
    }

    fn parse_field_attributes(&self, stream: &mut TokenStream) -> Result<Vec<Attribute>, String> {
        let mut attrs = Vec::new();
        
        while let Some(Token::Punct('#')) = stream.peek() {
            stream.next();
            
            if let Some(Token::Group(Delimiter::Bracket, tokens)) = stream.next() {
                let mut attr_stream = TokenStream::new(tokens);
                let path = self.parse_path(&mut attr_stream)?;
                
                attrs.push(Attribute {
                    path,
                    tokens: attr_stream.tokens[attr_stream.position..].to_vec(),
                });
            }
        }
        
        Ok(attrs)
    }

    fn parse_type(&self, stream: &mut TokenStream) -> Result<Type, String> {
        let mut path = Vec::new();
        
        loop {
            path.push(stream.expect_ident()?);
            
            if let Some(Token::Punct(':')) = stream.peek() {
                stream.next();
                stream.expect_punct(':')?;
            } else {
                break;
            }
        }
        
        let generics = if let Some(Token::Punct('<')) = stream.peek() {
            stream.next();
            let mut generics = Vec::new();
            
            loop {
                generics.push(self.parse_type(stream)?);
                
                match stream.peek() {
                    Some(Token::Punct(',')) => {
                        stream.next();
                    }
                    Some(Token::Punct('>')) => {
                        stream.next();
                        break;
                    }
                    _ => break,
                }
            }
            
            generics
        } else {
            Vec::new()
        };
        
        Ok(Type { path, generics })
    }
}

struct CodeGenerator {
    output: String,
    indent_level: usize,
}

impl CodeGenerator {
    fn new() -> Self {
        CodeGenerator {
            output: String::new(),
            indent_level: 0,
        }
    }

    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    fn write_line(&mut self, line: &str) {
        for _ in 0..self.indent_level {
            self.output.push_str("    ");
        }
        self.output.push_str(line);
        self.output.push('\n');
    }

    fn write(&mut self, text: &str) {
        self.output.push_str(text);
    }

    fn generate_impl(&mut self, input: &DeriveInput, trait_name: &str) {
        let generics_str = if input.generics.is_empty() {
            String::new()
        } else {
            format!("<{}>", input.generics.join(", "))
        };

        self.write_line(&format!("impl{} {} for {}{} {{", 
            generics_str, trait_name, input.name, generics_str));
        self.indent();
    }

    fn end_impl(&mut self) {
        self.dedent();
        self.write_line("}");
    }

    fn generate_function(&mut self, name: &str, params: &[(&str, &str)], return_type: &str) {
        let params_str: Vec<String> = params
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ty))
            .collect();

        self.write_line(&format!("fn {}({}) -> {} {{", 
            name, params_str.join(", "), return_type));
        self.indent();
    }

    fn end_function(&mut self) {
        self.dedent();
        self.write_line("}");
    }
}

struct DeriveMacro {
    name: String,
    generator: fn(&DeriveInput, &mut CodeGenerator) -> Result<(), String>,
}

impl DeriveMacro {
    fn new(name: String, generator: fn(&DeriveInput, &mut CodeGenerator) -> Result<(), String>) -> Self {
        DeriveMacro { name, generator }
    }

    fn expand(&self, input: &DeriveInput) -> Result<String, String> {
        let mut codegen = CodeGenerator::new();
        (self.generator)(input, &mut codegen)?;
        Ok(codegen.output)
    }
}

fn derive_debug(input: &DeriveInput, codegen: &mut CodeGenerator) -> Result<(), String> {
    codegen.generate_impl(input, "std::fmt::Debug");
    
    codegen.generate_function("fmt", &[("&self", ""), ("f", "&mut std::fmt::Formatter")], "std::fmt::Result");
    
    codegen.write_line(&format!("f.debug_struct(\"{}\")", input.name));
    codegen.indent();
    
    for field in &input.fields {
        codegen.write_line(&format!(".field(\"{}\", &self.{})", field.name, field.name));
    }
    
    codegen.write_line(".finish()");
    codegen.dedent();
    
    codegen.end_function();
    codegen.end_impl();
    
    Ok(())
}

fn derive_clone(input: &DeriveInput, codegen: &mut CodeGenerator) -> Result<(), String> {
    codegen.generate_impl(input, "Clone");
    
    codegen.generate_function("clone", &[("&self", "")], "Self");
    
    codegen.write_line(&format!("{} {{", input.name));
    codegen.indent();
    
    for field in &input.fields {
        codegen.write_line(&format!("{}: self.{}.clone(),", field.name, field.name));
    }
    
    codegen.dedent();
    codegen.write_line("}");
    
    codegen.end_function();
    codegen.end_impl();
    
    Ok(())
}

fn derive_default(input: &DeriveInput, codegen: &mut CodeGenerator) -> Result<(), String> {
    codegen.generate_impl(input, "Default");
    
    codegen.generate_function("default", &[], "Self");
    
    codegen.write_line(&format!("{} {{", input.name));
    codegen.indent();
    
    for field in &input.fields {
        codegen.write_line(&format!("{}: Default::default(),", field.name));
    }
    
    codegen.dedent();
    codegen.write_line("}");
    
    codegen.end_function();
    codegen.end_impl();
    
    Ok(())
}

fn derive_serialize(input: &DeriveInput, codegen: &mut CodeGenerator) -> Result<(), String> {
    codegen.generate_impl(input, "Serialize");
    
    codegen.generate_function("serialize", &[("&self", ""), ("serializer", "S")], "Result<S::Ok, S::Error>");
    codegen.write_line("where S: Serializer");
    
    codegen.write_line(&format!("let mut state = serializer.serialize_struct(\"{}\", {})?;", 
        input.name, input.fields.len()));
    
    for field in &input.fields {
        codegen.write_line(&format!("state.serialize_field(\"{}\", &self.{})?;", 
            field.name, field.name));
    }
    
    codegen.write_line("state.end()");
    
    codegen.end_function();
    codegen.end_impl();
    
    Ok(())
}

struct MacroRegistry {
    macros: HashMap<String, DeriveMacro>,
}

impl MacroRegistry {
    fn new() -> Self {
        let mut registry = MacroRegistry {
            macros: HashMap::new(),
        };
        
        registry.register(DeriveMacro::new("Debug".to_string(), derive_debug));
        registry.register(DeriveMacro::new("Clone".to_string(), derive_clone));
        registry.register(DeriveMacro::new("Default".to_string(), derive_default));
        registry.register(DeriveMacro::new("Serialize".to_string(), derive_serialize));
        
        registry
    }

    fn register(&mut self, macro_def: DeriveMacro) {
        self.macros.insert(macro_def.name.clone(), macro_def);
    }

    fn expand(&self, trait_name: &str, input: &DeriveInput) -> Result<String, String> {
        self.macros
            .get(trait_name)
            .ok_or_else(|| format!("Unknown derive macro: {}", trait_name))?
            .expand(input)
    }
}

struct AttributeMacro {
    name: String,
    transformer: fn(&DeriveInput, &[Token]) -> Result<String, String>,
}

impl AttributeMacro {
    fn new(name: String, transformer: fn(&DeriveInput, &[Token]) -> Result<String, String>) -> Self {
        AttributeMacro { name, transformer }
    }

    fn transform(&self, input: &DeriveInput, args: &[Token]) -> Result<String, String> {
        (self.transformer)(input, args)
    }
}

fn builder_macro(input: &DeriveInput, _args: &[Token]) -> Result<String, String> {
    let mut codegen = CodeGenerator::new();
    
    let builder_name = format!("{}Builder", input.name);
    
    codegen.write_line(&format!("struct {} {{", builder_name));
    codegen.indent();
    
    for field in &input.fields {
        let ty_str = field.ty.path.join("::");
        codegen.write_line(&format!("{}: Option<{}>,", field.name, ty_str));
    }
    
    codegen.dedent();
    codegen.write_line("}");
    codegen.write_line("");
    
    codegen.write_line(&format!("impl {} {{", builder_name));
    codegen.indent();
    
    codegen.write_line("fn new() -> Self {");
    codegen.indent();
    codegen.write_line(&format!("{} {{", builder_name));
    codegen.indent();
    
    for field in &input.fields {
        codegen.write_line(&format!("{}: None,", field.name));
    }
    
    codegen.dedent();
    codegen.write_line("}");
    codegen.dedent();
    codegen.write_line("}");
    codegen.write_line("");
    
    for field in &input.fields {
        let ty_str = field.ty.path.join("::");
        codegen.write_line(&format!("fn {}(mut self, value: {}) -> Self {{", field.name, ty_str));
        codegen.indent();
        codegen.write_line(&format!("self.{} = Some(value);", field.name));
        codegen.write_line("self");
        codegen.dedent();
        codegen.write_line("}");
        codegen.write_line("");
    }
    
    codegen.write_line(&format!("fn build(self) -> Result<{}, String> {{", input.name));
    codegen.indent();
    codegen.write_line(&format!("Ok({} {{", input.name));
    codegen.indent();
    
    for field in &input.fields {
        codegen.write_line(&format!("{}: self.{}.ok_or(\"{} is required\")?,", 
            field.name, field.name, field.name));
    }
    
    codegen.dedent();
    codegen.write_line("})");
    codegen.dedent();
    codegen.write_line("}");
    
    codegen.dedent();
    codegen.write_line("}");
    
    Ok(codegen.output)
}

fn getters_setters_macro(input: &DeriveInput, _args: &[Token]) -> Result<String, String> {
    let mut codegen = CodeGenerator::new();
    
    codegen.write_line(&format!("impl {} {{", input.name));
    codegen.indent();
    
    for field in &input.fields {
        let ty_str = field.ty.path.join("::");
        
        codegen.write_line(&format!("pub fn {}(&self) -> &{} {{", field.name, ty_str));
        codegen.indent();
        codegen.write_line(&format!("&self.{}", field.name));
        codegen.dedent();
        codegen.write_line("}");
        codegen.write_line("");
        
        codegen.write_line(&format!("pub fn set_{}(&mut self, value: {}) {{", field.name, ty_str));
        codegen.indent();
        codegen.write_line(&format!("self.{} = value;", field.name));
        codegen.dedent();
        codegen.write_line("}");
        codegen.write_line("");
    }
    
    codegen.dedent();
    codegen.write_line("}");
    
    Ok(codegen.output)
}