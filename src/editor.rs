mod tab;
mod tab_viewer;

use cpal::traits::{DeviceTrait, HostTrait};
use eframe::egui;
use eframe::egui::Vec2;
use egui_dock::{
  Tree
};

use tab::Tab;
use tab_viewer::TabViewer;

pub struct Editor {
  tree: Tree<Tab>,
}

impl Editor {
  pub fn new() -> Self {
    let tree = Tree::new(Vec::new());
    Self {
      tree,
    }
  }
}

impl eframe::App for Editor {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::panel::CentralPanel::default().show(ctx, |ui| {
      egui::TopBottomPanel::top("Logica::MenuBar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
          egui::widgets::global_dark_light_mode_switch(ui);
          ui.menu_button("File", |ui| {
            if ui.add(egui::widgets::Button::new("Exit")).clicked() {
              frame.close();
            }
          });
          ui.menu_button("Logic", |ui| {
            if ui.button("New Synth Logic").clicked() {
              self.tree.push_to_focused_leaf(Tab::new_synth_tab());
              ui.close_menu();
            }
          });
        });
      });

      egui::CentralPanel::default().show(ctx, |_ui| {
        let layer_id = egui::LayerId::background();
        let max_rect = ctx.available_rect();
        let clip_rect = ctx.available_rect();
        let id = egui::Id::new("egui_dock::DockArea");
        let mut ui = egui::Ui::new(ctx.clone(), layer_id, id, max_rect, clip_rect);
        egui_dock::DockArea::new(&mut self.tree)
          .show_inside(&mut ui, &mut TabViewer::new());
      });
    });
  }
}
