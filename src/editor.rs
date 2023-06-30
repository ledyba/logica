mod synth;

use std::rc::Rc;
use eframe::egui;
use eframe::egui::Layout;
use crate::editor::synth::SynthEditor;
use crate::player::Player;

pub struct Editor {
  unused_id: u64,
  player: Rc<Player>,
  tree: egui_dock::Tree<Tab>,
}

impl Editor {
  pub fn new(player: Rc<Player>) -> Self {
    Self {
      unused_id: 0,
      player: player.clone(),
      tree: egui_dock::Tree::new(Vec::new()),
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
        ui.menu_button("Logic", |ui| {
          if ui.button("New Synth Logic").clicked() {
            self.tree.push_to_focused_leaf(Tab::new_synth_tab(self.unused_id, self.player.clone()));
            self.unused_id += 1;
            ui.close_menu();
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
      let mut dock = egui_dock::DockArea::new(&mut self.tree);
      dock.show_inside(&mut ui, &mut TabViewer::new());
    });
  }
}

pub struct TabViewer {}

impl TabViewer {
  pub fn new() -> Self {
    Self {
    }
  }
}

pub enum Tab {
  Synth(SynthEditor)
}

impl Tab {
  pub fn new_synth_tab(id: u64, player: Rc<Player>) -> Self {
    Self::Synth(SynthEditor::new(id, player))
  }
}

impl egui_dock::TabViewer for TabViewer {
  type Tab = Tab;

  fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
    match tab {
      Tab::Synth(tab) => tab.ui(ui),
    }
  }

  fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
    match tab {
      Tab::Synth(tab) => tab.title(),
    }
  }
}
