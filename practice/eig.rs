use std::convert::TryInto;
use std::marker::PhantomData;

pub struct PacketBuffer<'a> {
    data: &'a [u8],
}

pub struct EthernetFrame<'a> {
    buffer: &'a PacketBuffer<'a>,
    dest: &'a [u8],
    src: &'a [u8],
    ethertype: u16,
    payload: PacketBuffer<'a>,
}

pub struct Ipv4Packet<'a> {
    buffer: &'a PacketBuffer<'a>,
    version: u8,
    ihr: u8,
    src_ip: &'a [u8],
    dst_ip: &'a [u8],
    payload: PacketBuffer<'a>,
}

pub struct TcpSegment<'a> {
    buffer: &'a PacketBuffer<'a>,
    src_port: u16,
    dst_port: u16,
    payload: &'a [u8],
}

impl<'a> PacketBuffer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn slice(&self, start: usize, len: usize) -> Option<PacketBuffer<'a>> {
        if start + len <= self.data.len() {
            Some(PacketBuffer {
                data: &self.data[start..start + len],
            })
        } else {
            None
        }
    }
    
    pub fn read_u16(&self, offset: usize) -> Option<u16> {
        if offset + 2 <= self.data.len() {
             Some(u16::from_be_bytes(self.data[offset..offset+2].try_into().unwrap()))
        } else {
            None
        }
    }
}

impl<'a> EthernetFrame<'a> {
    pub fn parse(buffer: PacketBuffer<'a>) -> Option<Self> {
        if buffer.data.len() < 14 { return None; }
        let dest = &buffer.data[0..6];
        let src = &buffer.data[6..12];
        let ethertype = u16::from_be_bytes(buffer.data[12..14].try_into().unwrap());
        let payload = buffer.slice(14, buffer.data.len() - 14)?;
        
        
        
        Some(EthernetFrame {
            buffer: &buffer, 
            dest,
            src,
            ethertype,
            payload,
        })
    }
}

pub struct EthFrame<'a> {
    raw: &'a [u8],
    payload: &'a [u8],
}

impl<'a> EthFrame<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { raw: data, payload: &data[14..] }
    }
}

pub struct PacketMetadata<'a> {
    timestamp: u64,
    source_interface: &'a str,
}

pub struct CapturedPacket<'a> {
    data: &'a [u8],
    meta: &'a PacketMetadata<'a>,
}

pub trait ProtocolParser<'a> {
    type Output;
    fn parse(input: &'a [u8]) -> Option<Self::Output>;
}

pub struct UdpParser;

pub struct UdpHeader<'a> {
    src: u16,
    dst: u16,
    payload: &'a [u8],
}

impl<'a> ProtocolParser<'a> for UdpParser {
    type Output = UdpHeader<'a>;
    fn parse(input: &'a [u8]) -> Option<Self::Output> {
         if input.len() < 8 { return None; }
         let src = u16::from_be_bytes(input[0..2].try_into().unwrap());
         let dst = u16::from_be_bytes(input[2..4].try_into().unwrap());
         Some(UdpHeader {
             src,
             dst,
             payload: &input[8..],
         })
    }
}

pub struct FlowKey<'a> {
    src_ip: &'a [u8],
    dst_ip: &'a [u8],
    src_port: u16,
    dst_port: u16,
    proto: u8,
}

pub struct Flowtracker<'a> {
    active_flows: HashMap<FlowKey<'a>, Vec<&'a [u8]>>,
}

impl<'a> Hash for FlowKey<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.src_ip.hash(state);
        self.dst_ip.hash(state);
        self.src_port.hash(state);
        self.dst_port.hash(state);
        self.proto.hash(state);
    }
}

impl<'a> PartialEq for FlowKey<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.src_ip == other.src_ip && 
        self.dst_ip == other.dst_ip &&
        self.src_port == other.src_port &&
        self.dst_port == other.dst_port &&
        self.proto == other.proto
    }
}
impl<'a> Eq for FlowKey<'a> {}

pub struct DpiRule<'a> {
    pattern: &'a [u8],
    name: &'a str,
}

pub struct DpiEngine<'a> {
    rules: Vec<DpiRule<'a>>,
}

impl<'a> DpiEngine<'a> {
    pub fn scan(&self, payload: &'a [u8]) -> Option<&'a str> {
        for rule in &self.rules {
            if payload.windows(rule.pattern.len()).any(|w| w == rule.pattern) {
                return Some(rule.name);
            }
        }
        None
    }
}

pub struct Reassembler<'a> {
    fragments: HashMap<u32, Vec<&'a [u8]>>,
}

impl<'a> Reassembler<'a> {
    pub fn add_fragment(&mut self, id: u32, data: &'a [u8]) {
        self.fragments.entry(id).or_insert_with(Vec::new).push(data);
    }
}

pub struct PacketStream<'a> {
    source: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for PacketStream<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + 4 <= self.source.len() {
            let len = u32::from_be_bytes(self.source[self.pos..self.pos+4].try_into().unwrap()) as usize;
            self.pos += 4;
            if self.pos + len <= self.source.len() {
                let pkt = &self.source[self.pos..self.pos+len];
                self.pos += len;
                Some(pkt)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct LayerStack<'a> {
    l2: Option<EthFrame<'a>>,
    l3: Option<Ipv4Packet<'a>>, 
    l4: Option<TcpSegment<'a>>,
}

pub struct AnalyzerContext<'a> {
    filter: fn(&'a [u8]) -> bool,
    capture_limit: usize,
}

pub struct FilteredPacketIter<'a, I> 
where I: Iterator<Item = &'a [u8]>
{
    iter: I,
    ctx: &'a AnalyzerContext<'a>,
}

impl<'a, I> Iterator for FilteredPacketIter<'a, I> 
where I: Iterator<Item = &'a [u8]>
{
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.iter.next()?;
            if (self.ctx.filter)(item) {
                return Some(item);
            }
        }
    }
}

pub struct MultiSourceCapture<'a> {
    sources: Vec<&'a [u8]>,
}

impl<'a> MultiSourceCapture<'a> {
    pub fn iter(&'a self) -> impl Iterator<Item = &'a u8> {
        self.sources.iter().flat_map(|s| s.iter())
    }
}

pub struct PacketRef<'a> {
    ptr: *const u8,
    len: usize,
    _marker: PhantomData<&'a [u8]>,
}

impl<'a> PacketRef<'a> {
    pub unsafe fn from_raw(ptr: *const u8, len: usize) -> Self {
        Self { ptr, len, _marker: PhantomData }
    }
    
    pub fn as_slice(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

use std::hash::Hash;

fn main() {
    println!("Packet Analyzer");
}
