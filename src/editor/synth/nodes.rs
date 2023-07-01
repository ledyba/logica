use eframe::egui::{Align, Color32, Id, LayerId, Layout, Order, Pos2, Rect, Response, RichText, Rounding, Sense, Stroke, Ui, Vec2, Widget, WidgetText};
use egui_dock::egui::{Label, TextStyle};

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
    let size = Vec2::new(150.0, 100.0);
    let rect = Rect::from_min_size(ui.max_rect().min, size).translate(self.position);
    let resp = ui.allocate_rect(rect, Sense::click_and_drag());
    ui.allocate_ui_at_rect(rect, |ui| {
      ui.vertical_centered_justified(|ui| {
        let rect = Rect::from_min_size(ui.cursor().min, Vec2::new(size.x, 20.0));
        ui.painter().rect_filled(rect, Rounding::none(), Color32::DARK_GRAY);
        ui.add_space(2.0);
        let text = RichText::from("Title").strong().size(16.0);
        ui.label(text)
      }).inner;
      ui.add_space(2.0);
      if ui.button("button").clicked() {
        println!("Click");
      }
    });
    ui.painter().rect_stroke(rect.expand(2.0), Rounding::none(), Stroke::new(2.0, Color32::WHITE));
    //let title_rect = Rect::from_min_size(title_rect.min, Vec2::new(max_size.x, title_rect.height()));
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
