use std::fmt::format;
use eframe::egui;
use eframe::egui::{Color32, Layout, PointerButton, Rect, Response, RichText, Rounding, Sense, Stroke, Ui, Vec2, Widget};
use eframe::egui::style::Widgets;

mod sin_node;
pub use sin_node::SinNode;

pub struct Node {
  position: Vec2,
  node_impl: Box<dyn NodeImpl>,
  hidden: bool,
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
      hidden: false,
    }
  }

  pub fn render(&mut self, ui: &mut Ui, pan: Vec2) -> Response {
    ui.set_clip_rect(ui.available_rect_before_wrap()); // Clip tab bar.
    let size = Vec2::new(150.0, 100.0);
    let rect = Rect::from_min_size(ui.max_rect().min, size).translate(self.position + pan);
    let resp = ui.allocate_ui_at_rect(rect, |ui| {
      let title_rect = ui.vertical_centered_justified(|ui| {
        let rect = Rect::from_min_size(ui.cursor().min, Vec2::new(size.x, 22.0));
        ui.painter().rect_filled(rect, Rounding::none(), Color32::DARK_GRAY);
        let text = RichText::from(self.node_impl.title()).strong().size(18.0);
        ui.label(text);
        // Content
        let cursor = if self.hidden {
          rect.right_bottom()
        } else {
          let mut ui = ui.child_ui(ui.available_rect_before_wrap().shrink(5.0), Layout::default());
          let mut ctx = NodeContext {
            ui,
          };
          self.node_impl.ui(&mut ctx);
          ctx.ui.cursor().right_top() + Vec2::splat(5.0)
        };
        ui.painter().rect_stroke(Rect::from_two_pos(rect.min, cursor).expand(2.0), Rounding::none(), Stroke::new(2.0, Color32::WHITE));
        rect
      }).inner;
      ui.interact(title_rect, ui.id().with("drag"), Sense::click_and_drag())
    }).inner;
    if resp.dragged() {
      self.position += resp.drag_delta();
    }
    if resp.clicked_by(PointerButton::Primary) {
      self.hidden = !self.hidden;
    }
    resp
  }
}

impl NodeContext {
  pub fn ui(&mut self) -> &mut Ui {
    &mut self.ui
  }

  pub fn constant(&mut self, title: &str, value: &mut f64) {
    self.ui.horizontal(|ui| {
      ui.label(RichText::from(title).size(16.0));
      ui.add(egui::DragValue::new(value)
        .clamp_range(0.0..=22.0*1000.0)
        .prefix("  ").suffix("   [Hz]  ")
        .speed(0.1));
    });
  }

  pub fn output(&mut self, title: &str) {
    let ui = &mut self.ui;
    ui.label(RichText::from(title).size(16.0));
    ui.painter().circle_stroke(ui.cursor().right_top() + Vec2::new(-5.0, -8.0), 8.0, Stroke::new(2.0,Color32::DARK_GRAY));
  }
}
