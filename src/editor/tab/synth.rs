use eframe::egui;
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

  pub fn ui(&mut self, ui: &mut egui::Ui) {
    ui.label("SynthTab");
  }

  pub fn title(&mut self) -> egui::WidgetText {
    egui::WidgetText::from("Synth")
  }
}