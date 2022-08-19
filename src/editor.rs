use std::{path::PathBuf, sync::Arc};
use eframe::egui;
use eframe::egui::Widget;
use egui_node_graph::*;
use log::info;

pub struct Editor {
}

impl Editor {
  pub fn new() -> Self {
    Self {
    }
  }
}

impl eframe::App for Editor {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::panel::CentralPanel::default().show(ctx, |ui| {
      egui::TopBottomPanel::top("top").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
          egui::widgets::global_dark_light_mode_switch(ui);
          if ui.add(egui::widgets::Button::new("Menu")).clicked() {
            info!("Clicked");
          }
        });
      });
      let graph_response = egui::CentralPanel::default()
        .show(ctx, |ui| {
          ui.heading("Logica");
        })
        .inner;
    });
  }
}
