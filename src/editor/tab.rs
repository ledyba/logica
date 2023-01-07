mod synth;

use eframe::egui;
use synth::SynthTab;

pub enum Tab {
  Synth(SynthTab)
}

impl Tab {
  pub fn new_synth_tab() -> Self {
    Self::Synth(SynthTab::default())
  }
}
