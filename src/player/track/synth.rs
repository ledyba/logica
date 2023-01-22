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
    let num_channel = config.channels as usize;
    use std::f64::consts::PI;
    let k = PI * 2.0 * self.freq;
    let samples_per_second = config.sample_rate.0 as f64 * num_channel as f64;
    let mut idx = start_idx;
    for chunk in buff.chunks_exact_mut(num_channel) {
      let v = ((idx as f64 * k / samples_per_second).sin() * 0.8) as f32;
      for out in chunk {
        *out = v;
      }
      idx += num_channel;
    }
  }

  fn is_done(&self) -> bool {
    false
  }
}
