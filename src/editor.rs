mod tab;
mod tab_viewer;

use std::rc::Rc;
use eframe::egui;
use egui_dock::{
  Tree
};

use tab::Tab;
use tab_viewer::TabViewer;
use crate::player::Player;

pub struct Editor {
  player: Rc<Player>,
  tree: Tree<Tab>,
}

impl Editor {
  pub fn new(player: Rc<Player>) -> Self {
    let tree = Tree::new(Vec::new());
    Self {
      player,
      tree,
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
            self.tree.push_to_focused_leaf(Tab::new_synth_tab(self.player.clone()));
            ui.close_menu();
          }
        });
      });
    });
    egui::panel::CentralPanel::default().show(ctx, |ui| {
      let layer_id = egui::LayerId::background();
      let max_rect = ctx.available_rect();
      let clip_rect = ctx.available_rect();
      let id = egui::Id::new("egui_dock::DockArea");
      let mut ui = egui::Ui::new(ctx.clone(), layer_id, id, max_rect, clip_rect);
      egui_dock::DockArea::new(&mut self.tree)
        .show_inside(&mut ui, &mut TabViewer::new());
    });
  }
}
