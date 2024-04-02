use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use eframe::egui;
use eframe::egui::{Id, LayerId, Order, PointerButton, Sense, Ui, Vec2};
use eframe::emath::TSTransform;
use super::stage::*;
use super::nodes::*;

pub struct Editor {
  nodes: HashMap<usize, Node>,
  node_idx: usize,
  stage: Rc<RefCell<Stage>>,
  pan: Vec2,
  show_new_node_window: bool,
}

impl Editor {
  pub fn new() -> Self {
    Self {
      nodes: HashMap::new(),
      node_idx: 0,
      stage: Rc::new(RefCell::new(Stage::new())),
      pan: Vec2::splat(0.0),
      show_new_node_window: false,
    }
  }
  pub fn ui(&mut self, ui: &mut Ui) {
    self.stage.borrow_mut().start_frame();
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
        let pos = -self.pan + Vec2::splat(15.0);
        if ui.button("Sin").clicked() {
          self.add_node(Node::new(pos, SinNode::new(440.0)));
          self.show_new_node_window = false;
        }
      });
    }
    ui.with_layer_id(LayerId::new(Order::Background, ui.id().with("node_layer")), |ui| {
      ui.ctx().set_transform_layer(ui.layer_id(), TSTransform::new(self.pan, 1.0));
      self.nodes.retain(|id, node| {
        node.render(*id, self.stage.clone(), ui).is_some()
      });
      self.stage.borrow_mut().ui(ui);
    });
  }

  pub fn add_node(&mut self, node: Node) {
    let id = self.node_idx;
    self.node_idx += 1;
    self.nodes.insert(id, node);
  }
}
