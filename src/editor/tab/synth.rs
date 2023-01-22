use std::rc::Rc;
use eframe::egui;
use crate::player::Player;
use crate::synth::Synth;

pub struct SynthTab {
  player: Rc<Player>,
  synth: Synth,
  freq: String,
}

impl SynthTab {
  pub fn new(player: Rc<Player>) -> Self {
    let synth = Synth::default();
    let freq = synth.freq.to_string();
    Self {
      player,
      synth,
      freq,
    }
  }

  pub fn ui(&mut self, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
      if ui.button("▶ Play").clicked() {
        self.player.start().expect("[BUG] Failed to play");
        self.player.register(0.0, Box::new(crate::player::SynthTrack::new(&self.synth)))
      }
      if ui.button("■ Stop").clicked() {
        self.player.pause().expect("[BUG] Failed to pause");
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
