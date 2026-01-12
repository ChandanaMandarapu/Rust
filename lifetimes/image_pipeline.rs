use std::marker::PhantomData;

pub struct Pixel(u8, u8, u8);

pub struct Image<'a> {
    data: &'a [u8],
    width: u32,
    height: u32,
}

pub struct MutImage<'a> {
    data: &'a mut [u8],
    width: u32,
    height: u32,
}

pub trait Filter<'a> {
    fn apply(&self, src: &Image<'a>, dst: &mut MutImage<'a>);
}

pub struct GrayscaleFilter;
impl<'a> Filter<'a> for GrayscaleFilter {
    fn apply(&self, src: &Image<'a>, dst: &mut MutImage<'a>) {
    }
}

pub struct Kernel<'a> {
    weights: &'a [f32],
    size: u32,
}

pub struct Convolution<'a> {
    kernel: &'a Kernel<'a>,
}

impl<'a> Filter<'a> for Convolution<'a> {
    fn apply(&self, src: &Image<'a>, dst: &mut MutImage<'a>) {
    }
}

pub struct ImageRegion<'a> {
    image: &'a Image<'a>,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl<'a> ImageRegion<'a> {
    pub fn pixel_at(&self, x: u32, y: u32) -> Option<&'a u8> {
        None
    }
}

pub struct Pipeline<'a> {
    filters: Vec<Box<dyn Filter<'a> + 'a>>,
}

impl<'a> Pipeline<'a> {
    pub fn run(&self, src: &Image<'a>, dst: &mut MutImage<'a>) {
        for f in &self.filters {
            f.apply(src, dst);
        }
    }
}

pub struct Histogram<'a> {
    buckets: &'a mut [u32; 256],
}

impl<'a> Histogram<'a> {
    pub fn compute(img: &Image<'a>, buckets: &'a mut [u32; 256]) -> Self {
        for b in img.data {
             buckets[*b as usize] += 1;
        }
        Self { buckets }
    }
}

pub struct TextureAtlas<'a> {
    image: &'a Image<'a>,
    regions: std::collections::HashMap<String, ImageRegion<'a>>,
}

pub struct Sprite<'a> {
    texture: &'a ImageRegion<'a>,
}

pub struct FrameBuffer<'a> {
    buffer: &'a mut [u32],
}

pub struct GlyphCache<'a> {
    atlas: &'a mut TextureAtlas<'a>,
}

pub struct RawDecoder<'a> {
    bytes: &'a [u8],
}

impl<'a> RawDecoder<'a> {
    pub fn decode(&self) -> Image<'a> {
        Image { data: self.bytes, width: 0, height: 0 }
    }
}

pub struct AsyncLoader<'a> {
    path: &'a str,
    callback: Box<dyn FnOnce(Image<'a>) + 'a>, 
}

pub struct Layer<'a> {
    img: &'a Image<'a>,
    opacity: f32,
}

pub struct Compositor<'a> {
    layers: Vec<Layer<'a>>,
}

impl<'a> Compositor<'a> {
    pub fn composite(&self, out: &mut MutImage<'a>) {
    }
}

pub struct PaintBrush<'a> {
    color: &'a Pixel,
    shape: &'a [bool], 
}

pub struct CanvasState<'a> {
    history: Vec<&'a Image<'a>>,
}

pub struct ImageView<'a> {
    source: &'a Image<'a>,
    zoom: f32,
}

pub struct Lut<'a> {
    table: &'a [u8; 256],
}

impl<'a> Filter<'a> for Lut<'a> {
    fn apply(&self, _src: &Image<'a>, _dst: &mut MutImage<'a>) {}
}

pub struct ColorProfile<'a> {
    name: &'a str,
    curve: &'a [f32],
}

pub struct ImageMetadata<'a> {
    exif: std::collections::HashMap<&'a str, &'a str>,
}

pub struct TiffTag<'a> {
    id: u16,
    data: &'a [u8],
}

pub struct TiffReader<'a> {
    tags: Vec<TiffTag<'a>>,
}

fn main() {
    println!("Image");
}
