use egui::{Ui, Frame, Style, CtxRef, Response, Area, Window, Widget};
use std::collections::HashMap;
use std::pin::Pin;

#[derive(Default)]
pub struct NodeHost {
  nodes_counter: u32,
  nodes: HashMap<u32, NodeSlot>,
}

enum NodeSlot {
  Input(Node),
  Output(Node),
}

pub struct NodeUi<'a> {
  pub ui: &'a mut Ui,
}

impl NodeUi<'_> {
  pub fn input(&mut self, add_contents: impl FnOnce(&mut Ui)) -> Option<Response> {
    let resp = self.ui.horizontal(|ui| {
      let desired_size = ui.spacing().interact_size.y * egui::vec2(1.0, 1.0);
      let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
      let radius = 0.5 * rect.height();
      let center = rect.center();
      let visuals = ui.style().interact_selectable(&response, false);
      ui.painter()
        .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
      add_contents(ui);
    }).response;
    Some(resp)
  }

  pub fn output(&mut self, add_contents: impl FnOnce(&mut Ui)) -> Option<Response> {
    let resp = self.ui.horizontal(|ui| {
      add_contents(ui);
      let desired_size = egui::vec2(ui.available_width(), ui.spacing().interact_size.y);
      let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
      let radius = 0.5 * rect.height();
      let center = rect.center();
      let visuals = ui.style().interact_selectable(&response, false);
      ui.painter()
        .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }).response;
    Some(resp)
  }
}

pub struct Node {
  frame: Frame,
  area: Area,
}

impl Node {
  pub fn new(id: u32) -> Self {
    Self {
      frame: Frame::window(&Style::default()),
      area: Area::new(id),
    }
  }

  pub fn show(self, ctx: &CtxRef, add_contents: impl FnOnce(&mut NodeUi)) -> Option<Response> {
    let r = self.area.show(ctx, |ui| {
      self.frame.show(ui, |ui| {
        let mut node = NodeUi {
          ui,
        };
        add_contents(&mut node)
      });
    });
    Some(r)
  }
}

pub struct InputWidget {

}
/*
impl Widget for InputWidget {
  fn ui(self, ui: &mut Ui) -> Response {

  }
}
*/
pub struct OutputWidget {

}
