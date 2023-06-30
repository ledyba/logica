use eframe::egui::{Color32, Id, LayerId, Order, Pos2, Rect, Rounding, Sense, Stroke, Ui, Vec2};

pub enum ValueType {
  Scalar,
  Vec2,
}

pub enum SlotType {
  Input,
  Output,
  Constant
}

pub struct Slot {
  value_type: ValueType,
  slot_type: SlotType,
}

pub struct Node {
  slots: Vec<Slot>,
  position: Vec2,
}

impl Node {
  pub fn new(position: Vec2) -> Self {
    Self {
      slots: Vec::new(),
      position,
    }
  }

  pub fn render(&mut self, ui: &mut Ui) {
    ui.set_clip_rect(ui.available_rect_before_wrap()); // Clip tab bar.
    let rect = Rect::from_min_size(ui.max_rect().min, Vec2::INFINITY).translate(self.position);
    let rect = ui.allocate_ui_at_rect(rect, |ui| {
      ui.label("hey");
      if ui.button("button").clicked() {
        println!("Click");
      }
    }).response.rect;
    let painter = ui.painter();
    painter.rect_stroke(rect.expand(10.0), Rounding::same(5.0), Stroke::new(2.0, Color32::from_rgb(255, 0, 0)));
    let mut resp = ui.allocate_rect(rect.expand(10.0), Sense::click_and_drag());
    if resp.dragged() {
      self.position += resp.drag_delta();
    }
  }
}

pub struct Editor {
  nodes: Vec<Node>,
}

impl Editor {
  pub fn new() -> Self {
    Self {
      nodes: Vec::new(),
    }
  }
  pub fn add_node(&mut self, node: Node) {
    self.nodes.push(node);
  }
  pub fn render(&mut self, ui: &mut Ui) {
    for node in &mut self.nodes {
      node.render(ui);
    }
  }
}
