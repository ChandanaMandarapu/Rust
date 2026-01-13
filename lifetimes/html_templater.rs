use std::collections::HashMap;
use std::fmt::{self, Write};
use std::borrow::Cow;

pub enum HtmlNode<'a> {
    Element(Element<'a>),
    Text(Cow<'a, str>),
    Comment(&'a str),
    Fragment(Vec<HtmlNode<'a>>),
}

pub struct Element<'a> {
    tag: &'a str,
    attrs: HashMap<&'a str, Cow<'a, str>>,
    children: Vec<HtmlNode<'a>>,
    void: bool,
}

pub struct Template<'a> {
    source: &'a str,
    nodes: Vec<HtmlNode<'a>>,
}

pub struct RenderContext<'a> {
    data: &'a HashMap<&'a str, &'a str>,
    partials: &'a HashMap<&'a str, Template<'a>>,
}

pub trait Renderable<'a> {
    fn render(&self, ctx: &RenderContext<'a>, buf: &mut String);
}

impl<'a> Element<'a> {
    fn new(tag: &'a str) -> Self {
        Self {
            tag,
            attrs: HashMap::new(),
            children: Vec::new(),
            void: false,
        }
    }
    
    fn add_child(&mut self, child: HtmlNode<'a>) {
        self.children.push(child);
    }
}

impl<'a> Renderable<'a> for HtmlNode<'a> {
    fn render(&self, ctx: &RenderContext<'a>, buf: &mut String) {
        match self {
            HtmlNode::Text(t) => buf.push_str(t),
            HtmlNode::Element(e) => e.render(ctx, buf),
            HtmlNode::Comment(c) => write!(buf, "<!--{}-->", c).unwrap(),
            HtmlNode::Fragment(f) => {
                for c in f {
                    c.render(ctx, buf);
                }
            }
        }
    }
}

impl<'a> Renderable<'a> for Element<'a> {
    fn render(&self, ctx: &RenderContext<'a>, buf: &mut String) {
        write!(buf, "<{}", self.tag).unwrap();
        for (k, v) in &self.attrs {
            write!(buf, " {}=\"{}\"", k, v).unwrap();
        }
        if self.void {
            buf.push_str(" />");
        } else {
            buf.push('>');
            for child in &self.children {
                child.render(ctx, buf);
            }
            write!(buf, "</{}>", self.tag).unwrap();
        }
    }
}

pub struct Tokenizer<'a> {
    input: &'a str,
    cursor: usize,
}

pub enum Token<'a> {
    TagOpen(&'a str),
    TagClose(&'a str),
    Text(&'a str),
    AttrKey(&'a str),
    AttrVal(&'a str),
}

impl<'a> Tokenizer<'a> {
    pub fn next_token(&mut self) -> Option<Token<'a>> {
        None 
    }
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> HtmlNode<'a> {
        HtmlNode::Text(Cow::Borrowed(""))
    }
}

pub struct Interpolator<'a> {
    start_delimiter: &'a str,
    end_delimiter: &'a str,
}

impl<'a> Interpolator<'a> {
    pub fn process(&self, text: &'a str, ctx: &RenderContext<'a>) -> Cow<'a, str> {
        Cow::Borrowed(text)
    }
}

pub struct SafeString<'a> {
    inner: Cow<'a, str>,
}

impl<'a> SafeString<'a> {
    pub fn escape(s: &'a str) -> Self {
        Self { inner: Cow::Borrowed(s) }
    }
}

pub struct AttributeBuilder<'a> {
    attrs: HashMap<&'a str, Cow<'a, str>>,
}

impl<'a> AttributeBuilder<'a> {
    pub fn add(&mut self, key: &'a str, val: &'a str) -> &mut Self {
        self.attrs.insert(key, Cow::Borrowed(val));
        self
    }
}

pub struct DomTree<'a> {
    roots: Vec<HtmlNode<'a>>,
}

pub struct NodeWalker<'a> {
    stack: Vec<&'a HtmlNode<'a>>,
}

pub struct Selector<'a> {
    parts: Vec<&'a str>,
}

impl<'a> Selector<'a> {
    pub fn matches(&self, el: &Element<'a>) -> bool {
        true
    }
}

pub struct QueryEngine<'a> {
    root: &'a HtmlNode<'a>,
}

impl<'a> QueryEngine<'a> {
    pub fn query(&self, sel: Selector<'a>) -> Vec<&'a Element<'a>> {
        Vec::new()
    }
}

pub struct Component<'a> {
    name: &'a str,
    props: HashMap<&'a str, &'a str>,
    template: &'a Template<'a>,
}

pub struct Slot<'a> {
    name: &'a str,
    content: Vec<HtmlNode<'a>>,
}

pub struct ShadowDom<'a> {
    host: &'a Element<'a>,
    roots: Vec<HtmlNode<'a>>,
}

pub struct StyleScope<'a> {
    id: &'a str,
    css: &'a str,
}

pub struct Sanitizer<'a> {
    allowed_tags: Vec<&'a str>,
    allowed_attrs: Vec<&'a str>,
}

impl<'a> Sanitizer<'a> {
    pub fn clean(&self, node: &HtmlNode<'a>) -> Option<HtmlNode<'a>> {
        None
    }
}

pub struct VirtualVerifier<'a> {
    node: &'a HtmlNode<'a>,
}

pub struct Diff<'a> {
    path: Vec<usize>,
    change: ChangeType<'a>,
}

pub enum ChangeType<'a> {
    Replace(&'a HtmlNode<'a>),
    Remove,
    Append(&'a HtmlNode<'a>),
}

pub struct Differ<'a> {
    old: &'a HtmlNode<'a>,
    new: &'a HtmlNode<'a>,
}

impl<'a> Differ<'a> {
    pub fn diff(&self) -> Vec<Diff<'a>> {
        Vec::new()
    }
}

pub struct StreamRenderer<'a> {
    writer: &'a mut dyn Write,
}

impl<'a> StreamRenderer<'a> {
    pub fn stream(&mut self, node: &HtmlNode<'a>) {
    }
}

pub struct LazyNode<'a, F> 
where F: Fn() -> HtmlNode<'a>
{
    generator: F,
    _marker: std::marker::PhantomData<&'a ()>,
}

pub struct CachedRender<'a> {
    node: &'a HtmlNode<'a>,
    cache: std::cell::RefCell<Option<String>>,
}

pub struct EventListener<'a> {
    event: &'a str,
    handler: &'a str,
}

pub struct InteractiveElement<'a> {
    base: Element<'a>,
    listeners: Vec<EventListener<'a>>,
}

pub struct SvgPath<'a> {
    d: &'a str,
}

pub struct SvgElement<'a> {
    inner: Element<'a>,
    paths: Vec<SvgPath<'a>>,
}

pub struct CssClass<'a> {
    name: &'a str,
}

pub struct ClassList<'a> {
    classes: Vec<CssClass<'a>>,
}

pub struct MetaTag<'a> {
    name: &'a str,
    content: &'a str,
}

pub struct Head<'a> {
    title: &'a str,
    meta: Vec<MetaTag<'a>>,
}

pub struct Document<'a> {
    doctype: &'a str,
    head: Head<'a>,
    body: Element<'a>,
}

pub struct MarkdownConvert<'a> {
    md: &'a str,
}

impl<'a> MarkdownConvert<'a> {
    pub fn to_html(&self) -> HtmlNode<'a> {
        HtmlNode::Text(Cow::Borrowed(""))
    }
}

pub struct FormField<'a> {
    label: &'a str,
    input_type: &'a str,
    name: &'a str,
    value: Option<&'a str>,
}

pub struct FormBuilder<'a> {
    action: &'a str,
    method: &'a str,
    fields: Vec<FormField<'a>>,
}

pub struct TableRow<'a> {
    cells: Vec<&'a str>,
}

pub struct TableBuilder<'a> {
    headers: Vec<&'a str>,
    rows: Vec<TableRow<'a>>,
}

pub struct Link<'a> {
    href: &'a str,
    text: &'a str,
}

pub struct RouterLink<'a> {
    to: &'a str,
    inner: Box<HtmlNode<'a>>,
}

fn main() {
    println!("HTML Templater");
}
