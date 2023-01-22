mod synth;

pub trait Track {
  fn play(&mut self, ts: f64, buff: &mut [f32], sample_rate: f64);
  fn is_done(&self) -> bool;
}

pub use synth::SynthTrack;
