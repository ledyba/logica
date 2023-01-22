pub struct SynthTrack {
  freq: f64,
}

impl SynthTrack {
  pub fn new(synth: &crate::synth::Synth) -> Self {
    Self {
      freq: synth.freq,
    }
  }
}

impl super::Track for SynthTrack {
  fn play(&mut self, ts: f64, buff: &mut [f32], sample_rate: f64) {
    for (idx, d) in (0..buff.len()).zip(buff.iter_mut()) {
      *d = ((ts + (idx as f64 / sample_rate)) * self.freq).sin() as f32;
    }
  }

  fn is_done(&self) -> bool {
    false
  }
}
