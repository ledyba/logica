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
  editor: nodes::Editor,
}

impl SynthEditor {
  pub fn new(id: u64, player: Rc<Player>) -> Self {
    let synth = Synth::default();
    Self {
      id,
      player,
      synth,
      editor: nodes::Editor::new()
    }
  }

  pub fn ui(&mut self, ui: &mut Ui) {
    let ctx = ui.ctx();
    let layer_id = egui::LayerId::new(egui::Order::Background, egui::Id::new(self.id));
    let max_rect = ui.available_rect_before_wrap();
    let clip_rect = ui.available_rect_before_wrap();
    let id = egui::Id::new("egui_dock::DockArea::Synth").with(self.id);
    let mut ui = Ui::new(ctx.clone(), layer_id, id, max_rect, clip_rect);
    self.render_editor(&mut ui);
    let resp = ui.interact(
      clip_rect,
      ui.id(),
      Sense::hover(),
    );
  }

  fn render_editor(&mut self, ui: &mut Ui) {
    self.editor.render(ui);
  }

  pub fn title(&mut self) -> WidgetText {
    WidgetText::from(format!("Synth[{}]", self.id))
  }

  pub fn play(&mut self) {
    self.player.register(0.0, Box::new(crate::player::SynthTrack::new(&self.synth)));
  }
}
