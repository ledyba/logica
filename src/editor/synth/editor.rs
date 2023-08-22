use eframe::egui;
use eframe::egui::{PointerButton, Sense, Ui, Vec2};
use super::nodes::*;

pub struct Editor {
  nodes: Vec<Node>,
  pan: Vec2,
  show_new_node_window: bool,
}

impl Editor {
  pub fn new() -> Self {
    Self {
      nodes: Vec::new(),
      pan: Vec2::splat(0.0),
      show_new_node_window: false,
    }
  }
  pub fn ui(&mut self, ui: &mut Ui) {
    let resp = ui.interact(ui.available_rect_before_wrap(), ui.id().with("MainPanel"), Sense::drag());
    if resp.dragged_by(PointerButton::Middle) {
      self.pan += resp.drag_delta();
    }
    let changed = if resp.clicked_by(PointerButton::Secondary) {
      self.show_new_node_window = !self.show_new_node_window;
      true
    } else {
      false
    };

    if self.show_new_node_window {
      let mut window = egui::Window::new("New node");
      if changed {
        window = window.current_pos(resp.interact_pointer_pos().expect("[BUG] No pointer position"));
      }
      window.show(ui.ctx(), |ui| {
      });
    }
    for node in &mut self.nodes {
      node.render(ui, self.pan);
    }
  }

  pub fn add_node(&mut self, node: Node) {
    self.nodes.push(node);
  }
}
