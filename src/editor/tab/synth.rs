use eframe::egui;
use crate::synth::Synth;

pub struct SynthTab {
  freq: String,
  synth: Synth,
}

impl SynthTab {
  pub fn new(synth: Synth) -> Self {
    Self {
      freq: synth.freq.to_string(),
      synth,
    }
  }

  pub fn ui(&mut self, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
      if ui.button("▶ Play").clicked() {
      }
      if ui.button("■ Stop").clicked() {
      }
    });
    ui.separator();
    ui.horizontal(|ui| {
      ui.label("Freq");
      if ui.text_edit_singleline(&mut self.freq).changed() {
        if let Ok(freq) = self.freq.parse::<f64>() {
          self.synth.freq = freq;
        } else {
          self.freq = self.synth.freq.to_string();
        }
      }
    });
  }

  pub fn title(&mut self) -> egui::WidgetText {
    egui::WidgetText::from("Synth")
  }
}

impl Default for SynthTab {
  fn default() -> Self {
    Self::new(Synth::default())
  }
}
