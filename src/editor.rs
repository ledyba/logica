mod synth;

use std::{path::PathBuf, sync::Arc};
use eframe::egui;
use eframe::egui::Widget;
use egui_node_graph::*;

pub struct Editor {
  synth_tree: egui_dock::Tree<synth::Synth>
}

impl Editor {
  pub fn new() -> Self {
    Self {
      synth_tree: egui_dock::Tree::new(Vec::new()),
    }
  }
}

impl eframe::App for Editor {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::panel::CentralPanel::default().show(ctx, |ui| {
      egui::TopBottomPanel::top("top").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
          egui::widgets::global_dark_light_mode_switch(ui);
          ui.menu_button("File", |ui| {
            if ui.add(egui::widgets::Button::new("Exit")).clicked() {
              frame.close();
            }
          });
          ui.menu_button("Logic", |ui| {
            if ui.button("New Logic").clicked() {
              self.synth_tree.push_to_focused_leaf(synth::Synth::new());
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
        egui_dock::DockArea::new(&mut self.synth_tree)
          .show_inside(&mut ui, &mut synth::SynthTab::new());
      });
    });
  }
}
