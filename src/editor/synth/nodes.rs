use std::cell::RefCell;
use std::rc::Rc;
use eframe::egui;
use eframe::egui::{Color32, LayerId, Layout, PointerButton, Rect, Response, RichText, Rounding, Sense, Stroke, Ui, Vec2, Widget};
use eframe::epaint::RectShape;

mod sin_node;
pub use sin_node::SinNode;
use super::stage::*;

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
  id: usize,
  stage: Rc<RefCell<Stage>>,
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

  pub fn render(&mut self, id: usize, stage: Rc<RefCell<Stage>>, ui: &mut Ui, pan: Vec2) -> Option<Response> {
    let mut ui = ui.child_ui(ui.available_rect_before_wrap(), Layout::default());
    let size = Vec2::new(150.0, 100.0);
    let bg_idx = ui.painter().add(RectShape::filled(Rect::ZERO, Rounding::default(), Color32::BLACK));
    let rect = Rect::from_min_size(ui.max_rect().min, size).translate(self.position + pan);
    let resp = ui.allocate_ui_at_rect(rect, |ui| {
      let title_rect = ui.vertical_centered_justified(|ui| {
        let rect = Rect::from_min_size(ui.cursor().min, Vec2::new(size.x, 22.0));
        ui.painter().rect_filled(rect, Rounding::ZERO, Color32::DARK_GRAY);
        if !self.hidden { // [X] Button
          let box_seg = Rect::from_two_pos(rect.right_top(), rect.right_top()+Vec2::new(-22.0, 22.0)).shrink(1.0);
          let line_seg = box_seg.shrink(2.0);
          let line_stroke = Stroke::new(2.0, Color32::BLACK);
          ui.painter().rect_filled(box_seg, 2.0, Color32::WHITE);
          ui.painter().line_segment([line_seg.left_top(), line_seg.right_bottom()], line_stroke);
          ui.painter().line_segment([line_seg.left_bottom(), line_seg.right_top()], line_stroke);
          let resp = ui.interact(box_seg, ui.id().with(id).with("click_x"), Sense::click());
          if resp.clicked_by(PointerButton::Primary) {
            return None;
          }
        }
        let text = RichText::from(self.node_impl.title()).strong().size(18.0);
        ui.label(text);
        // Content
        let cursor = if self.hidden {
          rect.right_bottom()
        } else {
          let ui = ui.child_ui(ui.available_rect_before_wrap().shrink(5.0), Layout::default());
          let mut ctx = NodeContext::new(id, stage, ui);
          self.node_impl.ui(&mut ctx);
          ctx.ui.cursor().right_top() + Vec2::splat(5.0)
        };
        ui.painter().rect_stroke(Rect::from_two_pos(rect.min, cursor).expand(2.0), Rounding::default(), Stroke::new(2.0, Color32::WHITE));
        ui.painter().set(bg_idx, RectShape::filled(Rect::from_two_pos(rect.min, cursor).expand(2.0), Rounding::default(), Color32::BLACK));
        if !self.hidden {
          Some(Rect::from_two_pos(rect.min, rect.max - Vec2::new(22.0, 0.0)))
        } else {
          Some(rect)
        }
      }).inner;
      if let Some(title_rect) = title_rect {
        Some(ui.interact(title_rect, ui.id().with(id).with("drag_or_click_title"), Sense::click_and_drag()))
      } else {
        None
      }
    }).inner;
    let Some(resp) = resp else {
      return None;
    };
    if resp.dragged() {
      self.position += resp.drag_delta();
    }
    if resp.clicked_by(PointerButton::Primary) {
      self.hidden = !self.hidden;
    }
    Some(resp)
  }
}

impl NodeContext {
  pub fn new(id: usize, stage: Rc<RefCell<Stage>>, ui: Ui) -> Self {
    Self {
      id,
      stage,
      ui,
    }
  }
  pub fn ui(&mut self) -> &mut Ui {
    &mut self.ui
  }

  pub fn constant(&mut self, title: &str, value: &mut f64, unit: &str) {
    self.ui.horizontal(|ui| {
      ui.label(RichText::from(title).size(16.0));
      ui.add(egui::DragValue::new(value)
        .clamp_range(0.0..=22.0*1000.0)
        .prefix("  ").suffix(format!("   [{}]  ", unit))
        .speed(0.1));
    });
  }

  pub fn output(&mut self, title: &str) {
    let ui = &mut self.ui;
    let stage = self.stage.borrow_mut();
    ui.label(RichText::from(title).size(16.0));
    ui.painter().circle_stroke(ui.cursor().right_top() + Vec2::new(-5.0, -8.0), 8.0, Stroke::new(2.0,Color32::DARK_GRAY));
  }
}
