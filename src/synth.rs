use egui_snarl::Snarl;

pub struct Synth {
}

impl Synth {
  pub fn new(snarl: &Snarl<crate::editor::Node>) -> Self {
    Self {
    }
  }

  pub fn play(&mut self, config: &cpal::StreamConfig, buff: &mut [f32], start_idx: usize) {
    use std::f64::consts::PI;
    let num_channel = config.channels as usize;
    let k = PI * 2.0 * 444.0; //FIXME: Fill.
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
}
