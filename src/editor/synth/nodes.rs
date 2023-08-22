use eframe::egui::{Color32, Layout, Rect, RichText, Rounding, Sense, Stroke, Ui, Vec2};

pub struct Node {
  position: Vec2,
  node_impl: Box<dyn NodeImpl>,
}

pub trait NodeImpl {
  fn title(&self) -> &'static str;
  fn ui(&mut self, node: &mut NodeContext);
}

pub struct NodeContext {
  ui: Ui,
}

impl Node {
  pub fn new<Impl: NodeImpl + 'static>(position: Vec2, node_impl: Impl) -> Self {
    Self {
      position: position + Vec2::splat(4.0) + Vec2::splat(10.0),
      node_impl: Box::new(node_impl),
    }
  }

  pub fn render(&mut self, ui: &mut Ui, pan: Vec2) {
    ui.set_clip_rect(ui.available_rect_before_wrap()); // Clip tab bar.
    let size = Vec2::new(150.0, 100.0);
    let rect = Rect::from_min_size(ui.max_rect().min, size).translate(self.position + pan);
    let resp = ui.allocate_rect(rect, Sense::click_and_drag());
    ui.allocate_ui_at_rect(rect, |ui| {
      ui.vertical_centered_justified(|ui| {
        let rect = Rect::from_min_size(ui.cursor().min, Vec2::new(size.x, 20.0));
        ui.painter().rect_filled(rect, Rounding::none(), Color32::DARK_GRAY);
        ui.add_space(2.0);
        let text = RichText::from(self.node_impl.title()).strong().size(16.0);
        ui.label(text)
      }).inner;
      ui.painter().rect_stroke(rect.expand(2.0), Rounding::none(), Stroke::new(2.0, Color32::WHITE));
      // Content
      let mut ui = ui.child_ui(ui.available_rect_before_wrap().shrink(5.0), Layout::default());
      let mut ctx = NodeContext {
        ui,
      };
      self.node_impl.ui(&mut ctx);
    });
    //let title_rect = Rect::from_min_size(title_rect.min, Vec2::new(max_size.x, title_rect.height()));
    if resp.dragged() {
      self.position += resp.drag_delta();
    }
  }


}

impl NodeContext {
  pub fn ui(&mut self) -> &mut Ui {
    &mut self.ui
  }

  pub fn constant(&mut self, value: &mut f64) {
    let mut str = value.to_string();
    self.ui.text_edit_singleline(&mut str);
    if let Ok(v) = str.parse::<f64>() {
      *value = v;
    }
  }
}

pub struct SinNode {
  freq: f64,
}

impl SinNode {
  pub fn new(freq: f64) -> Self {
    Self {
      freq,
    }
  }
}

impl NodeImpl for SinNode {
  fn title(&self) -> &'static str {
    "SinNode"
  }

  fn ui(&mut self, node: &mut NodeContext) {
    node.constant(&mut self.freq);
  }

}
