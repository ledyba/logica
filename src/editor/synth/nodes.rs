use eframe::egui::{Color32, Pos2, Rect, Rounding, Sense, Stroke, Ui, Vec2};

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
    let rect = Rect::from_min_size(ui.max_rect().min, Vec2::new(200.0, 200.0)).translate(self.position);
    let resp = ui.allocate_ui_at_rect(rect, |ui| {
      let rect = ui.available_rect_before_wrap();
      ui.painter().rect_stroke(rect, Rounding::same(10.0), Stroke::new(2.0, Color32::from_rgb(255, 0, 0)));
      ui.interact(ui.clip_rect(), ui.id(), Sense::click_and_drag())
    }).inner;

    if resp.dragged() {
      self.position += resp.drag_delta();
    }
    if resp.clicked() {
      println!("Clicked");
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
