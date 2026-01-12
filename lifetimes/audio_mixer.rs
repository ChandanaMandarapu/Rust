use std::marker::PhantomData;

pub struct AudioBuffer<'a> {
    samples: &'a [f32],
    channels: usize,
    sample_rate: u32,
}

pub struct MutAudioBuffer<'a> {
    samples: &'a mut [f32],
    channels: usize,
    sample_rate: u32,
}

pub trait AudioProcessor<'a> {
    fn process(&mut self, input: &AudioBuffer<'a>, output: &mut MutAudioBuffer<'a>);
}

pub struct GainProcessor<'a> {
    gain: f32,
    _marker: PhantomData<&'a ()>,
}

impl<'a> GainProcessor<'a> {
    pub fn new(gain: f32) -> Self {
        Self { gain, _marker: PhantomData }
    }
}

impl<'a> AudioProcessor<'a> for GainProcessor<'a> {
    fn process(&mut self, input: &AudioBuffer<'a>, output: &mut MutAudioBuffer<'a>) {
        let len = std::cmp::min(input.samples.len(), output.samples.len());
        for i in 0..len {
            output.samples[i] = input.samples[i] * self.gain;
        }
    }
}

pub struct Mixer<'a> {
    inputs: Vec<&'a AudioBuffer<'a>>,
}

impl<'a> Mixer<'a> {
    pub fn mix(&self, output: &mut MutAudioBuffer<'a>) {
        output.samples.fill(0.0);
        for input in &self.inputs {
            let len = std::cmp::min(input.samples.len(), output.samples.len());
            for i in 0..len {
                output.samples[i] += input.samples[i];
            }
        }
    }
}

pub struct AudioStream<'a> {
    callback: Box<dyn FnMut(&mut MutAudioBuffer<'a>) + 'a>, 
}

pub struct AudioContext<'a> {
    sample_rate: u32,
    device_name: &'a str,
}

pub struct SourceNode<'a> {
    buffer: &'a AudioBuffer<'a>,
    pos: usize,
}

impl<'a> SourceNode<'a> {
    pub fn read(&mut self, frames: usize) -> Option<AudioBuffer<'a>> {
        if self.pos + frames <= self.buffer.samples.len() {
             let slice = &self.buffer.samples[self.pos..self.pos+frames];
             self.pos += frames;
             Some(AudioBuffer {
                 samples: slice,
                 channels: self.buffer.channels,
                 sample_rate: self.buffer.sample_rate,
             })
        } else {
            None
        }
    }
}

pub struct EffectInfo<'a> {
    name: &'a str,
    params: &'a [f32],
}

pub struct EffectChain<'a> {
    processors: Vec<Box<dyn AudioProcessor<'a> + 'a>>,
}

impl<'a> EffectChain<'a> {
    pub fn process(&mut self, input: &AudioBuffer<'a>, output: &mut MutAudioBuffer<'a>) {
        if let Some(first) = self.processors.first_mut() {
            first.process(input, output);
        }
    }
}

pub struct SampleIter<'a> {
    buffer: &'a AudioBuffer<'a>,
    idx: usize,
}

impl<'a> Iterator for SampleIter<'a> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.buffer.samples.len() {
            let s = self.buffer.samples[self.idx];
            self.idx += 1;
            Some(s)
        } else {
            None
        }
    }
}

pub struct ChannelSplitter<'a> {
    buffer: &'a AudioBuffer<'a>,
}

impl<'a> ChannelSplitter<'a> {
    pub fn get_channel(&self, ch: usize) -> AudioBuffer<'a> {
        AudioBuffer {
            samples: self.buffer.samples,
            channels: 1,
            sample_rate: self.buffer.sample_rate,
        }
    }
}

pub struct WavHeader<'a> {
    raw: &'a [u8],
}

impl<'a> WavHeader<'a> {
    pub fn format(&self) -> u16 {
        1
    }
}

pub struct SpectrumAnalysis<'a> {
    window: &'a [f32],
}

impl<'a> SpectrumAnalysis<'a> {
    pub fn compute(&self, input: &'a [f32]) -> Vec<f32> {
        vec![0.0; input.len()]
    }
}

pub struct ParameterAutomation<'a> {
    points: &'a [(f64, f32)], 
}

impl<'a> ParameterAutomation<'a> {
    pub fn value_at(&self, time: f64) -> f32 {
        0.0
    }
}

pub struct AudioGraph<'a> {
    nodes: Vec<Box<dyn AudioProcessor<'a> + 'a>>,
    connections: Vec<(usize, usize)>,
}

pub struct SharedBuffer<'a> {
    data: &'a [f32],
}

pub struct ReferenceMixer<'a> {
    master: &'a mut SharedBuffer<'a>,
    slaves: Vec<&'a SharedBuffer<'a>>,
}

pub struct MidiMessage<'a> {
    raw: &'a [u8],
}

pub struct MidiEvent<'a> {
    msg: MidiMessage<'a>,
    timestamp: u64,
}

pub struct Sequencer<'a> {
    events: Vec<MidiEvent<'a>>,
}

impl<'a> Sequencer<'a> {
    pub fn next(&self, time: u64) -> impl Iterator<Item = &MidiEvent<'a>> {
        self.events.iter().filter(move |e| e.timestamp == time)
    }
}

pub struct SynthVoice<'a> {
    wavetable: &'a [f32],
    phase: f32,
}

impl<'a> SynthVoice<'a> {
    pub fn render(&mut self) -> f32 {
        let val = self.wavetable[self.phase as usize];
        self.phase += 1.0;
        val
    }
}

pub struct Resampler<'a> {
    input: &'a AudioBuffer<'a>,
    ratio: f64,
}

pub struct FftPlan<'a> {
    twiddles: &'a [f32],
}

pub struct ConvolutionReverb<'a> {
    ir: &'a [f32],
}

fn main() {
    println!("Audio Mixer");
}
