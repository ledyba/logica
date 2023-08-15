use eframe::egui;
use eframe::egui::{Color32, Layout, PointerButton, Rect, RichText, Rounding, Sense, Stroke, Ui, Vec2};

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
  title: String,
  slots: Vec<Slot>,
  position: Vec2,
}

impl Node {
  pub fn new(title: &str, position: Vec2) -> Self {
    Self {
      title: title.to_string(),
      slots: Vec::new(),
      position: position + Vec2::splat(4.0) + Vec2::splat(10.0),
    }
  }

  pub fn show(&mut self, ui: &mut Ui, pan: Vec2) {
    ui.set_clip_rect(ui.available_rect_before_wrap()); // Clip tab bar.
    let size = Vec2::new(150.0, 100.0);
    let rect = Rect::from_min_size(ui.max_rect().min, size).translate(self.position + pan);
    let resp = ui.allocate_rect(rect, Sense::click_and_drag());
    ui.allocate_ui_at_rect(rect, |ui| {
      ui.vertical_centered_justified(|ui| {
        let rect = Rect::from_min_size(ui.cursor().min, Vec2::new(size.x, 20.0));
        ui.painter().rect_filled(rect, Rounding::none(), Color32::DARK_GRAY);
        ui.add_space(2.0);
        let text = RichText::from(&self.title).strong().size(16.0);
        ui.label(text)
      }).inner;
      // Content
      let mut ui = ui.child_ui(ui.available_rect_before_wrap().shrink(5.0), Layout::default());
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
  pub fn add_node(&mut self, node: Node) {
    self.nodes.push(node);
  }
  pub fn render(&mut self, ui: &mut Ui) {
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
      node.show(ui, self.pan);
    }
  }
}
