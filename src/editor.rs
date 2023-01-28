mod synth;

use std::rc::Rc;
use eframe::egui;

use crate::editor::synth::SynthEditor;
use crate::player::Player;

pub struct Editor {
  player: Rc<Player>,
  synth_editor: SynthEditor,
}

impl Editor {
  pub fn new(player: Rc<Player>) -> Self {
    Self {
      player: player.clone(),
      synth_editor: SynthEditor::new(player.clone()),
    }
  }
}

impl eframe::App for Editor {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("Logica::MenuBar").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        egui::widgets::global_dark_light_mode_switch(ui);
        ui.menu_button("File", |ui| {
          if ui.add(egui::widgets::Button::new("Exit")).clicked() {
            self.player.pause().expect("Failed to stop");
            frame.close();
          }
        });
      });
      ui.separator();
      egui::menu::bar(ui, |ui| {
        if ui.button("▶ Play").clicked() {
          self.player.start().expect("[BUG] Failed to play");
          self.synth_editor.play();
        }
        if ui.button("■ Stop").clicked() {
          self.player.pause().expect("[BUG] Failed to pause");
        }
      });
    });
    egui::panel::CentralPanel::default().show(ctx, |ui| {
      self.synth_editor.show(ui);
    });
  }
}
