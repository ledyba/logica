mod synth;

pub trait Track {
  fn play(&mut self, sample_rate: f64, buff: &mut [f32], start_idx: usize);
  fn is_done(&self) -> bool;
}

pub use synth::SynthTrack;
