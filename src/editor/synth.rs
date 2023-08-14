mod nodes;

use std::rc::Rc;
use eframe::egui::{Layout, Ui, Vec2, WidgetText};
use crate::editor::synth::nodes::Node;
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
    let mut editor = nodes::Editor::new();
    editor.add_node(Node::new(Vec2::new(0.0, 0.0)));
    Self {
      id,
      player,
      synth,
      editor,
    }
  }

  pub fn ui(&mut self, ui: &mut Ui) {
    let max_rect = ui.available_rect_before_wrap();
    let mut ui = ui.child_ui(max_rect, Layout::default());
    self.render_editor(&mut ui);
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
