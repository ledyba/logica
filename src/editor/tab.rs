mod synth;

pub use synth::SynthTab;

pub enum Tab {
  Synth(SynthTab)
}

impl Tab {
  pub fn new_synth_tab(player: std::rc::Rc<crate::player::Player>) -> Self {
    Self::Synth(SynthTab::new(player))
  }
}
