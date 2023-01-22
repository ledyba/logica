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
  fn play(&mut self, config: &cpal::StreamConfig, buff: &mut [f32], start_idx: usize) {
    use std::f64::consts::PI;
    let k = PI * 2.0 * self.freq;
    let sample_rate = config.sample_rate.0 as f64 * config.channels as f64;
    for (idx, samples) in (start_idx..start_idx + (buff.len() / config.channels as usize)).zip(buff.chunks_exact_mut(2)) {
      let v = ((idx as f64 * k / sample_rate).sin() * 0.8) as f32;
      for sample in samples {
        *sample = v;
      }
    }
  }

  fn is_done(&self) -> bool {
    false
  }
}
