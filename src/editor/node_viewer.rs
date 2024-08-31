use egui_snarl::{ui::{PinInfo, SnarlViewer}, Snarl};

use crate::editor::node::Node;

pub struct NodeViewer {
}

impl NodeViewer {
  pub fn new() -> Self {
    Self {
    }
  }
}

impl SnarlViewer<Node> for NodeViewer {
  fn title(&mut self, node: &Node) -> String {
    "Synth".to_string()
  }

  fn outputs(&mut self, node: &Node) -> usize {
    0
  }

  fn inputs(&mut self, node: &Node) -> usize {
    0
  }

  fn show_input(
    &mut self,
    pin: &egui_snarl::InPin,
    ui: &mut eframe::egui::Ui,
    scale: f32,
    snarl: &mut Snarl<Node>
  ) -> PinInfo {
    PinInfo::star()
  }

  fn show_output(
    &mut self,
    pin: &egui_snarl::OutPin,
    ui: &mut eframe::egui::Ui,
    scale: f32,
    snarl: &mut Snarl<Node>,
  ) -> PinInfo {
    PinInfo::star()
  }
}
