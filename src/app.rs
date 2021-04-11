use epi::{Frame, Storage, IconData};
use egui::{Rgba, CtxRef, Vec2, Id, LayerId, Order, Color32, Visuals};
use std::time::Duration;
use std::ops::{Add, Deref};
use rand::SeedableRng;
use crate::egui_nodes::node;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct App {
  rand: rand::prelude::StdRng,
}

impl App {
  fn new() -> Self {
    Self {
      rand: rand::prelude::StdRng::seed_from_u64(0 as u64),
    }
  }
}

impl Default for App {
  fn default() -> Self {
    App::new()
  }
}

impl epi::App for App {
  fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) {
    node::Node::new(0).show(ctx, |node| {
      node.output(|ui| {
        ui.label("Output");
      });
      node.ui.separator();
      node.input(|ui| {
        ui.label("Source of Input");
      });
    });
    let mut age = 0;
    egui::Window::new("a").id(Id::new(2)).show(ctx, |ui| {
      ui.heading("My egui Application");
      ui.add(egui::Slider::from_get_set(0.0..=120.0, |v| 0.0).text("age"));
      if ui.button("Click each year").clicked() {
        age += 1;
      }
      ui.label(format!("Hello '{}', age {}", "name", age));
    });
  }

  fn name(&self) -> &str {
    "logica"
  }

}

pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
  // Widget code can be broken up in four steps:
  //  1. Decide a size for the widget
  //  2. Allocate space for it
  //  3. Handle interactions with the widget (if any)
  //  4. Paint the widget

  // 1. Deciding widget size:
  // You can query the `ui` how much space is available,
  // but in this example we have a fixed size widget based on the height of a standard button:
  let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

  // 2. Allocating space:
  // This is where we get a region of the screen assigned.
  // We also tell the Ui to sense clicks in the allocated region.
  let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

  // 3. Interact: Time to check for clicks!
  if response.clicked() {
    *on = !*on;
    response.mark_changed(); // report back that the value changed
  }

  // 4. Paint!
  // First let's ask for a simple animation from egui.
  // egui keeps track of changes in the boolean associated with the id and
  // returns an animated value in the 0-1 range for how much "on" we are.
  let how_on = ui.ctx().animate_bool(response.id, *on);
  // We will follow the current style by asking
  // "how should something that is being interacted with be painted?".
  // This will, for instance, give us different colors when the widget is hovered or clicked.
  let visuals = ui.style().interact_selectable(&response, *on);
  // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
  let rect = rect.expand(visuals.expansion);
  let radius = 0.5 * rect.height();
  ui.painter()
    .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
  // Paint the circle, animating it from left to right with `how_on`:
  let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
  let center = egui::pos2(circle_x, rect.center().y);
  ui.painter()
    .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);


  // All done! Return the interaction response so the user can check what happened
  // (hovered, clicked, ...) and maybe show a tooltip:
  response
}