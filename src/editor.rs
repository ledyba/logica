mod node;
mod node_viewer;

use std::rc::Rc;
use eframe::egui::{self, Vec2};
use eframe::egui::Layout;
use egui_snarl::ui::{BackgroundPattern, SnarlStyle};
use egui_snarl::Snarl;
use crate::player::Player;
use crate::synth::Synth;

pub use node::Node;

pub struct Editor {
  player: Rc<Player>,
  // Snarl
  node_viewer: node_viewer::NodeViewer,
  snarl: Snarl<node::Node>,
  snarl_style: SnarlStyle,
}

impl Editor {
  pub fn new(player: Rc<Player>) -> Self {
    let mut snarl_style = SnarlStyle::new();
    snarl_style.bg_pattern = Some(BackgroundPattern::grid(Vec2::splat(50.0), 0.0));
    Self {
      player: player.clone(),
      node_viewer: node_viewer::NodeViewer::new(),
      snarl: Snarl::new(),
      snarl_style,
    }
  }
}

impl eframe::App for Editor {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("Logica::MenuBar").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        egui::widgets::global_dark_light_mode_switch(ui);
        ui.menu_button("File", |ui| {
          if ui.add(egui::widgets::Button::new("Exit")).clicked() {
            self.player.pause().expect("Failed to stop");
            // https://github.com/emilk/egui/pull/3564/files#diff-1d11751241c22642de9437e860ff42990d20d9ff1e6015ecf168ec7616e67417
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
          }
        });
      });
      ui.separator();
      egui::menu::bar(ui, |ui| {
        if ui.button("▶ Play").clicked() {
          self.player.start(Synth::new(&self.snarl)).expect("[BUG] Failed to play");
        }
        if ui.button("■ Stop").clicked() {
          self.player.pause().expect("[BUG] Failed to pause");
        }
      });
    });
    egui::panel::CentralPanel::default().show(ctx, |ui| {
      let ctx = ui.ctx();
      let max_rect = ctx.available_rect();
      let frame = egui::Frame::central_panel(ui.style());
      let ui_stack_info = egui::UiStackInfo::new(egui::UiKind::CentralPanel).with_frame(frame);
      let mut ui = ui.child_ui(max_rect, Layout::default(), Some(ui_stack_info));
      
      self.snarl.show(
        &mut self.node_viewer,
        &self.snarl_style,
        egui::Id::new("snarl"),
        &mut ui,
      )
    });
  }
}
