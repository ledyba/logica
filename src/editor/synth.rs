mod nodes;

use std::fmt::format;
use std::rc::Rc;
use eframe::egui;
use eframe::egui::{Color32, Pos2, Rect, Rounding, Sense, Ui, Vec2, WidgetText};
use eframe::epaint::Stroke;
use crate::player::Player;
use crate::synth::Synth;

pub struct SynthEditor {
  id: u64,
  player: Rc<Player>,
  synth: Synth,

}

impl SynthEditor {
  pub fn new(id: u64, player: Rc<Player>) -> Self {
    let synth = Synth::default();
    Self {
      id,
      player,
      synth,
    }
  }

  pub fn ui(&mut self, ui: &mut Ui) {
    let ctx = ui.ctx();
    let layer_id = egui::LayerId::new(egui::Order::Background, egui::Id::new(self.id));
    let max_rect = ui.available_rect_before_wrap();
    let clip_rect = ui.available_rect_before_wrap();
    let id = egui::Id::new("egui_dock::DockArea::Synth").with(self.id);
    let mut ui = Ui::new(ctx.clone(), layer_id, id, max_rect, clip_rect);
    let resp = ui.interact(
      clip_rect,
      ui.id(),
      Sense::hover(),
    );
    // let scroll = ui.ctx().input(|state| state.scroll_delta)
    ui.allocate_ui_at_rect(Rect::from_min_size(max_rect.min, Vec2::new(200.0, 200.0)), |ui| {
      let rect = ui.available_rect_before_wrap();
      ui.painter().rect_stroke(rect.shrink(10.0), Rounding::same(10.0), Stroke::new(2.0, Color32::from_rgb(255, 0, 0)));
    });

  }

  pub fn title(&mut self) -> WidgetText {
    WidgetText::from(format!("Synth: {}", self.id))
  }

  pub fn play(&mut self) {
    self.player.register(0.0, Box::new(crate::player::SynthTrack::new(&self.synth)));
  }
}
