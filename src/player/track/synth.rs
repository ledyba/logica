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
  fn play(&mut self, sample_rate: f64, buff: &mut [f32], start_idx: usize) {
    use std::f64::consts::PI;
    let k = PI * 2.0 * self.freq;
    for (idx, d) in (start_idx..start_idx + buff.len()).zip(buff.iter_mut()) {
      *d = ((idx as f64 * k / sample_rate).sin() * 0.8) as f32;
    }
  }

  fn is_done(&self) -> bool {
    false
  }
}
