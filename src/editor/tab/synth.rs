use crate::synth::Synth;

pub struct SynthTab {
  synth: Synth,
}

impl SynthTab {
  pub fn new(synth: Synth) -> Self {
    Self {
      synth,
    }
  }
}