use std::marker::PhantomData;

pub struct Theme<'a> {
    pub primary_color: &'a str,
    pub secondary_color: &'a str,
    pub font: &'a str,
}

pub struct Context<'a> {
    pub theme: &'a Theme<'a>,
    pub window_width: u32,
    pub window_height: u32,
}

pub trait Component<'a> {
    fn draw(&self, ctx: &Context<'a>);
    fn measure(&self, ctx: &Context<'a>) -> (u32, u32);
}

pub struct Button<'a> {
    label: &'a str,
    style: &'a str, 
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str, style: &'a str) -> Self {
        Self { label, style }
    }
}

impl<'a> Component<'a> for Button<'a> {
    fn draw(&self, ctx: &Context<'a>) {
        println!("Drawing button {} with theme {}", self.label, ctx.theme.primary_color);
    }

    fn measure(&self, _ctx: &Context<'a>) -> (u32, u32) {
        (100, 40)
    }
}

pub struct Text<'a> {
    content: &'a str,
}

impl<'a> Component<'a> for Text<'a> {
    fn draw(&self, ctx: &Context<'a>) {
        println!("Text: {} in font {}", self.content, ctx.theme.font);
    }
    
    fn measure(&self, _ctx: &Context<'a>) -> (u32, u32) {
        (self.content.len() as u32 * 10, 20)
    }
}

pub struct Container<'a> {
    children: Vec<Box<dyn Component<'a> + 'a>>,
    layout: &'a str,
}

impl<'a> Container<'a> {
    pub fn new(layout: &'a str) -> Self {
        Self {
            children: Vec::new(),
            layout,
        }
    }
    
    pub fn add_child(&mut self, child: Box<dyn Component<'a> + 'a>) {
        self.children.push(child);
    }
}

impl<'a> Component<'a> for Container<'a> {
    fn draw(&self, ctx: &Context<'a>) {
        println!("Container start layout {}", self.layout);
        for child in &self.children {
            child.draw(ctx);
        }
        println!("Container end");
    }
    
    fn measure(&self, ctx: &Context<'a>) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;
        for child in &self.children {
            let (w, h) = child.measure(ctx);
            width = std::cmp::max(width, w);
            height += h;
        }
        (width, height)
    }
}

pub struct RefWidget<'a, T: Component<'a>> {
    widget: &'a T,
}

impl<'a, T: Component<'a>> Component<'a> for RefWidget<'a, T> {
    fn draw(&self, ctx: &Context<'a>) {
        self.widget.draw(ctx);
    }
    fn measure(&self, ctx: &Context<'a>) -> (u32, u32) {
        self.widget.measure(ctx)
    }
}

pub struct ScrollView<'a> {
    content: Box<dyn Component<'a> + 'a>,
    scroll_pos: &'a mut usize,
}

impl<'a> Component<'a> for ScrollView<'a> {
    fn draw(&self, ctx: &Context<'a>) {
        self.content.draw(ctx);
    }
    fn measure(&self, ctx: &Context<'a>) -> (u32, u32) {
        self.content.measure(ctx)
    }
}

pub struct EventListener<'a, F> 
where F: Fn(&'a str) 
{
    callback: F,
    _marker: PhantomData<&'a ()>,
}

pub struct ButtonGroup<'a> {
    buttons: Vec<Button<'a>>,
    selected: Option<&'a Button<'a>>,
}

impl<'a> ButtonGroup<'a> {
    pub fn select(&mut self, index: usize) {
        if let Some(btn) = self.buttons.get(index) {
        }
    }
}

pub struct ExternalButtonGroup<'a> {
    buttons: &'a [Button<'a>],
    active: Option<&'a Button<'a>>,
}

impl<'a> ExternalButtonGroup<'a> {
    pub fn set_active(&mut self, idx: usize) {
        if let Some(btn) = self.buttons.get(idx) {
            self.active = Some(btn);
        }
    }
}

pub struct TreePath<'a> {
    segments: Vec<&'a str>,
}

pub struct TreeView<'a> {
    root: &'a str,
    expanded: Vec<TreePath<'a>>,
}

pub struct Canvas<'a> {
    buffer: &'a mut [u32],
    width: usize,
    height: usize,
}

impl<'a> Canvas<'a> {
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }
    
    pub fn sub_canvas<'b>(&'b mut self, x: usize, y: usize, w: usize, h: usize) -> Canvas<'b> {
        Canvas {
            buffer: self.buffer, 
            width: w,
            height: h, 
        }
    }
}

pub struct RenderPipeline<'a> {
    layers: Vec<&'a dyn Component<'a>>,
}

impl<'a> RenderPipeline<'a> {
    pub fn render(&self, ctx: &Context<'a>) {
        for layer in &self.layers {
            layer.draw(ctx);
        }
    }
}

pub struct AssetLoader<'a> {
    base_path: &'a str,
}

pub struct ImageAsset<'a> {
    data: &'a [u8],
    source: &'a AssetLoader<'a>,
}

pub struct FontAsset<'a> {
    glyphs: HashMap<char, &'a [u8]>,
    name: &'a str,
}

pub struct StyleEngine<'a> {
    styles: HashMap<&'a str, HashMap<&'a str, &'a str>>,
}

impl<'a> StyleEngine<'a> {
    pub fn get_style(&self, class: &str, prop: &str) -> Option<&'a str> {
        self.styles.get(class).and_then(|m| m.get(prop)).copied()
    }
}

pub struct LayoutConstraint<'a> {
    min_width: &'a u32,
    max_width: &'a u32,
}

pub struct FlexBox<'a> {
    items: Vec<&'a dyn Component<'a>>,
    direction: &'a str,
}

pub struct Grid<'a> {
    cells: Vec<Vec<&'a dyn Component<'a>>>,
}

pub struct Modal<'a> {
    content: &'a dyn Component<'a>,
    overlay_color: &'a str,
}

pub struct Tooltip<'a> {
    text: &'a str,
    target: &'a dyn Component<'a>,
}

pub struct ValidatedInput<'a> {
    value: &'a str,
    validator: fn(&'a str) -> bool,
}

impl<'a> ValidatedInput<'a> {
    pub fn is_valid(&self) -> bool {
        (self.validator)(self.value)
    }
}

pub struct Checkbox<'a> {
    label: &'a str,
    checked: &'a mut bool,
}

impl<'a> Checkbox<'a> {
    pub fn toggle(&mut self) {
        *self.checked = !*self.checked;
    }
}

pub struct RadioGroup<'a, T> {
    options: &'a [T],
    selected: &'a mut Option<usize>,
}

impl<'a, T> RadioGroup<'a, T> {
    pub fn select(&mut self, idx: usize) {
        if idx < self.options.len() {
            *self.selected = Some(idx);
        }
    }
}

pub struct Menu<'a> {
    items: Vec<MenuItem<'a>>,
}

pub struct MenuItem<'a> {
    label: &'a str,
    action: &'a dyn Fn(),
    submenu: Option<&'a Menu<'a>>,
}

fn main() {
    println!("UI Tree");
}
