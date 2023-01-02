mod tab;

use eframe::egui;
use egui_dock::{
  Tree
};

use tab::Tab;

pub struct Editor {
  tree: Tree<Tab>
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
              self.tree.push_to_focused_leaf(Tab::new_synth_tab());
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
          .show_inside(&mut ui, &mut tab::TabViewer::new());
      });
    });
  }
}
