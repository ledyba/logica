mod synth;

use std::rc::Rc;
use eframe::egui;
use eframe::egui::Layout;
use crate::editor::synth::SynthEditor;
use crate::player::Player;

pub struct Editor {
  unused_id: u64,
  player: Rc<Player>,
  editor: SynthEditor,
}

impl Editor {
  pub fn new(player: Rc<Player>) -> Self {
    Self {
      unused_id: 0,
      player: player.clone(),
      editor: SynthEditor::new(128u64, player.clone()),
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
        }
        if ui.button("■ Stop").clicked() {
          self.player.pause().expect("[BUG] Failed to pause");
        }
      });
    });
    egui::panel::CentralPanel::default().show(ctx, |ui| {
      let ctx = ui.ctx();
      let max_rect = ctx.available_rect();
      let mut ui = ui.child_ui(max_rect, Layout::default());
      self.editor.ui(&mut ui);
    });
  }
}
