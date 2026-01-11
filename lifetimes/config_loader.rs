use std::borrow::Cow;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct ConfigSource<'a> {
    raw_text: &'a str,
}

pub struct ConfigValue<'a> {
    source: &'a ConfigSource<'a>,
    range: (usize, usize),
}

impl<'a> ConfigValue<'a> {
    pub fn as_str(&self) -> &'a str {
        &self.source.raw_text[self.range.0..self.range.1]
    }
}

pub struct Section<'a> {
    name: &'a str,
    values: HashMap<&'a str, ConfigValue<'a>>,
}

pub struct Configuration<'a> {
    sections: HashMap<&'a str, Section<'a>>,
    defaults: Option<&'a Configuration<'a>>,
}

impl<'a> Configuration<'a> {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
            defaults: None,
        }
    }

    pub fn set_defaults(&mut self, defaults: &'a Configuration<'a>) {
        self.defaults = Some(defaults);
    }

    pub fn get_value(&self, section: &str, key: &str) -> Option<&'a str> {
        if let Some(sec) = self.sections.get(section) {
            if let Some(val) = sec.values.get(key) {
                return Some(val.as_str());
            }
        }
        
        if let Some(def) = self.defaults {
            def.get_value(section, key)
        } else {
            None
        }
    }
    
    pub fn get_merged<'b>(&'b self, other: &'b Configuration<'a>) -> MergedConfig<'a, 'b> {
        MergedConfig {
            primary: self,
            secondary: other,
        }
    }
}

pub struct MergedConfig<'a, 'b> {
    primary: &'b Configuration<'a>,
    secondary: &'b Configuration<'a>,
}

impl<'a, 'b> MergedConfig<'a, 'b> {
    pub fn get(&self, section: &str, key: &str) -> Option<&'a str> {
        self.primary.get_value(section, key).or_else(|| self.secondary.get_value(section, key))
    }
}

pub struct EnvVarOverride<'a> {
    vars: HashMap<String, String>,
    base: &'a Configuration<'a>,
}

impl<'a> EnvVarOverride<'a> {
    
    pub fn get(&self, section: &str, key: &str) -> Option<Cow<str>> {
        let env_key = format!("{}_{}", section.to_uppercase(), key.to_uppercase());
        if let Some(val) = self.vars.get(&env_key) {
            Some(Cow::Owned(val.clone()))
        } else {
            self.base.get_value(section, key).map(Cow::Borrowed)
        }
    }
}

pub struct ConfigParser<'a> {
    source: &'a ConfigSource<'a>,
}

impl<'a> ConfigParser<'a> {
    pub fn parse(&self) -> Configuration<'a> {
        Configuration::new()
    }
}

pub struct CommandLineArgs<'a> {
    args: Vec<&'a str>,
}

pub struct AppContext<'a> {
    config: &'a Configuration<'a>,
    args: &'a CommandLineArgs<'a>,
}

impl<'a> AppContext<'a> {
    pub fn get_arg(&self, idx: usize) -> Option<&'a str> {
        self.args.args.get(idx).copied()
    }
    
    pub fn get_cfg(&self, s: &str, k: &str) -> Option<&'a str> {
        self.config.get_value(s, k)
    }
}

pub struct DynamicReloader<'a> {
    current: Configuration<'a>,
    parser: ConfigParser<'a>,
}

impl<'a> DynamicReloader<'a> {
    pub fn reload(&mut self) {
        self.current = self.parser.parse();
    }
    
    pub fn get(&self) -> &Configuration<'a> {
        &self.current
    }
}

pub struct KeyIterator<'a> {
    config: &'a Configuration<'a>,
    section_iter: std::collections::hash_map::Keys<'a, &'a str, Section<'a>>,
    current_key_iter: Option<std::collections::hash_map::Keys<'a, &'a str, ConfigValue<'a>>>,
}

impl<'a> Iterator for KeyIterator<'a> {
    type Item = (&'a str, &'a str);
    fn next(&mut self) -> Option<Self::Item> {
        None 
    }
}

pub struct RecursiveConfig<'a> {
    parent: Option<&'a RecursiveConfig<'a>>,
    values: HashMap<&'a str, &'a str>,
}

impl<'a> RecursiveConfig<'a> {
    pub fn resolve(&self, key: &str) -> Option<&'a str> {
        if let Some(&v) = self.values.get(key) {
            Some(v)
        } else {
            self.parent.and_then(|p| p.resolve(key))
        }
    }
}

pub trait ConfigVisitor<'a> {
    fn visit_section(&mut self, name: &'a str);
    fn visit_value(&mut self, key: &'a str, value: &'a str);
}

pub struct PrintingVisitor;
impl<'a> ConfigVisitor<'a> for PrintingVisitor {
    fn visit_section(&mut self, name: &'a str) {
        println!("[{}]", name);
    }
    fn visit_value(&mut self, key: &'a str, value: &'a str) {
        println!("{} = {}", key, value);
    }
}

pub fn traverse<'a>(config: &'a Configuration<'a>, visitor: &mut dyn ConfigVisitor<'a>) {
    for (name, sec) in &config.sections {
        visitor.visit_section(name);
        for (k, v) in &sec.values {
            visitor.visit_value(k, v.as_str());
        }
    }
}

pub struct PathResolver<'a> {
    base_dir: &'a str,
}

impl<'a> PathResolver<'a> {
    pub fn resolve(&self, path: &'a str) -> Cow<'a, str> {
        if path.starts_with("/") {
            Cow::Borrowed(path)
        } else {
            Cow::Owned(format!("{}/{}", self.base_dir, path))
        }
    }
}

pub struct Validator<'a> {
    rules: Vec<Box<dyn Fn(&'a str) -> bool + 'a>>,
}

impl<'a> Validator<'a> {
    pub fn validate(&self, val: &'a str) -> bool {
        self.rules.iter().all(|f| f(val))
    }
}

pub struct TypedConfig<'a, T> {
    val: T,
    origin: &'a str,
}

pub struct ConfigLayer1<'a>(&'a str);
pub struct ConfigLayer2<'a>(&'a str);
pub struct ConfigLayer3<'a>(&'a str);

pub struct NestedLayers<'a> {
    l1: ConfigLayer1<'a>,
    l2: ConfigLayer2<'a>,
    l3: ConfigLayer3<'a>,
}

impl<'a> NestedLayers<'a> {
    pub fn get_deepest(&self) -> &'a str {
        self.l3.0
    }
}

fn main() {
    let src = ConfigSource { raw_text: "key=val" };
    println!("Config");
}
