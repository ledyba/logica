use eframe::egui::{Color32, Label, Pos2, Stroke, Ui};
use egui_snarl::{ui::{PinInfo, SnarlViewer}, InPin, NodeId, OutPin, Snarl};

use crate::editor::node::Node;

use super::node::Oscillator;

pub struct NodeViewer;

impl NodeViewer {
  pub fn new() -> Self {
    Self
  }
}

impl SnarlViewer<Node> for NodeViewer {
  fn title(&mut self, node: &Node) -> String {
    match node {
      Node::Oscillator(osc) => match osc {
        super::node::Oscillator::Sin => "Sin Oscillator".to_string(),
      },
    }
  }

  #[inline(always)]
  fn show_header(
      &mut self,
      node: NodeId,
      _inputs: &[InPin],
      _outputs: &[OutPin],
      ui: &mut Ui,
      _scale: f32,
      snarl: &mut Snarl<Node>,
  ) {
      ui.add(Label::new(self.title(&snarl[node])).selectable(false));
  }

  fn outputs(&mut self, node: &Node) -> usize {
    match node {
      Node::Oscillator(osc) => match osc {
        super::node::Oscillator::Sin => 1,
      },
    }
  }

  fn inputs(&mut self, node: &Node) -> usize {
    match node {
      Node::Oscillator(osc) => match osc {
        super::node::Oscillator::Sin => 1,
      },
    }
  }

  fn show_input(
    &mut self,
    pin: &InPin,
    ui: &mut eframe::egui::Ui,
    scale: f32,
    snarl: &mut Snarl<Node>
  ) -> PinInfo {
    match &mut snarl[pin.id.node] {
      Node::Oscillator(ref mut osc) => match osc {
        Oscillator::Sin => {
          ui.add(Label::new("Freq").selectable(false));
          PinInfo::circle().with_stroke(Stroke::new(1.0, Color32::GREEN))
        },
      }
    }
  }

  fn show_output(
    &mut self,
    pin: &OutPin,
    ui: &mut eframe::egui::Ui,
    scale: f32,
    snarl: &mut Snarl<Node>,
  ) -> PinInfo {
    match &mut snarl[pin.id.node] {
      Node::Oscillator(ref mut osc) => match osc {
        Oscillator::Sin => {
          ui.add(Label::new("Output").selectable(false));
          PinInfo::circle().with_stroke(Stroke::new(1.0, Color32::BLUE))
        },
      }
    }
  }

  fn has_body(&mut self, node: &Node) -> bool {
    match node {
      Node::Oscillator(osc) => match osc {
        super::node::Oscillator::Sin => false,
      },
    }
  }

  fn show_body(
    &mut self,
    node: egui_snarl::NodeId,
    inputs: &[egui_snarl::InPin],
    outputs: &[egui_snarl::OutPin],
    ui: &mut Ui,
    scale: f32,
    snarl: &mut Snarl<Node>,
  ) {
    match &mut snarl[node] {
      Node::Oscillator(ref mut osc) => match osc {
          Oscillator::Sin => unreachable!("No body"),
      },
    }
  }

  #[inline(always)]
  fn has_graph_menu(
    &mut self,
    _pos: Pos2,
    _snarl: &mut Snarl<Node>,
  ) -> bool {
    true
  }

  fn show_graph_menu(
    &mut self,
    pos: Pos2,
    ui: &mut Ui,
    _scale: f32,
    snarl: &mut Snarl<Node>,
  ) {
    ui.label("Add New Node");
    ui.menu_button("Oscillator", |ui| {
      if ui.button("Sin Wave").clicked() {
        snarl.insert_node(pos, Node::Oscillator(Oscillator::Sin));
        ui.close_menu();
      }
    });
  }
}
