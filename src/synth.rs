pub struct Synth {
  pub freq: f64,
}

impl Synth {
  pub fn new(
    freq: f64,
  ) -> Self {
    Self {
      freq,
    }
  }
}

impl Default for Synth {
  fn default() -> Self {
    Self::new(
      440.0,
    )
  }
}
